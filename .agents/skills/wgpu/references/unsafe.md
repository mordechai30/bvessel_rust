# Exceptional GPU unsafe paths

- Keep safe validation/runtime checks as the default; require a documented reason and complete proof for
  each disabled protection. A tutorial or presumed speedup is not that proof.
- Prefer Device::create_shader_module with validation/runtime checks. For create_shader_module_trusted,
  prove every disabled check's contract, including bounds and indexing arithmetic on every execution path.
- Treat passthrough shader creation as bypassing wgpu tooling/validation. Prove backend shader validity,
  host/binding compatibility, and target restrictions; failures can include driver crashes or undefined
  behavior.
- Keep workgroup-memory initialization enabled unless every read follows a proved write on every path,
  with correct partial groups, barriers, and selected backend semantics.
- Prefer clear/load attachment operations. Before using the unsafe LoadOp::DontCare token, prove all required
  pixels are written before blending or Store exposes them; partial geometry coverage is not full
  initialization.
- Treat ExperimentalFeatures::enabled as a separate documented safety/risk decision: implementation bugs
  can be reached through otherwise safe API calls. A capability check alone is not that safety assessment.
- Contain raw HAL access in a reviewed abstraction that proves backend ownership, aliasing, synchronization,
  and destruction across every safe wrapper operation. Ordinary safe-handle retention is not a raw API proof.
- Do not infer GPU bounds, initialization, or command completion from successful host compilation or CPU
  tests.

Sources: [trusted shader safety][trusted], [passthrough safety][passthrough], [raw HAL access][hal],
[experimental token][exp], [attachment token][load].

[trusted]: https://docs.rs/wgpu/30.0.1/wgpu/struct.Device.html#method.create_shader_module_trusted
[passthrough]: https://docs.rs/wgpu/30.0.1/wgpu/struct.Device.html#method.create_shader_module_passthrough
[hal]: https://docs.rs/wgpu/30.0.1/wgpu/struct.Device.html#method.as_hal
[exp]: https://docs.rs/wgpu/30.0.1/wgpu/struct.ExperimentalFeatures.html
[load]: https://docs.rs/wgpu/30.0.1/wgpu/struct.LoadOpDontCare.html
