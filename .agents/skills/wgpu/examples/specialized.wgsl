@id(0) override LANES: u32 = 64u;
@group(0) @binding(0) var<storage, read_write> values: array<f32>;
@group(0) @binding(1) var<uniform> logical_count: u32;

// Host dispatch count must use the same LANES specialization as the pipeline.
// There is no barrier, so inactive invocations may return immediately.
@compute @workgroup_size(LANES)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    if (gid.x >= logical_count || gid.x >= arrayLength(&values)) {
        return;
    }
    values[gid.x] *= 2.0;
}
