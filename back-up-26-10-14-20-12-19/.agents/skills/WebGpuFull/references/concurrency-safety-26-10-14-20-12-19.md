# GPU concurrency and safety contracts

## Order CPU preparation, queue work, and completion separately

- Prepare immutable upload records in CPU tasks; validate logical counts and checked byte sizes before allocation.
- Keep each mutable encoder/staging allocator exclusively owned. Move finished command buffers to a submission owner.
- Use one submission owner or an explicit sequencing protocol for related writes and submissions on a shared queue.
- Do not infer application ordering from `Send`/`Sync`; another producer's submission can flush pending queue writes.
- Attach uploads to the intended batch. Drop `write_buffer_with` views before that batch is submitted.
- Use separate records for separate draws/dispatches; bind groups and draws refer to buffers, not CPU-value snapshots.
- Distinguish a completed CPU task, a submitted batch, and completed GPU work.
- Do not wait on the CPU between GPU-only dependent phases; order commands on the queue instead.
- Do not treat cancellation of a CPU future as cancellation or rollback of already submitted GPU commands.
- Check [queue transfers][queue], [submission completion][done], and [native polling][poll].

## Choose the parameter representation

- Use uniforms for small runtime parameters and indexed storage for dense or GPU-produced tables.
- Compare dynamic uniforms with storage indexing; account for device-offset alignment and padded allocation stride.
- Use vertex/instance streams for regular attributes; compute may write a buffer later consumed as vertex data.
- Do not force vertex pulling merely because a buffer has both `STORAGE` and `VERTEX` usage.
- Use immediate data only when its feature and byte limit are granted.
- Use encoder copies when different values must appear between commands; place copies outside active passes.
- Verify raw layout or shader-aware serialization. Do not require `Pod` when serialization establishes valid bytes.
- Do not manually implement `Pod` to waive padding, field validity, or interior-mutability restrictions.
- Check [WGSL layout][layout] and [Pod obligations][pod]; keep transfer/binding alignment separate.

## Reuse from completion and reject obsolete results

- Use safe handle ownership for ordinary resources retained by submitted work.
- Distinguish dropping a handle from explicit destruction; do not destroy resources needed by future work.
- Reclaim explicitly mapped staging slots only after the associated GPU use completes and mapping becomes legal.
- Do not overwrite overlapping records needed by commands in the same pending batch.
- Use queue order for ordinary unmapped updates; do not add a whole-device wait or fixed frame delay per update.
- Bound readback count and bytes. Tag requests with device generation, resource generation, and request identity.
- Reject obsolete callbacks after resize/replacement/device loss; retain owners until their callback obligations end.
- Copy results out of mapped views, drop views, then unmap before ordinary GPU reuse.
- Check [buffer mapping/lifetimes][buffer] and [staging reuse][belt].

## Keep unchecked GPU paths exceptional

- Prefer `Device::create_shader_module` with validation and runtime checks.
- For trusted shader creation, prove every disabled check's contract, including bounds and indexing arithmetic.
- For passthrough shaders, prove backend shader validity and host/binding compatibility; ordinary validation is bypassed.
- Keep raw HAL ownership, aliasing, synchronization, and destruction inside a reviewed abstraction.
- Keep workgroup-memory initialization enabled unless every read is proved to follow initialization on every path.
- Treat experimental features as a separate documented risk decision; their token admits bugs in otherwise safe calls.
- Prefer clear/load operations; prove full initialization before using an unsafe `LoadOp::DontCare` token.
- Do not claim passing host Rust checks proves shader safety or GPU completion ordering.
- Follow the [device unsafe contracts][device], [experimental token][experimental], and [attachment token][dontcare].
- Preserve required safety and clarity; do not enable unchecked paths solely because a tutorial uses them.

## Verify failure and scheduling paths

- Check distinct per-command values, several encoded ping-pong steps, and a partial final workgroup.
- Check delayed completion, bounded upload/readback saturation, and reordered CPU producers.
- Check obsolete readbacks after resource replacement and device loss.
- Validate feature fallbacks and safe defaults; report unrun GPU tests and benchmarks explicitly.

[queue]: https://docs.rs/wgpu/30.0.1/wgpu/struct.Queue.html#method.write_buffer
[done]: https://docs.rs/wgpu/30.0.1/wgpu/struct.Queue.html#method.on_submitted_work_done
[poll]: https://docs.rs/wgpu/30.0.1/wgpu/struct.Device.html#method.poll
[layout]: https://www.w3.org/TR/WGSL/#memory-layouts
[pod]: https://docs.rs/bytemuck/latest/bytemuck/trait.Pod.html#safety
[buffer]: https://docs.rs/wgpu/30.0.1/wgpu/struct.Buffer.html
[belt]: https://docs.rs/wgpu/30.0.1/wgpu/util/struct.StagingBelt.html
[device]: https://docs.rs/wgpu/30.0.1/wgpu/struct.Device.html
[experimental]: https://docs.rs/wgpu/30.0.1/wgpu/struct.ExperimentalFeatures.html
[dontcare]: https://docs.rs/wgpu/30.0.1/wgpu/struct.LoadOpDontCare.html
