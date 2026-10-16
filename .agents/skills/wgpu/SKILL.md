---
name: wgpu
description: >-
  Plan, implement, review, debug, and optimize WebGPU through Rust wgpu and WGSL.
  Use for GPU resources, CPU-to-GPU data flow, pipelines, compute, synchronization, and precision.
---

# WebGPU through Rust wgpu

Preserve the resolved wgpu version and target contract. Distinguish native backends from browser/Wasm;
verify Rust APIs against the locked release and shader rules against supported WGSL/Naga capabilities.
Translate browser examples by intent, not host API spelling. Do not upgrade dependencies to fit an example.

Report missing shaders/assets, unsupported selected paths, unrun GPU checks, and material source conflicts.
Show conflicting advice with its exact source and effect; reuse prior resolutions and resolve policy conflicts
before adopting the advice. Discard unrelated fetched shader, asset, and tool material after relevant use.

Read only the references needed for the decision. For uploads, read both data and lifecycle: correct bytes
alone do not establish queue sequencing or safe reuse.

| Decision | Read |
| --- | --- |
| Workload division, persistent state, ownership handoff, capabilities | [Design](references/design.md) |
| Constants, overrides, uniforms, storage, instances, immediates | [Shader inputs](references/inputs.md) |
| Rust/WGSL ABI, serialization, ranges, transfers, staging | [Layout and transfers](references/data.md) |
| Shared queues, mapping, stale readback, surface/device recovery | [Lifecycle](references/lifecycle.md) |
| Pipeline/layout identity, cached state, bundles, indirect draws | [Pipelines](references/pipelines.md) |
| Usage scopes, phases, aliasing, ping-pong, guarded barriers | [Compute](references/compute.md) |
| Attachments, MSAA, format support, sampling, color, mip generation | [Textures](references/textures.md) |
| Half storage, arithmetic, capability variants, error budgets | [Precision](references/precision.md) |
| Unchecked shaders, initialization, raw HAL, experimental features | [GPU unsafe](references/unsafe.md) |
| Failure checks, timestamps, diagnostics, example execution | [Verification](references/verification.md) |
| Source authority, focused worked explanations, inspected snapshots | [Sources](references/sources.md) |

Use [Rust contract helpers](examples/gpu-contracts.rs) with [object shader](examples/object.wgsl),
[specialized compute](examples/specialized.wgsl), or [immediate selection](examples/immediate-selection.wgsl)
for their respective mechanisms. Use [guarded reduction](examples/guarded-reduction.wgsl) for partial groups
and [mixed precision](examples/mixed-precision.wgsl) for half storage with f32 arithmetic.
Read [example execution](references/verification.md#example-execution) before checking these resources.
Treat their wgpu 30.0.1 spellings as a checked example baseline, not the project's required version.
