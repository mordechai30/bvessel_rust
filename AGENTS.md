# Project instructions

## Scope and priorities

- Use the `rust` skill for Rust planning, implementation, review, concurrency, unsafe, and CPU performance.
- Use the `wgpu` skill for Rust wgpu/WGSL work. Use both skills for mixed host/shader work.
- Apply the project rules below when a skill or source gives different advice.
- Read only material needed for the current decision. Discard unrelated fetched content after extracting
  evidence.
- Prefer explicit validation, named intermediate values, and straightforward control flow over shorter idioms.
  Accept extra complexity only for a material requirement or measured execution-time or memory benefit.
- Reject style-only lint rewrites that obscure ownership, errors, or state transitions.
- Apply rustfmt and the selected style edition. Use the Rust Style Guide's [width], [comments], and [docs]
  rules.

## Toolchain and compile-time optimization

- Use stable Rust 1.99.1 or newer for production builds. Do not equate the stable channel with memory safety.
- Use nightly only for selected compiler experiments or diagnostics; keep production code stable-compatible.
- Measure clean builds and representative incremental rebuilds separately. Run `cargo build --timings` to find
  expensive crates; inspect the HTML report whose path Cargo prints under `target/cargo-timings/`.
- Set `[profile.dev] debug = "line-tables-only"` for quick iteration; restore `debug = "full"` when inspecting
  variables or function parameters in a debugger.
- Try eight compiler frontend threads as a starting point; compare build time and peak memory. For the current
  package, run `cargo +nightly rustc -- --jobs-frontend=8` on a supporting compiler. Older nightlies use
  `cargo +nightly rustc -- -Zthreads=8`; verify the selected compiler's supported spelling first.
- Use Cranelift for development builds that support the required instructions and do not need panic unwinding.
  Install `rustc-codegen-cranelift-preview` for the selected nightly before using it. Run `cargo +nightly
  -Zcodegen-backend build --config 'profile.dev.codegen-backend="cranelift"'`. Use LLVM for release
  measurements and builds that require unsupported intrinsics or unwind behavior.
- Keep nightly flags scoped to explicit nightly commands; do not place them in flags used by stable builds.
- Inspect macro expansion and monomorphization when build timings identify them as bottlenecks. Move
  substantial non-generic work out of repeatedly instantiated generic bodies; do not refactor unrelated code.
- Read [build configuration] and [compile times] for additional measured options; do not copy example
  speedups.

## Checks and profiling

- Run all scripts and CLI workflows outside sandbox restrictions. Do not read script implementations unless
  explicitly asked; use their documented inputs, outputs, and invocation instructions.
- Run `cargo fmt --all -- --check` and `cargo clippy --workspace --all-targets --locked -- -D warnings`.
  Select valid feature/target combinations; do not replace the product matrix with `--all-features`.
- Enable workspace Clippy `pedantic` and `nursery` warnings; keep justified exceptions narrow and explained.
- Use `bacon clippy` or `bacon test` for background feedback. Run completed final checks separately.
- Use `cargo nextest run --locked` for compiled tests when supported; run `cargo test --doc --locked`
  separately.
- Run `cargo audit` and `cargo deny check` in dedicated CI checks; do not repeat them in every pipeline step.
- Use these tools for the limiting cost; replace placeholders with actual binaries, targets, and arguments:

| Purpose | Invocation or setup | Input and output |
| --- | --- | --- |
| End-to-end timing | `hyperfine --warmup 3 --export-json timings.json './baseline input' './candidate input'` | Prebuild equivalent binaries; replace both commands. Inspect timing distributions and `timings.json`. |
| CPU sampling | `cargo flamegraph --bin NAME -- ARGS` | Replace `NAME` and `ARGS`; retain release symbols, verify the platform backend, and inspect `flamegraph.svg`. |
| Heap allocation | Gate `dhat::Alloc` and `dhat::Profiler::new_heap()` in a diagnostic build | Resolve the single global allocator; retain the guard through measured work. Inspect `dhat-heap.json`. |
| GPU execution | Use supported timestamp queries and a target-backend capture | Separate CPU preparation/encoding, GPU work, transfers, and presentation latency. |

- Remeasure runtime without profiler instrumentation. Compare equivalent output quality and realistic
  workloads. Distinguish allocation volume, peak live memory, retained capacity, and process resident memory.

## Memory layout and hot paths

- Prefer `Option<NonZeroU8>` over `Option<u8>` when present values exclude zero. Preserve `Some(0)` when
  meaningful; do not change domain values merely to obtain a smaller representation. Use checked nonzero
  construction.
- Use `Box<[T]>` and `Box<str>` for finished owned data that needs no growth or capacity reuse. Keep `Vec` and
  `String` for reusable construction, staging, and scratch buffers; account for conversion/reallocation costs.
- Measure actual type size, alignment, field offsets, and allocation size classes. Do not sort default-layout
  Rust fields mechanically. Preserve explicit FFI and Rust/WGSL layouts when changing representation.
- Reduce per-object overhead in large collections: use bounded compact offsets, combine related owned lists,
  omit reconstructible repeated data, and pack flags when their access patterns and contracts permit it.
- Compare boxing rare large enum variants with contiguous storage. Include added allocations and pointer
  chasing; use encoded records when sequential access and avoided serialization outweigh decoding costs.
- Reuse construction buffers, reserve from checked estimates, and bound retained capacity after large inputs.
  Use `clone_from` when cloning into an existing destination can reuse its allocation.
- Avoid unnecessary hot-loop cloning, string conversion, intermediate collections, and repeated serialization.
  Use direct loops when clearer; use `extend` instead of collecting a temporary solely to append it.
- Use contiguous hot data and disjoint worker outputs. Change AoS/SoA layout or parallel granularity only for
  measured access patterns; preserve entity identity and numerical order/tolerance.
- Try safe iteration, explicit range invariants, and slice-based loops before unchecked indexing. Inspect
  generated code when necessary; a hot loop alone does not prove remaining checks or a material speedup.
- Use faster hashers only for inputs whose collision risk permits them. Measure inlining, SIMD, LTO, codegen
  units, allocator changes, and PGO separately; preserve deployment CPU support and panic behavior.
- Read [Cloudflare's cache optimization] for compact immutable storage and [the Performance Book] for other
  techniques. Treat their workload-specific results as evidence for mechanisms, not promised gains.

## Unsafe code constraints

- Strongly prefer safe Rust. Permit sound unsafe optimizations only when safe alternatives materially worsen
  execution time or memory use. Do not accept code complexity alone as the performance exception.
- Keep each unsafe operation small and isolated. Record its measured benefit and explain established facts in
  a preceding `// SAFETY:` comment; document unsafe caller obligations under `# Safety`.
- Expose a safe API only when its implementation establishes every safety invariant. Mark an interface unsafe
  when callers must supply guarantees it cannot establish. Review safe mutations and destruction too.
- Do not reject a dependency solely for internal unsafe; evaluate its safe interface and maintained contracts.
- Permit these exceptions only with the proofs in [unsafe performance contracts]:
  - Unchecked hot-path indexing after bounds validity is established and retained.
  - Direct initialization of spare capacity when avoiding an extra initialization pass materially helps.
  - Shared raw output pointers when disjoint writes, allocation liveness, and completion are proved.
  - Trusted iterator lengths when unchecked construction has a complete producer/consumer contract.
  - Zero-copy FFI/type conversion with proved layout, value validity, lifetime, ownership, and destruction.
  - Architecture intrinsics with proved CPU-feature, alignment, and access requirements.
- Test boundary inputs and interruption paths. Use supported memory/model diagnostics for the changed proof;
  do not execute deliberate undefined behavior in ordinary tests or treat passing tests as soundness proof.

## Google Rust Style Guide and idioms

- Use the linked sections of the local Google guide. Apply this project's unsafe and toolchain exceptions; do
  not import Chromium/Fuchsia-only build, runtime, or repository rules.
- Use [flexible immutable inputs]: `&T`, `&[T]`, `&str`, and `Option<&T>` when only the payload is needed.
  Accept the container when its capacity, ownership, or replacement is part of the operation.
- Prefer unique ownership and borrowing over [unnecessary `Rc`/`Arc`]. Retain shared ownership when
  alternatives are unreasonably complex; distinguish cloning a shared handle from copying its payload.
- Use [pattern matching] instead of checking `is_some()` and then unwrapping. Use `expect` for a necessary,
  established invariant that cannot be expressed by the type or control flow.
- Use [newtypes] for distinct domain identities or validated state; preserve validation through construction
  and conversion. Use aliases only when a distinct type contract is unnecessary.
- Use [`non_exhaustive`] for externally consumed enums intended to grow; keep deliberately closed enums
  exhaustive. Do not apply this external-evolution rule mechanically to internal application states.
- Use [`must_use`] when ignoring a returned value defeats the operation; do not duplicate an existing
  return-type annotation without a reason.
- Prefer [`format!`/`write!`] for multi-part formatting and substitution. Write into an existing buffer when
  reuse avoids allocation; do not assume formatting always allocates less than every use of `+`.
- Use typestate for useful compile-time ownership transitions. Keep runtime checks for external or changing
  facts and use runtime state machines where retries, persistence, or concurrency require them.

## wgpu and WGSL

- Retain the Rust ownership contract at GPU handoff boundaries. Establish bytes, ranges, usages, logical
  counts, and producer/consumer order together; do not treat host byte validity as proof of WGSL
  compatibility.
- Keep GPU-produced state on the GPU for its next shader/indirect consumer. Reuse resources while their
  contracts hold; distinguish CPU completion, queue submission, and completed GPU use.
- Preserve validation and initialization by default. Apply separate proofs before unchecked shader, raw HAL,
  or disabled-initialization paths; CPU-side unsafe approval does not establish GPU safety.

[width]: https://doc.rust-lang.org/style-guide/#indentation-and-line-width
[comments]: https://doc.rust-lang.org/style-guide/#comments
[docs]: https://doc.rust-lang.org/style-guide/#doc-comments
[build configuration]: https://nnethercote.github.io/perf-book/build-configuration.html
[compile times]: https://nnethercote.github.io/perf-book/compile-times.html
[Cloudflare's cache optimization]: https://blog.cloudflare.com/dns-cache-memory-optimization-1111/
[the Performance Book]: https://nnethercote.github.io/perf-book/print.html
[unsafe performance contracts]: references/rust-performance-contracts.md
[flexible immutable inputs]: references/RustStyleGuideGoogle.md#accept-flexible-references-for-immutable-data
[unnecessary `Rc`/`Arc`]: references/RustStyleGuideGoogle.md#avoid-rc-and-arc
[pattern matching]: references/RustStyleGuideGoogle.md#prefer-pattern-matching-over-check-then-unwrap
[newtypes]: references/RustStyleGuideGoogle.md#newtypes-versus-type-aliases
[`non_exhaustive`]: references/RustStyleGuideGoogle.md#apply-non_exhaustive-to-publicly-exposed-enums
[`must_use`]: references/RustStyleGuideGoogle.md#must_use-on-functions
[`format!`/`write!`]: references/RustStyleGuideGoogle.md#prefer-format--write-for-concatenation-and-substitution
