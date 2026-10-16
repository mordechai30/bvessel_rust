# wgpu and WGSL contracts

## CPU-to-GPU data flow

- Preserve the resolved crate version, backend, and granted device features/limits.
- Prepare immutable CPU records; validate logical counts and checked offsets, sizes, and allocation budgets.
- Use verified raw bytes or shader-aware serialization such as encase; initialize every transferred byte.
- Verify WGSL member offsets, matrix columns, struct size, array stride, and address-space restrictions.
- Keep Rust byte validity, vertex decoding, buffer transfer alignment, and device binding alignment separate.
- Do not require every uniform size to be divisible by 16; do not treat `Pod` as proof of WGSL compatibility.
- Do not manually implement `Pod` to bypass padding, field validity, or interior-mutability obligations.
- Allocate with required usages; upload records; bind intended ranges; encode dependencies; submit the batch.
- Reuse bind groups while resources/ranges remain valid; rebuild them when the referenced resource changes.
- Start small updates with queue writes; measure before introducing a staging belt or mapped pool.
- Keep GPU-produced data on the GPU when another shader or indirect command is its next consumer.
- Check [WGSL layouts][layout], [Pod safety][pod], and [encase serialization][encase].

## Parameters are not draw snapshots

- Use uniforms for small runtime parameters and storage indexing for dense or GPU-produced tables.
- Compare dynamic uniforms with indexed storage; include device-offset padding in cost estimates.
- Use vertex/instance streams for regular attributes; permit compute-produced `STORAGE | VERTEX` buffers.
- Use immediate data only with its granted capability; use constants/overrides for appropriate fixed choices.
- Upload distinct records for distinct draws/dispatches; bind groups record resource references, not CPU values.
- Do not overwrite one offset repeatedly before one submit and expect different draws to retain earlier values.
- Use separate ranges or ordered encoder copies when per-command values differ; encode copies outside passes.
- Follow the [queue-write contract][queue]: CPU bytes are copied now, transfers precede the next submitted commands.

## Concurrency and completion

- Keep mutable encoders and staging allocators exclusively owned; move finished command buffers to a submitter.
- Establish one submission owner or an explicit ordering protocol for related shared-queue writes/submissions.
- Do not infer application sequencing from `Send`/`Sync`; another submitter can flush pending writes.
- Distinguish CPU task completion, submission, and GPU completion; CPU cancellation does not undo submitted work.
- Order dependent GPU commands without CPU waits between phases.
- Reclaim mapped slots from associated completion/remapping, not fixed frame counts.
- Do not overwrite records still needed within the same pending batch; bound upload/readback counts and bytes.
- Tag readback with device/resource generations and request identity; reject obsolete results after replacement/loss.
- Drive native callback progress without blocking the UI; preserve callback owners until completion obligations end.
- Drop mapped views before unmapping and ordinary GPU reuse; check [buffer mapping][buffer] and [polling][poll].

## Compute, rendering, and resource validity

- Advance ping-pong state per encoded simulation step; render its final destination.
- Treat compute scopes per dispatch and render scopes per whole pass, including replaced bindings.
- Split global compute phases into dispatches; retain one compute pass when resource scopes permit it.
- Guard inactive-lane loads with `if`, use neutral tile values, and let every required lane reach barriers.
- Do not use `select` to guard an invalid load: value arguments are evaluated before selection.
- Validate logical count against the bound capacity; do not confuse `arrayLength` with live records.
- Preserve transparent composition order; group opaque work when state reduction is useful.
- Cache complete shader/layout/render-state identity and device generation; bound variants.
- Populate every sampled mip level; allocation alone does not generate mips.
- Match required shader inputs to compatible vertex formats; do not ban extra valid attributes or allowed defaults.
- Check [usage scopes][scopes], [WGSL calls][calls], and [vertex formats][vertex].

## Unsafe and lifecycle boundaries

- Use safe handle ownership for ordinary resources retained by submitted work.
- Distinguish dropping, explicit destruction, mapped-slot reuse, and raw HAL lifetime obligations.
- Do not add fixed-frame deferred deletion for assumed safe-handle use-after-free.
- Prefer validated shader creation and initialized workgroup memory.
- For trusted/passthrough shaders, prove disabled runtime checks, shader validity, and host/binding compatibility.
- For raw HAL access, prove ownership, synchronization, aliasing, and destruction across safe wrappers too.
- Treat experimental-feature tokens as separate risks: implementation bugs can affect otherwise safe calls.
- Prove attachment initialization before using unsafe `LoadOp::DontCare`; prefer clear/load defaults.
- Follow [device contracts][device] and [experimental-feature obligations][experimental].

## Reviewed evidence and limits

Use these snapshots as optional examples, not upgrade or architecture instructions:

- [wgpu v30][wgpu], commit `a0bd44d7ffa87b6127fb37050af4b7db741718e2`:
  `wgpu/src/api/queue.rs` and `wgpu/src/util/belt.rs` establish upload timing and staging lifecycle.
  `examples/features/src/uniform_values/mod.rs` uses shader-aware serialization.
  `examples/features/src/boids/mod.rs` combines compute with vertex consumption.
- [learn-wgpu][learn], commit `d90b30088fceae214c53603207464c874a08b6d8`:
  retain useful host/shader examples; do not adopt the tutorial's manual-Pod shortcut without proving its contract.
- [Bevy][bevy], commit `c8d199b192a1b173746f7a160d259c4db8ab7789`:
  `crates/bevy_render/src/render_resource/uniform_buffer.rs` uses encase-backed uniforms.
  `crates/bevy_render/src/pipelined_rendering.rs` separates extraction and persistent execution ownership.

Treat the `wgpu 30.0.1` documentation links below as a separate API baseline from the inspected v30 snapshot.
Recheck the project's exact release before adopting descriptors. Do not claim exhaustive source coverage.
Test distinct draw values, partial workgroups, multiple simulation steps, delayed callbacks, and capability fallbacks.
Report unrun GPU checks and benchmarks; host compilation alone cannot verify GPU behavior.

[layout]: https://www.w3.org/TR/WGSL/#memory-layouts
[pod]: https://docs.rs/bytemuck/latest/bytemuck/trait.Pod.html#safety
[encase]: https://docs.rs/encase/latest/encase/
[queue]: https://docs.rs/wgpu/30.0.1/wgpu/struct.Queue.html#method.write_buffer
[buffer]: https://docs.rs/wgpu/30.0.1/wgpu/struct.Buffer.html
[poll]: https://docs.rs/wgpu/30.0.1/wgpu/struct.Device.html#method.poll
[scopes]: https://www.w3.org/TR/webgpu/#programming-model-synchronization
[calls]: https://www.w3.org/TR/WGSL/#function-calls
[vertex]: https://www.w3.org/TR/webgpu/#vertex-formats
[device]: https://docs.rs/wgpu/30.0.1/wgpu/struct.Device.html
[experimental]: https://docs.rs/wgpu/30.0.1/wgpu/struct.ExperimentalFeatures.html
[wgpu]: https://github.com/gfx-rs/wgpu/tree/v30
[learn]: https://github.com/sotrh/learn-wgpu
[bevy]: https://github.com/bevyengine/bevy
