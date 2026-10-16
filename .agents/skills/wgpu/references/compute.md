# Compute phases and resource scopes

- Establish producer/consumer order before choosing boundaries; initialize accumulation counters and give
  aggregate counts/totals/indirect arguments a defined writer. GPU queue order can connect phases without CPU
  waits.
- Treat each compute dispatch as a usage scope. Dependent dispatches may remain in one compute pass when
  their bindings/usages are legal; global phases need dispatch boundaries, not automatically new passes.
- Treat the whole render pass as a scope, including resources set by commands and later replaced.
  Overwriting a bind group does not remove its earlier usage. End active passes before encoder copies.
- Do not sample/write the same texture subresource incompatibly. Restrict mip/layer/aspect views; separate
  render-target feedback. Sampling attached depth requires the relevant attachment aspects to be read-only.
- Reject overlapping writable buffer bindings in a draw/dispatch. Disjoint ranges remove that overlap but
  do not legalize otherwise incompatible whole-buffer usages within a scope.
- Let safe wgpu manage transitions; do not invent native explicit barriers in ordinary safe host code.
- Use ping-pong for updates that read neighbors from generation N and write N+1. In-place updates require
  proof that no invocation reads another's changing state. Advance selection per encoded step, including
  several steps before one submit; render the final destination, not a once-per-frame swap guess.
- Use workgroupBarrier for workgroup memory and storageBarrier for storage ordering within a workgroup.
  Neither synchronizes all workgroups. Atomics do not establish a global barrier or order unrelated writes.
- Keep required barriers in uniform control flow. For partial groups, load neutral values for inactive
  lanes with actual if control flow, let every participating lane reach the barrier, and guard final writes.
- Do not use select(0.0, input[index], in_bounds) as protection: value arguments are evaluated before
  selection. Do not return inactive lanes before a later required barrier.
- Use checked ceiling division and guard logical count against bound capacity. Validate per-axis/product,
  workgroup storage, and dispatch limits; tile large work with a base index and overflow-safe shader indexing.
- Tune legal workgroup shapes against occupancy, bandwidth, and algorithm structure; 64 lanes or 8x8 is
  a starting hypothesis, not an optimum. Do not suppress uniformity errors to hide an invalid algorithm.
- Define the reduction grouping and numerical tolerance before changing a serial update to workgroup or
  multi-dispatch reductions. Distinct phases preserve dependencies, not necessarily identical float order.

Sources: [usage scopes][scopes], [binding aliasing][alias], [barriers][barriers], [WGSL calls][calls].

[scopes]: https://www.w3.org/TR/webgpu/#programming-model-synchronization
[alias]: https://www.w3.org/TR/webgpu/#abstract-opdef-encoder-bind-groups-alias-a-writable-resource
[barriers]: https://www.w3.org/TR/WGSL/#sync-builtin-functions
[calls]: https://www.w3.org/TR/WGSL/#function-calls
