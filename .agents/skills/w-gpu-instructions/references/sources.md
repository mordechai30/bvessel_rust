# Sources

## Authority and support

- Verify API validity against [WebGPU][api] and shader rules against [WGSL][wgsl].
- Use the [explainer][explainer] for design intent and the project's exact [wgpu release][rust] for Rust APIs.
- Use the reviewed baseline: WebGPU CRD 2026-09-15, WGSL CRD 2026-09-21, and `wgpu 30.0.1`.
- Recheck version-sensitive features/signatures; do not infer implementation support from publication alone.
- Treat tutorials and samples as optional examples, not required architecture or performance evidence.
- Consult [official samples][samples], [LearnWebGPU][learn], or [Fundamentals][fundamentals] for worked cases.
- Preserve target-specific measurements and verified contracts when adapting an example.

## Repository review supplement: 2026-10-14

Use these inspected snapshots as task-specific evidence, not upgrade or architecture mandates:

- [wgpu v30][wgpu-v30], commit `a0bd44d7ffa87b6127fb37050af4b7db741718e2`:
  `wgpu/src/api/queue.rs` documents transfer timing; `wgpu/src/util/belt.rs` tracks staging reuse.
  `wgpu/src/api/device.rs` and `wgpu-types/src/tokens.rs` specify unchecked/experimental obligations.
  `examples/features/src/uniform_values/mod.rs` uses encase; `examples/features/src/boids/mod.rs`
  demonstrates compute-produced vertex data and a uniform record without universal 16-byte size rounding.
- [learn-wgpu][learn-wgpu], commit `d90b30088fceae214c53603207464c874a08b6d8`:
  retain useful examples; reject the tutorial's manual-Pod shortcut unless all trait obligations are proved.
- [Bevy][bevy-repo], commit `c8d199b192a1b173746f7a160d259c4db8ab7789`:
  `crates/bevy_render/src/render_resource/uniform_buffer.rs` uses shader-aware serialization.
  `crates/bevy_render/src/pipelined_rendering.rs` uses bounded ownership handoffs.
  `crates/bevy_core_pipeline/src/core_3d/mod.rs` preserves transparent distance ordering.

Keep the inspected v30 snapshot separate from the existing `30.0.1` documentation baseline.
Recheck the actual project version; do not claim exhaustive repository/link coverage or unrun GPU checks.

[wgpu-v30]: https://github.com/gfx-rs/wgpu/tree/v30
[learn-wgpu]: https://github.com/sotrh/learn-wgpu
[bevy-repo]: https://github.com/bevyengine/bevy

[api]: https://www.w3.org/TR/webgpu/
[wgsl]: https://www.w3.org/TR/WGSL/
[explainer]: https://gpuweb.github.io/gpuweb/explainer/
[rust]: https://docs.rs/wgpu/30.0.1/wgpu/
[samples]: https://webgpu.github.io/webgpu-samples/
[learn]: https://eliemichel.github.io/LearnWebGPU/
[fundamentals]: https://webgpufundamentals.org/
