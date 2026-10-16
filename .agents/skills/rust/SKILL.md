---
name: rust
description: >-
  Plan, implement, review, and optimize Rust APIs, ownership, concurrency, unsafe boundaries,
  and Cargo workflows. Use for Rust code, toolchain changes, and CPU performance work.
---

# Rust

Preserve edition, MSRV, compiler pins, supported targets, runtime/pools, dependency/lockfile policy,
panic policy, and public contracts unless the task explicitly changes them. Verify APIs against that matrix.

Report material missing resources and source contradictions with the exact advice, source, and consequence.
Reuse prior explicit resolutions; obtain resolution before adopting advice that conflicts with user policy.
Discard unrelated fetched material and tool output after extracting task-relevant evidence.

Read the reference that owns the decision; do not load the complete collection.

| Decision | Read |
| --- | --- |
| Retention, IDs, copy-on-write, drop, pinning | [Ownership](references/ownership.md) |
| Public traits, validated state, numeric/text/wire contracts | [API contracts](references/api-contracts.md) |
| Execution choice, admission, guards, shutdown, atomics, polling | [Concurrency](references/concurrency.md) |
| Raw storage, FFI, callbacks, unsafe invariants | [Unsafe and FFI](references/unsafe-ffi.md) |
| Allocation, layout, dispatch, SIMD, build-profile tuning | [CPU performance](references/performance.md) |
| Product checks, acceptance cases, fixture execution | [Verification](references/verification.md) |
| Compiler, edition, dependency, or public-API migration | [Rust migrations](references/migration.md) |
| Bundled inventory/check-only hook scripts | [Script interfaces](references/scripts.md) |
| Task-specific diagnostic tools and pool/map integration | [Tool operation](references/tools.md) |
| Disputed source advice or inspected repository evidence | [Sources](references/sources.md) |

Use [storage/FFI fixtures](examples/contracts-std.rs) for their tested ownership mechanisms and
[async fixtures](examples/contracts-async.rs) for bounded supervision and retained read progress.
Use [future-bound examples](examples/future-bounds.rs) when specifying public Send/lifetime contracts.
Read the verification reference's [execution instructions](references/verification.md#fixture-execution)
before running examples. Do not install fixture dependencies into the user's project.

Run scripts outside sandbox restrictions. Do not read script implementations unless explicitly asked.
