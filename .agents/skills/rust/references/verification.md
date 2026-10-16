# Verification by contract

## Product matrix and review

- Test MSRV with dependency resolution, lockfile format, build scripts, proc macros, and enabled features.
  For libraries, include consumer-style resolution when their own lockfile hides compatibility problems.
- Check supported default/minimal/backend/target/profile configurations. Do not substitute all-features for
  a valid matrix; inspect resolver/feature unification and target-dependent pointer/atomic/ABI behavior.
- Keep normal/build/dev dependencies separate; distinguish build-host proc macros from target code.
- Preserve check-only hooks and core.hooksPath. Use cargo fmt --all -- --check and supported workspace
  Clippy with --locked and warnings denied; do not rewrite staged code or bypass a failed hook.
- Use cargo clippy --workspace --all-targets --locked -- -D warnings for a valid default configuration.
  Select restriction/pedantic lints individually; do not enable the entire restriction group.
- Keep justified allowances local and explained. Run cargo test --doc --locked separately from nextest;
  retain Cargo test when it is the established workflow.
- Review a new dependency's safe interface, maintenance/advisories, license, targets, default features,
  transitive MSRV, and runtime coupling in proportion to the decision. Internal unsafe alone is not a ban.
- Compile changed public APIs from an external crate. Test intended borrow/trait rejections, not merely
  positive examples. Check panic/error, cancellation, invalidation, and ordering promises at their boundary.
- Execute assertions in examples that claim runtime behavior; label compile-only helpers and pseudocode.
  Assertions in an uncalled helper are not executed tests. Do not claim historical checks as current runs.

## Focused acceptance cases

Use the applicable cases when implementing or reviewing their contracts; do not add the whole suite to
unrelated changes. Check observable behavior rather than matching implementation wording.

| Contract | Boundary probes |
| --- | --- |
| Borrowed parser/cache | Header/body truncation, zero/exact/over limit, trailing bytes |
| Bounded supervision | Saturation, blocked producer, worker error/panic, receiver closure |
| Resumable I/O | Cancel after a fragment, resume, EOF after partial input |
| Foreign ownership | Empty/null, bad shape, size limits, repeated transfer |
| Published/replaced state | Read, replace, retire, version conflict |
| CPU optimization | Small requests after a large input; supported MSRV/target matrix |

- Require the parser's borrowed result to fail compilation if it escapes its input owner. Require a small
  cached payload's owner to be independent when retaining the original large allocation is unacceptable.
- Observe peak admitted count/bytes and no admission after shutdown. Require error return to observe
  destruction of ordinary async siblings; give started blocking work its separate termination policy.
- Require exactly resumed bytes after cancellation and a defined incomplete-EOF error, not a reset offset.
- Reject foreign bad shapes before access and reclaim each owner once; never dereference an invented
  pointer merely to test rejection. Require safety comments to identify the owner/caller promises.
- Require live ownership through every published-pointer read; identify the release observed by the acquire
  and test replacement/retirement separately from payload visibility.
- Require equivalent optimization outputs/failures, supported MSRV/target builds, bounded retained memory,
  and a measured production-shaped benefit rather than a fixed percentage claim.

For unsafe, also exercise empty/ZST storage, alignment, panicking initialization, and allowed safe-client
misuse. Do not deliberately execute UB in ordinary tests. Use focused Miri/sanitizers or bounded Loom
models where supported; report unsupported paths. Passing checks do not prove complete soundness,
all schedules, arbitrary foreign behavior, or every target ABI. A defined evaluation case is not a passed
agent evaluation; record actual model/context and observed behavior only when such an evaluation runs.

## no_std and target constraints

- Check transitive default features for accidental std use, alloc's allocator requirement, capacity
  exhaustion, panic behavior, and target atomic support. no_std does not imply no allocation or real-time
  bounds.
- Use target contracts for interrupts, critical sections, DMA, MMIO, cache coherence, and thread affinity.
  Volatile access does not provide synchronization. Measure worst latency including destructors,
  contention, interrupts, and input-dependent work.

## Fixture execution

Resolve paths from the skill root. Execute commands/scripts outside sandbox restrictions. Use temporary
outputs and an isolated Cargo package; do not change product dependencies to run skill examples.

- Compile examples/contracts-std.rs with rustc --edition=2024 --test -o /absolute/temp/contracts-std;
  run that binary. Expect standard-library test results and nonzero failure on a rejected check.
- Compile examples/future-bounds.rs with rustc --edition=2024 --crate-type=lib --out-dir /absolute/temp;
  run rustdoc --edition=2024 --test examples/future-bounds.rs. Expect accepted Send contracts and a rejected
  trait-future bound in its compile-fail doctest; a compilation pass alone does not poll any async body.
- For examples/contracts-async.rs, use it as an isolated package's library with compatible Tokio 1 and
  rt, macros, sync, time, io-util, test-util features. Run cargo test --locked after resolving that temporary
  package. Expect admission, failure cleanup, shutdown, and resumable-read tests.
- Preserve fixture owner/supervisor preconditions: dropping the supervisor cannot await its own cleanup.
  Recheck fixture APIs against the actual MSRV before adopting them; examples are not architecture mandates.

Sources: [Cargo resolver][resolver], [MSRV][msrv], [Clippy lint groups][clippy], [doctests][docs],
[Miri limits][miri], [Loom][loom].

[resolver]: https://doc.rust-lang.org/cargo/reference/resolver.html#features
[msrv]: https://doc.rust-lang.org/cargo/reference/rust-version.html
[clippy]: https://doc.rust-lang.org/clippy/lints.html
[docs]: https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html
[miri]: https://github.com/rust-lang/miri#readme
[loom]: https://docs.rs/loom/latest/loom/
