# Authority, focused explanations, and inspected implementations

Use published WebGPU for portable host behavior and WGSL for shader semantics. Resolve Rust signatures,
features, and backend restrictions against the locked wgpu release. Publication does not establish Naga,
browser, driver, adapter, or granted-device support; use the actual selected implementation contract.

Treat retained Rust 30.0.1 spellings as examples. Recheck immediates, staging belts, mapped access, error
scopes, surface outcomes, and descriptors when the locked version differs. Do not mix fields from releases
or upgrade merely to fit an example. Distinguish the inspected v30 snapshot from the 30.0.1 API baseline.

Use these focused explanations only when they clarify the current decision; verify browser examples
against the Rust API and the actual specification contract:

| Difficult decision | Focused worked explanation |
| --- | --- |
| Per-object records and update lifetime | [Uniforms][uniforms], [storage tables][storage] |
| Explicit/shared versus inferred layouts | [Bind group layouts][bindings] |
| Offsets, matrix columns, array strides | [Memory layout][memory] |
| Queue writes versus ordered copies | [Copying data][copies] |
| Fixed fetch and instance streams | [Vertex buffers][vertex] |
| Built-ins versus stage interpolation | [Inter-stage values][varyings] |
| Pipeline constants versus runtime data | [Constants][constants], [immediates][immediates] |

Use inspected implementations as evidence for specific mechanisms:

- Inspect [wgpu boids][boids] for compute-produced vertex data and simulation/render dependencies.
- Inspect [wgpu uniform values][uniforms-rs] for encase-backed serialization and fixed uniform records.
- Inspect wgpu/src/api/queue.rs and wgpu/src/util/belt.rs for pending-write timing and staging reuse;
  inspect wgpu/src/api/device.rs and wgpu-types/src/tokens.rs for unchecked/experimental obligations.
- Inspect Bevy's crates/bevy_render/src/render_resource/uniform_buffer.rs for encoded uniforms,
  crates/bevy_render/src/pipelined_rendering.rs for bounded extraction/ownership handoff, and
  crates/bevy_core_pipeline/src/core_3d/mod.rs for transparent ordering. Do not require its ECS/runtime.
- Use learn-wgpu for parameter/layout examples only after proving the complete actual Pod contract. Do not
  treat a tutorial shortcut as a stronger authority than the trait or shader contract.

Record inspected commits as provenance, not a requirement to load or adopt whole repositories:

- wgpu v30: a0bd44d7ffa87b6127fb37050af4b7db741718e2.
- learn-wgpu: d90b30088fceae214c53603207464c874a08b6d8.
- Bevy: c8d199b192a1b173746f7a160d259c4db8ab7789.

Use the [explainer][explainer] for design intent and [official samples][samples] for task-specific worked
mechanisms; neither establishes validity or speed by popularity. Do not infer performance from showcases
or claim exhaustive source/link review. Keep exact primary contract links in each decision's reference.

[uniforms]: https://webgpufundamentals.org/webgpu/lessons/webgpu-uniforms.html
[storage]: https://webgpufundamentals.org/webgpu/lessons/webgpu-storage-buffers.html
[bindings]: https://webgpufundamentals.org/webgpu/lessons/webgpu-bind-group-layouts.html
[memory]: https://webgpufundamentals.org/webgpu/lessons/webgpu-memory-layout.html
[copies]: https://webgpufundamentals.org/webgpu/lessons/webgpu-copying-data.html
[vertex]: https://webgpufundamentals.org/webgpu/lessons/webgpu-vertex-buffers.html
[varyings]: https://webgpufundamentals.org/webgpu/lessons/webgpu-inter-stage-variables.html
[constants]: https://webgpufundamentals.org/webgpu/lessons/webgpu-constants.html
[immediates]: https://webgpufundamentals.org/webgpu/lessons/webgpu-immediates.html
[boids]: https://github.com/gfx-rs/wgpu/blob/a0bd44d7ffa8/examples/features/src/boids/mod.rs
[uniforms-rs]: https://github.com/gfx-rs/wgpu/blob/a0bd44d7ffa8/examples/features/src/uniform_values/mod.rs
[explainer]: https://gpuweb.github.io/gpuweb/explainer/
[samples]: https://webgpu.github.io/webgpu-samples/
