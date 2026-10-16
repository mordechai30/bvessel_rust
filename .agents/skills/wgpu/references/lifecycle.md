# Queue ordering, mapped ownership, and recovery

## Queue sequencing

- Distinguish CPU preparation completion, submitted command order, and completed GPU use. Send/Sync permits
  legal host sharing; it does not establish the application's intended inter-producer ordering.
- Route related uploads/submissions through one submission owner by default. Permit independent submitters
  only with an explicit protocol that associates uploads with batches and orders dependent submissions.
  Another producer's submit can flush pending transfers; a thread-safe Queue does not enforce this protocol.
- Treat write_buffer as an immediate CPU-to-staging copy whose transfer executes before commands in the
  next submit. Draws reference GPU buffers, not CPU-value snapshots: repeated writes to one offset before
  one submit cannot represent distinct object values. Use separate ranges, indexed records, immediate
  command bytes, or ordered encoder copies instead. An empty submit can flush pending transfers.
- Drop write_buffer_with views before the intended submit; their drop schedules the transfer but does not
  submit it. Native per-write staging allocation can justify measured reuse; it is not universally zero-cost.
- Do not treat CPU task cancellation as GPU command cancellation or rollback of submitted effects.

## Mapping, lifetime, and stale results

- Use queue order for ordinary unmapped updates after prior uses, without a whole-device wait per update.
- Safe wgpu handles retain resources needed by submitted work; dropping one does not require fixed-frame
  deferred deletion. Explicit destroy invalidates future use; do not destroy a resource still needed by
  encoded or future work. Distinguish raw backend ownership from ordinary safe-handle retention.
- For explicit mapped pools, reclaim only after associated completion/remapping. Do not wrap into overlapping
  records still needed in the same pending batch; elapsed frames are neither completion proof nor capacity
  control.
- Map after the producing submit or use the resolved version's deferred mapping API. Drop all mapped views
  before unmap; mapped/pending-map buffers cannot be used for ordinary GPU submission.
- Handle Result from Rust 30 get_mapped_range/get_mapped_range_mut. Keep callbacks short; drive native
  progress with submission/Device::poll, while browser progress uses its event loop.
- Avoid blocking UI/browser loops or adding on_submitted_work_done when mapping already supplies the needed
  wait.
- Bound request count, bytes, and retained results. Tag readback with device/resource generations and request
  identity; reject obsolete callbacks after resize/replacement/loss. Retain callback owners through
  obligations.
- Copy out required results, release mapped views, and unmap before normal GPU reuse. Preserve deliberate
  latency rather than serializing every frame to read the latest result.

## Surface and device recovery

- Retain CPU descriptors, shader/asset sources or reload paths sufficient to rebuild device-owned state.
  Rebuild device resources under a new generation; do not present old handles as transferable to a new device.
- Suspend acquisition/configuration at zero size. Release acquired textures before reconfiguration;
  recreate size-dependent resources together and retain format/sample-compatible pipelines.
- For Rust 30 CurrentSurfaceTexture, render/submit and queue.present usable Success or Suboptimal textures;
  schedule repair for Suboptimal. Skip Timeout/Occluded without spinning. Reconfigure Outdated, recreate
  Lost surfaces as appropriate, and report/fix Validation failures. Check whether the device was lost too.
- Do not transplant older Result<SurfaceTexture, SurfaceError> matches or texture.present into Rust 30.
- After device loss, request a replacement device/queue, recheck features/limits/variants, rebuild all
  device-owned resources, and invalidate handles/caches tied to the old device generation.

Sources: [queue-write timing][queue], [mapping ownership][buffer], [Device polling][device],
[surface outcomes][surface].

[queue]: https://docs.rs/wgpu/30.0.1/wgpu/struct.Queue.html#method.write_buffer
[buffer]: https://docs.rs/wgpu/30.0.1/wgpu/struct.Buffer.html#mapping-buffers
[device]: https://docs.rs/wgpu/30.0.1/wgpu/struct.Device.html#method.poll
[surface]: https://docs.rs/wgpu/30.0.1/wgpu/enum.CurrentSurfaceTexture.html
