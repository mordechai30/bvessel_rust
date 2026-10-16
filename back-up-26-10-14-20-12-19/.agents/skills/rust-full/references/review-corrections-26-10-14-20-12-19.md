# Rust review corrections

## Select execution and ownership by contract

- Preserve the project's runtime, framework task pools, thread affinity, and shutdown policy.
- Use async I/O for nonblocking operations and existing bounded pools for substantial CPU work.
- Keep a dedicated service thread when persistent ownership or affinity requires it; do not create a thread per item.
- Use controlled mutation for stateful services; do not require a pure-core architecture for every system.
- Use runtime-selected service/plugin traits when needed; measure dispatch, allocation, code size, and build time.
- Keep static inner loops where they fit; do not prohibit dynamic dispatch merely because a call is frequent.

## Check suspended state instead of cloning by rule

- Borrow across `await` when the borrow lifetime and complete future state satisfy the required bounds.
- Inspect captured parameters as well as values retained at suspension; `async move` does not extend a reference.
- Clone `Arc` only for another owner; do not confuse it with cloning the payload or taking a snapshot.
- Do not clone `Rc` to fix `Send`: it remains non-`Send`. Use a local executor for required local state.
- Distinguish `Send` transfer from `Sync` sharing; `Cell<T>` is `Send` for `T: Send`, but not `Sync`.
- Use the [Send contract][send], [Cell contract][cell], and [Arc contract][arc] for exact bounds.

## Bound and supervise the whole path

- Bound admission before creating tasks or capturing large inputs.
- Include pending producers, queued closures, active jobs, message bytes, and retained results in memory budgets.
- Do not claim a bounded message count prevents excessive memory use without a maximum message size.
- Retain task handles; distinguish application failure, panic, cancellation, and detached execution.
- Stop admission, cancel or drain by policy, then observe termination and finish fallible cleanup.
- Cooperatively stop started blocking work; aborting a started `spawn_blocking` handle generally cannot stop it.
- Check [blocking-task cancellation][blocking] and [task ownership][handle] for the selected runtime version.

## Hold locks for a deliberate operation

- End ordinary synchronous guards before suspension or reentrant callbacks.
- Permit an async mutex/RwLock guard across suspension when it protects a required asynchronous operation.
- Define lock order, reentry, cancellation, fairness, and failure behavior for that operation.
- Compare an owner task with bounded commands when a retained guard serializes unrelated work.
- Use [Tokio mutex contracts][mutex] rather than a universal ban on guards across `await`.

## Review the complete unsafe abstraction

- Prefer suitable safe wrappers; do not reject dependencies solely for internal unsafe implementations.
- Keep justified unsafe narrow; document caller obligations and facts established before each operation.
- Review safe constructors/mutations, callbacks, panic, forgotten guards, and destruction against the same invariant.
- Prove publication and reclamation separately; `SeqCst` does not create a multi-field transaction.
- Use focused Miri, sanitizer, or model checks where supported; passing checks do not prove soundness.
- Follow [undefined-behavior rules][ub] and [Miri limitations][miri]; report unsupported or unrun checks.

## Preserve behavior when measuring

- Preserve panic policy, diagnostic symbols, security, numerical order/tolerance, and supported deployment targets.
- Compare build-profile and parallel-granularity changes on representative optimized workloads.
- Do not adopt fixed parallel thresholds, forced inlining, or tutorial speedups as universal requirements.
- Check [Cargo profiles][profiles]; retain the clearer implementation when the performance difference is immaterial.

## Source provenance and limits

Use these inspected snapshots as examples, not dependency or architecture mandates:

- [Codex][codex], commit `76ed3b484d62ccceb9fa512ff4dd139f421814c3`:
  `codex-rs/core/src/tools/parallel.rs` retains a Tokio read/write admission guard through awaited dispatch.
  `codex-rs/core/src/tasks/mod.rs` combines static future implementations with a runtime-erased task boundary.
- [Bevy][bevy], commit `c8d199b192a1b173746f7a160d259c4db8ab7789`:
  `crates/bevy_render/src/pipelined_rendering.rs` uses a persistent thread and bounded ownership handoffs.
  `crates/bevy_ecs/src/world/unsafe_world_cell.rs` documents dynamically checked access inside a safe framework.

Treat Rust 1.99 as prior review metadata. Preserve actual compiler pins/MSRV; recheck exact APIs when adapting examples.
Do not claim every repository file, linked source, execution target, or schedule was reviewed or tested.

[send]: https://doc.rust-lang.org/std/marker/trait.Send.html
[cell]: https://doc.rust-lang.org/std/cell/struct.Cell.html
[arc]: https://doc.rust-lang.org/std/sync/struct.Arc.html
[blocking]: https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html
[handle]: https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html
[mutex]: https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html
[ub]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[miri]: https://github.com/rust-lang/miri
[profiles]: https://doc.rust-lang.org/cargo/reference/profiles.html
[codex]: https://github.com/openai/codex
[bevy]: https://github.com/bevyengine/bevy
