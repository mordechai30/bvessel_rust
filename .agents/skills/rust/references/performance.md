# CPU performance decisions

- Set a workload-specific latency, throughput, memory, or size target. Profile the limiting cost before
  adding complexity; stop when requirements are met and retain clearer code within measurement noise.
- Compare equivalent behavior in representative optimized builds. Record compiler, profile, features,
  target, hardware, inputs, setup amortization, cache policy, variance, and independent validation workloads.
- Separate total allocation volume, peak live heap, retained capacity, and RSS. Test small requests after
  unusually large requests; lazy iteration alone does not release a large owned input or buffered results.
- Remove repeated work/intermediate collections before raw access. Buffer small I/O and flush explicitly
  where error reporting matters; reserve from bounded estimates instead of maximum imaginable inputs.
- Use VecDeque for FIFO/end operations when split storage is acceptable; choose BTreeMap for ordered/range
  access and HashMap for lookup without ordering. Measure a linear vector for genuinely small collections.
  Use entry APIs when a measured repeated lookup is avoidable; preserve error and fallback evaluation.
- Compare contiguous storage, stable-address indirection, arenas, and inline containers by actual access
  and size distributions. Inspect type size/alignment rather than assuming Rust field declaration order.
- Use SoA for a measured field-selective bottleneck; maintain equal-length/entity identity invariants.
  Partition local reductions before per-item atomics, shared maps, or false-sharing padding.
- Preserve float order/tolerance and checked-integer intermediate behavior when regrouping reductions.
- Compare sequential/parallel granularity and skew; scoped threads provide lifetime bounds, not a reusable
  pool. Batch with a latency limit so throughput gains do not create unbounded response delay.
- Move substantial non-generic work out of generic bodies when it reduces measured monomorphization cost.
  Keep runtime-selected service/plugin/task boundaries dynamic where they fit; optimize dispatch granularity
  before replacing them with more generic variants.
- Inspect specialization, dispatch granularity, and vectorization before adding inline hints or intrinsics.
  Start useful cross-crate hints with inline; require evidence for always, cold outlining, or unchecked
  access.
- Keep target-feature preconditions and runtime fallback for SIMD; verify portable-SIMD stability. Never
  adopt target-cpu=native for a portable deployment without an explicit deployment contract.
- Do not trade adversarial-key collision protection for a faster deterministic hasher without assessing trust.
- Compare LTO/codegen units/size profiles with build time, code size, and runtime. Preserve panic/cleanup
  behavior, deployment support, diagnostic symbols, allocator policy, and reproducibility.
- Use representative PGO training plus independent validation. Apply flags at the intended target/profile
  scope; do not silently alter build scripts, dependencies, tests, or error semantics for a benchmark.
- Use black_box only as a best-effort optimization barrier, not a model of production inputs. Do not use
  unreachable_unchecked as a branch hint, and never use debug assertions as the only safety enforcement.

Sources: [Performance Book][perf], [profiles][profiles], [inlining][inline], [PGO][pgo].

[perf]: https://nnethercote.github.io/perf-book/benchmarking.html
[profiles]: https://doc.rust-lang.org/cargo/reference/profiles.html#profile-settings
[inline]: https://doc.rust-lang.org/reference/attributes/codegen.html#the-inline-attribute
[pgo]: https://doc.rust-lang.org/rustc/profile-guided-optimization.html
