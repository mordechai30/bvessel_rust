# Source authority and disputed advice

Resolve language semantics against the relevant Reference section and individual rustdoc contract;
resolve build behavior against Cargo/rustc manuals. Use the Style Guide for presentation. Recheck moving
stable/latest pages against the project's actual compiler/crate before adopting an API.

Apply these established decisions when a source offers conflicting advice:

| Source advice | Required interpretation |
| --- | --- |
| [Unlikely branches][reintech] | Reject unreachable_unchecked as a likelihood hint |
| Forced inline/native CPU/packed/abort flags | Require measured benefit and preserved product contracts |
| [Expression/lint rewrites][idioms] | Retain clearer control flow without material performance harm |
| [FFI error helpers][ffi-errors] | Assign pointer obligations to the correct unsafe caller boundary |
| [Returned strings][ffi-text] | Check signed status/count, termination, bounds, and owner lifetime |
| [Uncalled assertions][doc-init] | Compile-only evidence; execute before claiming runtime verification |

Keep a temporary CString argument alive through its call; require a retained owner for foreign retention.
Do not reject sound necessary FFI or treat an unsafe API's documented obligations as a source defect.
Distinguish incorrect examples from intentionally labeled counterexamples and conditional tradeoffs.

Use inspected repository implementations only for the stated mechanisms:

- Inspect [Codex dispatch][dispatch] for an admission guard deliberately held through awaited dispatch.
- Inspect [Codex tasks][tasks] for concrete future implementations at runtime-selected task boundaries.
- Inspect Bevy's crates/bevy_render/src/pipelined_rendering.rs for persistent execution ownership and
  bounded handoffs; inspect crates/bevy_ecs/src/world/unsafe_world_cell.rs for dynamic access invariants.
  Verify each wrapper's actual safety contract instead of treating a framework name as proof.

Record Codex commit 76ed3b484d62ccceb9fa512ff4dd139f421814c3 and Bevy commit
c8d199b192a1b173746f7a160d259c4db8ab7789 as inspected snapshots. Do not infer a mandatory runtime,
architecture, dependency, or compiler upgrade; do not claim exhaustive source review or unrun checks.
Keep historical source inventories outside normal operation; use focused contracts for current decisions.

[reintech]: https://reintech.io/blog/optimizing-rust-code-for-performance-guide
[idioms]: https://ianbull.com/posts/idiomatic-rust/
[ffi-errors]: https://rust-unofficial.github.io/patterns/idioms/ffi/errors.html
[ffi-text]: https://rust-unofficial.github.io/patterns/idioms/ffi/passing-strings.html
[doc-init]: https://rust-unofficial.github.io/patterns/idioms/rustdoc-init.html
[dispatch]: https://github.com/openai/codex/blob/76ed3b484d62/codex-rs/core/src/tools/parallel.rs
[tasks]: https://github.com/openai/codex/blob/76ed3b484d62/codex-rs/core/src/tasks/mod.rs
