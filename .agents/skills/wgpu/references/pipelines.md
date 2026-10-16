# Pipeline identity, binding compatibility, and recorded state

## Layouts and creation

- Use explicit shared layouts for switching/reuse; use inferred layouts for isolated pipelines, with bind
  groups specific to that pipeline. Choose group indices by reuse/update frequency, not fixed camera/material
  IDs. Put widely shared, infrequently switched data at low group indices when that improves prefix reuse.
- Match WGSL group/binding indices, stage visibility, access, minimum/effective buffer ranges, dynamic policy,
  texture dimension/sample/storage types, and sampler type. Unused writable entries still affect
  hazards/limits.
- For unfilterable float textures, use Float { filterable: false } with NonFiltering samplers; enable
  FLOAT32_FILTERABLE only when supported/requested/needed. Auto layout can choose an incompatible filterable
  type.
- Match attachment view formats/slots, depth/stencil state, multiview, and sample count. Pipeline MSAA count
  matches the multisampled attachment, not its single-sample resolve. Match indexed strip and index format.
- Validate required vertex inputs for compatible format categories; permit valid extra attributes and
  specified component defaults/truncation. Do not require identical shader/attribute vector widths.
- Use a generated fullscreen triangle where no mesh attributes are required. Start real meshes with fixed
  fetch for compact format decoding; choose pulling when arbitrary indexing or a measured reuse benefit
  warrants manual decoding. Compute-produced STORAGE|VERTEX data does not force pulling.
- Schedule expensive creation during controlled loading/rebuilds; JavaScript async pipeline creation is not
  a same-named Rust API. Check resolved-version descriptors before writing required fields.

## Complete cache identity

- Key live pipelines by device identity/generation, shader source/revision, entry points, per-stage overrides,
  layout identity/immediate size, vertex formats/offsets/strides/steps, primitive topology/strip/winding/cull,
  depth clip, attachment formats/blend/write masks, depth/stencil, sample count, multiview, and enabled
  extensions.
- Invalidate on ABI/device changes and bound variants. Exclude dimensions unless they affect specialization
  or another actual pipeline property; size-only resize need not rebuild a compatible pipeline.
- Distinguish live-handle caching from PipelineCache compilation data. Gate that API by feature/backend;
  persist only compatible wgpu-produced bytes with device compatibility key and atomic replacement.
  Do not accept arbitrary bytes through unsafe cache creation.

## Pass state and ordering

- Keep CPU binding caches local to a pass; compare bind group plus dynamic offsets/ranges. Invalidate after
  external helpers or bundle execution. A pipeline change retains only layout-compatible bindings.
- Distinguish fixed pipeline state from viewport/scissor/blend constant/stencil reference and data commands.
- Group opaque work to reduce state changes/overdraw where useful. Preserve transparent composition order
  across pipeline/material groups.

## Bundles and indirect commands

- Match bundle attachment formats, samples, and depth/stencil read-only policy to the pass.
- Treat pipeline, bindings, immediate, vertex/index state as cleared after bundle execution; rebind direct
  commands. Bundles do not inherit them. Viewport/scissor and blend/stencil dynamic values remain pass state.
- Update referenced buffer contents without rerecording. Rerecord changed handles, ranges, fixed arguments,
  immediate values, or command structure. Indirect arguments can change; recorded direct counts cannot.
- Generate exact wgpu::util indirect records with STORAGE|INDIRECT and actual copy usages. Preserve signed
  indexed base_vertex; keep first_instance zero without its feature. Gate multi-draw/count independently.
- Keep related indirect records in one buffer where useful and account for hidden validation cost beyond
  a pass timestamp; do not assume an extension's availability or a uniform benefit across backends.

Sources: [layouts][layouts], [default layout][auto], [Rust pipeline][rust], [bundles][bundles],
[cache][cache].

[layouts]: https://www.w3.org/TR/webgpu/#pipeline-layout
[auto]: https://www.w3.org/TR/webgpu/#default-pipeline-layout
[rust]: https://docs.rs/wgpu/30.0.1/wgpu/struct.RenderPipelineDescriptor.html
[bundles]: https://www.w3.org/TR/webgpu/#bundles
[cache]: https://docs.rs/wgpu/30.0.1/wgpu/struct.PipelineCache.html

Verify bundle transitions against [Rust execute_bundles][execute].

[execute]: https://docs.rs/wgpu/30.0.1/wgpu/struct.RenderPass.html#method.execute_bundles
