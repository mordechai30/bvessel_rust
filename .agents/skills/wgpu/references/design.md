# GPU workload and resource design

- Define workload dimensions, live/capacity counts, update frequency, output quality, latency budget,
  required backends, and the actual next consumer. Resolve missing requirements when they alter the design.
- Classify each resource by owner, CPU/GPU producer, readers, descriptor, subresource range, and logical
  lifetime. Separate persistent simulation/history state, rebuildable device resources, and pass-local state.
- Keep small or inherently serial CPU work on the CPU when upload/dispatch/readback overhead exceeds
  useful parallel work. Move sustained data-parallel work only with a clear next consumer and cost model.
- Keep GPU-produced state on the GPU for shader or indirect consumption. Reduce/compact bounded results
  before CPU readback; accept asynchronous result latency rather than adding a round trip each frame.
- Prepare immutable CPU records off the encoding path where useful. Keep mutable encoding/staging state
  exclusively owned; move finished command buffers through the submission owner. Use a bounded
  extraction/handoff stage rather than sharing a live mutable world, and budget pending producer records.
- Compare AoS/SoA, packed attributes, and data-parallel work by actual access; preserve host/shader contracts
  and numeric order. Contiguous data is not permission for arbitrary native memory aliasing.
- Select pass boundaries from producer/consumer dependencies before optimizing count. Compare fusion's
  avoided stores/loads against extra MRT traffic, register pressure, and altered ordering.
- Reuse persistent resources and update contents while descriptors remain valid. Pool temporary targets
  only across nonoverlapping logical lifetimes; rebuild dependent bindings when handles/ranges change.
- Use bundles for stable repeated command structures and indirect work for GPU-produced arguments when
  those address a real encoding/readback cost. Do not assume they reduce shader work or overdraw.
- Use built-in Rust wgpu APIs for native and Wasm targets; adapt browser principles without requiring
  Three.js, JavaScript host objects, or native-only extensions in portable paths.

For device negotiation, inspect adapter features/limits, request only the selected path's requirements
once, then allocate/dispatch using the granted device values. Union optional requirements with mandatory
ones before the request. Distinguish published specification features, implementation support, adapter
capability, requested device capability, and target backend restrictions.

Sources: [WebGPU model][api], [design intent][explainer], [Rust adapter][adapter].

[api]: https://www.w3.org/TR/webgpu/#programming-model
[explainer]: https://gpuweb.github.io/gpuweb/explainer/
[adapter]: https://docs.rs/wgpu/30.0.1/wgpu/struct.Adapter.html#method.request_device
