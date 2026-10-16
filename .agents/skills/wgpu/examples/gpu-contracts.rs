//! Focused wgpu 30.0.1 mechanisms, not a renderer or a dependency-upgrade prescription.
//! Use with the adjacent object, specialized, and immediate-selection WGSL modules.

/// Holds one logical object record independently of Rust's native field padding.
/// The shader ABI is tint at 0 and four transform columns at 16, for 80 bytes.
pub struct ObjectParams {
    /// Linear RGBA multiplier.
    pub tint: [f32; 4],
    /// Column-major object-to-clip transform.
    pub transform: [[f32; 4]; 4],
}

/// Serializes the fixed shader record without casting native struct storage.
/// Every output byte is initialized; no Pod implementation is required.
pub fn object_bytes(object: &ObjectParams) -> [u8; 80] {
    let mut bytes = [0; 80];
    let values = object.tint.iter().chain(object.transform.iter().flatten());
    for (slot, value) in bytes.chunks_exact_mut(4).zip(values) {
        slot.copy_from_slice(&value.to_le_bytes());
    }
    bytes
}

/// Calculates an allocation stride separately from the shader's record size.
/// Rejects zero alignment and rounding overflow before allocating any resources.
pub fn aligned_stride(record_size: u64, alignment: u64) -> Option<u64> {
    if alignment == 0 {
        return None;
    }
    let remainder = record_size % alignment;
    if remainder == 0 {
        Some(record_size)
    } else {
        record_size.checked_add(alignment - remainder)
    }
}

/// Owns a capacity of dynamic uniform records and one compatible group-zero binding.
/// Capacity is not a live count: bind only uploaded records and sequence uploads with their batch.
pub struct ObjectSlots {
    /// Persistent uniform allocation updated through the submission owner's queue.
    buffer: wgpu::Buffer,
    /// Explicit layout shared with the compatible pipeline.
    layout: wgpu::BindGroupLayout,
    /// Binding of one 80-byte record, adjusted by each command's dynamic offset.
    group: wgpu::BindGroup,
    /// Device-aligned allocation distance between records, not a WGSL array stride.
    stride: u32,
    /// Maximum number of records allocated by the caller's byte budget.
    capacity: usize,
}

impl ObjectSlots {
    /// Allocates fixed-size records under the caller's byte budget and device limits.
    /// Returns an error for invalid capacity, binding size, dynamic offsets, or arithmetic.
    pub fn new(
        device: &wgpu::Device,
        capacity: usize,
        byte_budget: u64,
    ) -> Result<Self, &'static str> {
        if capacity == 0 {
            return Err("capacity must be nonzero");
        }
        let limits = device.limits();
        let record_size = 80;
        if u64::from(limits.max_uniform_buffer_binding_size) < record_size {
            return Err("record exceeds uniform binding limit");
        }
        let stride = aligned_stride(
            record_size,
            u64::from(limits.min_uniform_buffer_offset_alignment),
        )
        .ok_or("invalid record stride")?;
        let count = u64::try_from(capacity).map_err(|_| "capacity exceeds u64")?;
        let total = stride.checked_mul(count).ok_or("allocation overflow")?;
        if total > byte_budget || total > limits.max_buffer_size {
            return Err("allocation exceeds budget or device limit");
        }
        let last_offset = stride.checked_mul(count - 1).ok_or("offset overflow")?;
        if last_offset > u64::from(u32::MAX) {
            return Err("dynamic offset exceeds u32");
        }
        let stride = u32::try_from(stride).map_err(|_| "stride exceeds u32")?;
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("object uniform slots"),
            size: total,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("object slot layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: wgpu::BufferSize::new(record_size),
                },
                count: None,
            }],
        });
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("one object record"),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &buffer,
                    offset: 0,
                    size: wgpu::BufferSize::new(record_size),
                }),
            }],
        });
        Ok(Self {
            buffer,
            layout,
            group,
            stride,
            capacity,
        })
    }

    /// Returns the layout to include at group zero of the example's pipeline.
    pub fn layout(&self) -> &wgpu::BindGroupLayout {
        &self.layout
    }

    /// Uploads distinct records without submitting or borrowing mutable encoding state.
    /// The queue owner must attach these writes to the intended batch; later writes are not draw snapshots.
    pub fn upload(
        &self,
        queue: &wgpu::Queue,
        objects: &[ObjectParams],
    ) -> Result<(), &'static str> {
        if objects.len() > self.capacity {
            return Err("live records exceed capacity");
        }
        for (index, object) in objects.iter().enumerate() {
            let index = u64::try_from(index).map_err(|_| "index exceeds u64")?;
            queue.write_buffer(
                &self.buffer,
                index * u64::from(self.stride),
                &object_bytes(object),
            );
        }
        Ok(())
    }

    /// Binds one previously uploaded record for the following draw.
    /// The caller tracks the uploaded live count and uses the compatible example pipeline.
    pub fn bind(&self, pass: &mut wgpu::RenderPass<'_>, index: usize) -> Result<(), &'static str> {
        if index >= self.capacity {
            return Err("index exceeds capacity");
        }
        let index = u32::try_from(index).map_err(|_| "index exceeds dynamic offset domain")?;
        let offset = index
            .checked_mul(self.stride)
            .ok_or("dynamic offset overflow")?;
        pass.set_bind_group(0, &self.group, &[offset]);
        Ok(())
    }
}

/// Creates one variant of specialized.wgsl after validating its lane count.
/// Cache this pipeline; dispatch counts must use the same specialized lane count.
pub fn specialize(
    device: &wgpu::Device,
    module: &wgpu::ShaderModule,
    lanes: u32,
) -> Result<wgpu::ComputePipeline, &'static str> {
    let limits = device.limits();
    if lanes == 0
        || lanes > limits.max_compute_workgroup_size_x
        || lanes > limits.max_compute_invocations_per_workgroup
    {
        return Err("invalid workgroup size");
    }
    Ok(
        device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("specialized compute"),
            layout: None,
            module,
            entry_point: Some("main"),
            compilation_options: wgpu::PipelineCompilationOptions {
                constants: &[("0", f64::from(lanes))],
                ..Default::default()
            },
            cache: None,
        }),
    )
}

/// Requests one device while preserving mandatory features and limits.
/// Optional f16 changes the granted shader path; unsupported mandatory features still fail the request.
pub async fn precision_device(
    adapter: &wgpu::Adapter,
    required_features: wgpu::Features,
    required_limits: wgpu::Limits,
    prefer_f16: bool,
) -> Result<(wgpu::Device, wgpu::Queue), wgpu::RequestDeviceError> {
    let mut features = required_features;
    if prefer_f16 && adapter.features().contains(wgpu::Features::SHADER_F16) {
        features |= wgpu::Features::SHADER_F16;
    }
    adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("precision device"),
            required_features: features,
            required_limits,
            ..Default::default()
        })
        .await
}

/// Returns a module only when its scoped validation reports no error.
/// Serialize overlapping scope operations; this helper does not catch other error classes or device loss.
pub async fn checked_shader(
    device: &wgpu::Device,
    source: &str,
) -> Result<wgpu::ShaderModule, wgpu::Error> {
    let scope = device.push_error_scope(wgpu::ErrorFilter::Validation);
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("checked WGSL"),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    });
    match scope.pop().await {
        Some(error) => Err(error),
        None => Ok(module),
    }
}

/// Records two indices for immediate-selection.wgsl's persistent data selection.
/// The device must grant IMMEDIATES and the pipeline must allocate at least eight immediate bytes.
pub fn select_draw(pass: &mut wgpu::RenderPass<'_>, object: u32, material: u32) {
    let mut bytes = [0; 8];
    bytes[..4].copy_from_slice(&object.to_le_bytes());
    bytes[4..].copy_from_slice(&material.to_le_bytes());
    pass.set_immediates(0, &bytes);
}

/// Encodes a vec2<f16> input as two little-endian IEEE binary16 values.
/// This changes storage representation; it does not select the shader's arithmetic precision.
pub fn half_pair_bytes(x: f32, y: f32) -> [u8; 4] {
    let x = half::f16::from_f32(x).to_bits().to_le_bytes();
    let y = half::f16::from_f32(y).to_bits().to_le_bytes();
    [x[0], x[1], y[0], y[1]]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_bytes_match_shader_offsets_and_columns() {
        let object = ObjectParams {
            tint: [1.0, 2.0, 3.0, 4.0],
            transform: [[5.0, 6.0, 7.0, 8.0], [9.0; 4], [10.0; 4], [11.0; 4]],
        };
        let bytes = object_bytes(&object);
        for (offset, value) in [(0, 1.0_f32), (12, 4.0), (16, 5.0), (28, 8.0), (64, 11.0)] {
            assert_eq!(&bytes[offset..offset + 4], &value.to_le_bytes());
        }
    }

    #[test]
    fn allocation_stride_is_not_shader_record_size() {
        assert_eq!(aligned_stride(80, 256), Some(256));
        assert_eq!(aligned_stride(80, 16), Some(80));
        assert_eq!(aligned_stride(80, 0), None);
        assert_eq!(aligned_stride(u64::MAX, 256), None);
    }

    #[test]
    fn half_pairs_contain_float_bits_instead_of_integer_casts() {
        assert_eq!(half_pair_bytes(1.0, -2.0), [0x00, 0x3c, 0x00, 0xc0]);
    }
    #[test]
    fn wgsl_variants_validate_and_object_abi_matches_host_bytes() {
        use naga::valid::{Capabilities, ValidationFlags, Validator};

        let shaders = [
            ("object", include_str!("object.wgsl")),
            ("specialized", include_str!("specialized.wgsl")),
            ("guarded reduction", include_str!("guarded-reduction.wgsl")),
            ("mixed precision", include_str!("mixed-precision.wgsl")),
            (
                "immediate selection",
                include_str!("immediate-selection.wgsl"),
            ),
        ];
        for (name, source) in shaders {
            let module = naga::front::wgsl::parse_str(source)
                .unwrap_or_else(|error| panic!("{name}: {}", error.emit_to_string(source)));
            // Offline capability acceptance does not negotiate any device features.
            Validator::new(ValidationFlags::all(), Capabilities::all())
                .validate(&module)
                .unwrap_or_else(|error| panic!("{name}: {error:?}"));
            if name == "object" {
                let (_, object) = module
                    .types
                    .iter()
                    .find(|(_, ty)| ty.name.as_deref() == Some("ObjectParams"))
                    .expect("ObjectParams type");
                let naga::TypeInner::Struct { ref members, span } = object.inner else {
                    panic!("ObjectParams is not a struct");
                };
                assert_eq!(span, 80);
                assert_eq!(members[0].offset, 0);
                assert_eq!(members[1].offset, 16);
            }
        }
    }
}
