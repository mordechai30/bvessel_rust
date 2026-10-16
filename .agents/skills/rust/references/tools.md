# Conditional Rust tool operation

Run only task-relevant tools through the existing project workflow. Preserve package-manager ownership;
use cargo install --locked PACKAGE only for a required Cargo-installed executable, not for libraries.

## Product feedback and fixes

- For cargo audit, run at Cargo.lock's root against a current RustSec database. Inspect affected versions,
  platform relevance, fixes, and scoped exceptions with review conditions; report stale/offline evidence.
  Run cargo audit, then repeat it and affected tests after a compatible dependency fix; keep the established
  CI/security check. A clean advisory database is not a soundness proof.
- For Bacon, select existing check/Clippy/test/nextest jobs matching product features/targets. Add bacon.toml
  only when authorized. A running watcher is not a completed final/CI check.
- For nextest, account for process-per-test isolation, shared services/fixtures, heavy-test concurrency, and
  timeouts. Run cargo nextest run --locked when supported. Keep resource controls in the existing
  .config/nextest.toml; report initial failures and use retries only under a deliberate flaky-test policy.
- For cargo fix or clippy --fix, start from reviewed changes, select a supported configuration, inspect
  edits, format, and rerun checks. Do not add allow-dirty/staged flags or edition migrations as routine
  cleanup. The rustfix library is not a CLI to install.

## Timing, CPU, and heap

- For Hyperfine, prebuild baseline/candidate binaries, verify outputs, declare cache/setup policy, repeat
  runs, and export evidence with --export-json. Check installed-version shell options before pipelines.
  For a warm-cache comparison, use hyperfine --warmup 3 --export-json timings.json './baseline input'
  './candidate input' with real prebuilt commands. Expect timing statistics/JSON, not correctness checks.
  Do not reset machine-wide caches without specific need and authorization.
- For flamegraph, keep release debug symbols and sample representative work. Verify version/backend and
  permissions: Linux perf, current macOS xctrace, or older DTrace. Run cargo flamegraph --bin NAME -- ARGS
  from the Cargo workspace for the actual binary/input. Expect flamegraph.svg and backend diagnostics;
  width is sampled cost, not chronology. Benchmark separately without capture overhead.
- For DHAT, feature-gate dhat::Alloc and the profiler guard; resolve the single global allocator and cover
  the measured work. Normal guard drop writes dhat-heap.json. Distinguish total bytes, peak/end heap, and RSS;
  serialize heap tests and measure runtime without instrumentation. Run the release executable with the
  chosen profiling feature, not an assumed universal feature name; inspect its JSON in a DHAT viewer.

## Async diagnostics and HTTP load

- Keep one application-owned tracing subscriber; libraries emit events rather than installing globals.
  Filter costly/sensitive fields and avoid eager expensive values. Instrument futures instead of keeping
  Span::enter guards across await; span elapsed time includes waiting and is not CPU time.
- For tracing-chrome, compose a layer, retain its flush guard until tasks finish, bound trace volume, and
  inspect the generated JSON with a compatible viewer. Measure instrumentation overhead separately.
- For Tokio Console, use console-subscriber plus the selected Tokio's tracing support and tokio_unstable cfg.
  Preserve existing flags and TRACE events for tokio/runtime targets. Keep the endpoint local and access
  controlled; compose the existing subscriber. The cfg alone does not require nightly or justify
  diagnostic production runs.
- For oha, use only an authorized endpoint with explicit duration/concurrency, representative routes/payloads,
  auth and reuse policy. For an approved local workload, oha -z 30s -c 20 http://127.0.0.1:8080/PATH
  emits load statistics; substitute the real path and selected limits. Compare latency/errors and server
  resources; detect load-generator saturation and collect instrumentation in a separate diagnostic run.

## CPU pools and shared lookup

- Use Rayon only when a needed CPU data-parallel capability is not supplied by the existing framework.
  Prefer disjoint output or fold/reduce, deliberate pool sizing, and an async handoff that does not block
  workers.
- Use DashMap only for required shared lookup measured against local maps/merge or simpler synchronization.
  Drop entry/reference guards before reentry, await, callbacks, or blocking work; follow each method's
  deadlock contract. Single-key updates are not multi-key transactions; define try_get retry and
  skew/contention policy.
- Enable optional pool/map integration only when used; do not wrap a concurrent map in another mutex by
  default.

Primary tools: [audit][audit], [Bacon][bacon], [nextest][nextest], [Hyperfine][hyperfine],
[flamegraph][flame], [DHAT][dhat], [tracing][tracing], [Chrome traces][chrome], [Console][console],
[oha][oha], [Rayon][rayon], [DashMap][dashmap].

[audit]: https://docs.rs/cargo-audit/latest/cargo_audit/
[bacon]: https://dystroy.org/bacon/
[nextest]: https://nexte.st/
[hyperfine]: https://github.com/sharkdp/hyperfine
[flame]: https://github.com/flamegraph-rs/flamegraph
[dhat]: https://docs.rs/dhat/latest/dhat/
[tracing]: https://docs.rs/tracing/latest/tracing/
[chrome]: https://docs.rs/tracing-chrome/latest/tracing_chrome/
[console]: https://github.com/tokio-rs/console
[oha]: https://github.com/hatoo/oha
[rayon]: https://docs.rs/rayon/latest/rayon/
[dashmap]: https://docs.rs/dashmap/latest/dashmap/struct.DashMap.html

For fixes, use [Cargo fix][fix]; for Rayon, check [parallel iterator contracts][parallel].

[fix]: https://doc.rust-lang.org/cargo/commands/cargo-fix.html
[parallel]: https://docs.rs/rayon/latest/rayon/iter/trait.ParallelIterator.html
