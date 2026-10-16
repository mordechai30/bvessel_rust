# Project instructions

## Shared policy

- Always prefer clarity in code over brevity unless it materially harms performance.
- Use explicit names, intermediate values, and straightforward control flow even when a shorter idiom exists.
- Follow Rust Style Guide formatting; do not trade ownership correctness or safety for readability.
- Try to avoid unsafe Rust as much as possible unless it materially complicates code or dependencies.
- Preserve the project's edition, MSRV, toolchain pin, target support, runtime, and public contracts.
- Verify API stability against the actual compiler; do not prescribe a fixed Rust release for every project.
- Ask for clarification when requirements or supplied references are materially unclear or unavailable.
- Give the user's safety and clarity requirements precedence; retain only compatible source advice.
- Show every source conflict found, link the exact advice, explain its effect, and ask the user to resolve it.
- Pause the affected decision until resolution; reuse an explicit prior resolution for the same conflict.
- Report material missing references, scripts, or assets. Do not claim checks that did not run.
- Load only task-relevant detail; discard unrelated fetched content and tool output from working context.

## Rust

### Memory and ownership

- Use values, arrays, `Vec`, and `String` first; use `Box` for exclusive heap ownership.
- Borrow for inspection, move for ownership transfer, and clone for deliberate independent ownership.
- Pass cheap `Copy` inputs by value; check the size of large inline values before treating copying as cheap.
- Accept `&[T]`, `&mut [T]`, and `&str` for immediate views. Accept growable containers when resizing is
  needed.
- Move owned values into retained work; do not clone merely to silence a borrow error.
- Use `Cow<'a, str>` or `Cow<'a, [T]>` when unchanged results can borrow and changed results require
  ownership.
- Use `Cow::to_mut` for mutation; account for cloning borrowed data and for `into_owned` cloning a borrow.

### Exclusive access

- Use shared references for shared access and mutable references for exclusive access.
- End overlapping borrows before growth or replacement; use explicit scopes for meaningful guard lifetimes.
- Split independent fields or slice regions through safe APIs. Check external indices and disjointness.
- Do not add cloning, `RefCell`, or unsafe merely to evade a borrow that can be narrowed.

### Reference lifetime enforcement

- Tie returned and stored references to their real owners; annotate only relationships the API needs.
- Return ownership when a borrow cannot fit the consumer's lifetime. Do not manufacture `'static`.
- Preserve a moved reference's original lifetime; do not claim task capture extends it.
- Prefer IDs, offsets, or separate owned storage over a self-referential growable buffer.
- Validate reused identities, bounds, and UTF-8 boundaries before resolving stored offsets.
- Copy small retained fields when borrowing would retain a substantially larger owner.

### Shared ownership and interior mutability

- Use `Rc` for single-thread shared ownership, `Arc` for cross-thread shared ownership, and `Weak` for cycles.
- Prefer a borrow when another owner is unnecessary; do not treat `Arc::clone` as a payload snapshot.
- Use `Cell` for replacement, `RefCell` for single-thread checked borrowing, and `Mutex` for synchronized
  access.
- Use recoverable borrow methods for expected conflicts. Choose `RwLock` only from measured access patterns.
- Do not treat `Arc` as a payload lock or use `Arc<RefCell<_>>` to bypass cross-thread synchronization.
- Use `Rc::make_mut` or `Arc::make_mut` for shared copy-on-write only when cloning and weak-reference behavior
  fit.

### RAII, destruction, and pinning

- Store resources in owning fields; use `Drop` for synchronous release and explicit fallible or async close.
- Use `take` or `replace` on borrowed fields only when the replacement preserves domain validity.
- Keep resource state private and construction validated; make destruction dependencies explicit.
- Do not make memory safety depend on a guard destructor running; preserve validity on panic and early return.
- Use `pin!`, `Box::pin`, or maintained projection helpers only for a real address-stability contract.
- Prefer IDs or separate owned buffers when they make pinning or self-reference unnecessary.

### Concurrency: tasks first

- Use task/result APIs and existing pools instead of a thread and synchronization for each operation.
- Use the existing async runtime for nonblocking I/O and bounded CPU pools for sustained computation.
- Preserve framework-owned pools; use a dedicated service thread for required affinity or persistent
  ownership.
- Borrow across `await` when lifetimes and suspended state allow it; cloning `Rc` does not make it `Send`.
- Keep small work sequential when scheduling costs dominate; await directly without needless spawning.
- Join independent I/O futures when suitable; do not confuse concurrent polling with CPU parallelism.
- Move inputs, share immutable snapshots, partition outputs, and merge local results before sharing writes.
- Bound admission before spawning; bound bytes, queued work, and retained completed results too.
- Include pending producers and queued blocking closures in the memory budget.
- Retain handles, distinguish application errors from panic/cancellation, and observe task termination.
- Stop admission on shutdown, then cancel or drain by policy and finish fallible flush/commit operations.
- End ordinary lock guards before await or callbacks; use async guards only for a deliberate resource
  contract.
- Define lock order, reentry, and cancellation behavior for an async guard retained across suspension.
- Cooperatively stop started blocking tasks; aborting their handles does not generally stop execution.
- Define poison recovery and validate snapshot versions before committing work that could be stale.
- Retain partial progress outside canceled futures; check each operation's cancellation contract.
- Define idempotency, transaction, or reconciliation behavior for externally visible effects and retries.
- Let `Send` and `Sync` derive from sound fields; do not add unsafe impls merely to make spawning compile.
- Document publication and reclamation for custom atomics; do not treat `SeqCst` as a transaction.

### Unsafe Rust

- Try to avoid unsafe Rust as much as possible unless it materially complicates code or dependencies.
- Use necessary, sound unsafe at FFI boundaries under this exception; do not treat FFI itself as a conflict.
- Prefer an existing safe wrapper when suitable; keep unavoidable unsafe operations within a narrow boundary.
- Compare safe APIs and established dependencies before accepting an exception; document the concrete reason.
- Do not reject a dependency solely because its implementation contains unsafe; assess its safe API and
  upkeep.
- Keep justified unsafe within a narrow reviewed abstraction; review safe methods that can change invariants.
- Document caller obligations in `# Safety` and exact established facts beside each operation in `// SAFETY:`.
- Enable `unsafe_op_in_unsafe_fn` to keep explicit unsafe operations visible when writing unsafe functions.
- Do not form invalid references, publish uninitialized vector elements, invent lifetimes, or reclaim twice.
- Validate foreign shapes and limits; keep allocation liveness in an explicit owner or unsafe caller contract.
- Normalize permitted null/empty foreign buffers before constructing Rust slices.
- Require foreign pointer validity, ownership, lifetime, aliasing, and buffer limits; null checks alone do not
  prove validity.
- Check foreign error codes and returned lengths before conversion or decoding; preserve NUL/UTF-8 contracts.
- Preserve validity under panic, cancellation, and forgotten guards; do not rely on debug-only preconditions.
- Use focused Miri, sanitizer, or model checks where supported; do not claim passing checks prove soundness.

### Slices, strings, and iterators

- Use safe checked slices and iterators before raw access; check range arithmetic before constructing ranges.
- Validate equal lengths before `zip`; reject or process remainders when partial records are possible.
- Use `String` and `&str` for UTF-8, and byte buffers for arbitrary bytes.
- Use `get` for external text ranges; reject invalid encoding unless lossy replacement is required.
- Use `.iter()` to borrow, `.iter_mut()` to mutate exclusively, and `.into_iter()` to consume.
- Materialize collections only when ownership or repeated work needs them; avoid per-record temporary vectors.
- Use a clear loop when an iterator chain obscures validation, errors, or state changes.

### Type safety, errors, and public APIs

- Use private validated newtypes and enums; preserve validation through deserialization and conversions.
- Use `Option` for absence and `Result` for recoverable failure; preserve useful error context.
- Propagate with `?` where suitable; do not unwrap untrusted or environmental failures or retry every error.
- Use `TryFrom` for narrowing and checked arithmetic for sizes; choose wrapping/saturating semantics
  explicitly.
- Prefer generics or `impl Trait` over dynamic dispatch when concrete types meet the requirement clearly.
- Use dynamic dispatch for required runtime heterogeneity or a measured reduction in code size or compile
  cost.
- Permit runtime-selected service and plugin boundaries; measure before replacing dynamic dispatch with
  generics.
- Apply trait bounds only for actual operations; preserve public lifetimes, auto traits, and future bounds.
- Use consuming methods for ownership transitions and functions/generics before macros.
- Evaluate macro inputs once and preserve hygiene and public paths; review generated unsafe separately.

### Performance

- Set the required latency, throughput, memory, or size target; measure, isolate, optimize, test, and
  remeasure.
- Compare equivalent behavior in representative optimized builds; record input, hardware, compiler, and
  features.
- Profile algorithm cost, allocation, copying, retention, cache behavior, contention, and tail latency as
  needed.
- Remove unnecessary work and allocations before adding low-level complexity; do not skip required processing.
- Prefer contiguous hot data; use separate field arrays for field-selective loops only when measurements
  justify it.
- Reserve from useful checked estimates; reuse buffers with a retention limit after unusually large inputs.
- Prefer safe traversal and local reductions before unchecked indexing, shared atomics, padding, SIMD, or
  packing.
- Preserve numerical order or tolerance, security, panic policy, and target portability when tuning.
- Inline small critical functions when measurement or code inspection shows missed useful inlining.
- Start with `#[inline]`; require evidence for `#[inline(always)]` and recheck runtime, binary size, and build
  time.
- Retain the clearer implementation when performance differs only within measurement noise.
- Stop adding complexity when requirements are met; record material performance reasons beside unusual code.

### Build, tests, naming, and documentation

- Use Cargo as build/package authority; preserve lockfile and dependency policy.
- Follow the [Rust Style Guide](https://doc.rust-lang.org/style-guide/) with rustfmt and the project style
  edition.
- Use the default four-space indentation and 100-character Rust width unless the project requires another
  style.
- Use `snake_case` for functions, fields, and modules, `UpperCamelCase` for types and variants, and uppercase
  constants.
- Prefer meaningful names for ownership, units, capacity, admission, and completion; use clear generic names
  as needed.
- Put `///` before item attributes and `//!` on modules/crates; write useful complete-sentence comments.
- Keep comment-only lines within 80 characters excluding indentation, or the code width if that is smaller.
- Install or integrate all applicable pre-commit checks in the existing hook system; do not overwrite other
  hooks.
- Run `cargo fmt --all -- --check` and project-supported Clippy checks in pre-commit; check the whole
  workspace.
- Run `cargo clippy --workspace --all-targets --locked -- -D warnings` for a valid default configuration.
- Run additional supported feature/target checks in CI; do not assume `--all-features` is a valid product
  matrix.
- Keep lint allowances narrow and explained; reject lint rewrites that reduce clarity without a material
  benefit.
- Test MSRV, boundaries, errors, partial progress, cancellation, shutdown, and relevant borrow/trait
  rejections.
- Use runnable rustdoc examples and compile-fail cases for public contracts; run doctests separately from
  nextest.
- Document ownership, invalidation, failure, cancellation, ordering, and safety where types do not express
  them.
- Before committing new nontrivial code, document fields with purpose and functions/types with purpose and
  usage.

### Toolchain and dependency inventory

- Use the `rust-instructions` skill's checker instructions when toolchain or dependency inventory is needed.
- Report missing, broken, outdated, newer-available, and unknown status; offer explicit install/update
  commands.
- Preserve toolchain pins and MSRV; distinguish an available newer release from a compatible required update.
- Do not install or update without authorization for that change; rerun relevant checks after an approved
  update.
- Check tool binaries separately from declared and lockfile-resolved library crates; do not install libraries
  globally.
- Add only required libraries to the correct normal, optional, or development dependency section.
- Use the `rust-instructions` skill for detailed checker operation and tool-specific integration instructions.

### Tool and crate use

- Select task-relevant tools through `rust-instructions`; preserve the project's existing checks and runtime.
- Use nextest when supported; run doctests separately. Keep Cargo test as a valid existing workflow.
- Add watchers, profilers, Rayon, or DashMap only when the workload needs them; do not require the whole list.
- Preserve diagnostic symbols and panic policy when comparing build profiles or optimization flags.

### Source use

- Distinguish required unsafe from unsound code; do not ask again for an exception already covered by this
  policy.
- Separate source defects from conditional tradeoffs; do not call an unsafe API's documented contract a
  defect.
- Keep ordinary formatting aligned with the Style Guide even when clearer control flow differs from an idiom.
- Use idioms to improve understanding; do not prefer expressions, chains, shadowing, or lint rewrites merely
  because they are shorter.
- Read [Rust idioms](https://rust-unofficial.github.io/patterns/idioms/) and
  [Ian Bull's article](https://ianbull.com/posts/idiomatic-rust/) only for advice compatible with this policy.
- Use exact Rust and crate API contracts to verify semantics; use the Style Guide for presentation.
- Use the [Performance Book](https://nnethercote.github.io/perf-book/) for conditional optimization options.
- Use the [Reintech guide](https://reintech.io/blog/optimizing-rust-code-for-performance-guide) as supporting
  advice.
- Do not use `unreachable_unchecked` as a branch hint or treat example flags as universal production settings.
- Use measure/isolate/optimize for profiling; do not require a historical transcript or reuse its speedups.
- Do not equate total allocation volume with peak memory or promise that every profiler backend works
  everywhere.

- Read [Rust review corrections](references/rust-review-corrections.md) for concurrency and unsafe decisions.

## wgpu and WGSL

- Preserve the resolved crate version; verify Rust descriptors against it and shader rules against WGSL.
- Define logical counts, byte layout, usages, binding ranges, and producer/consumer order before uploading.
- Use verified raw bytes or a shader-aware serializer; separate WGSL layout from transfer/binding alignment.
- Upload distinct command records. Queue writes to one offset before one submit are not draw snapshots.
- Order shared-queue uploads and submissions explicitly; CPU task completion is not GPU completion.
- Advance ping-pong state per encoded step. Use dispatch boundaries for global compute dependencies.
- Guard loads with control flow before barriers; `select` evaluates both value arguments.
- Reclaim mapped slots after completion; reject readback from obsolete device/request generations.
- Let safe handles retain ordinary resources; distinguish dropping, explicit destruction, and raw HAL access.
- Prefer validated shaders; justify unchecked shaders, disabled initialization, and experimental features
  separately.
- Preserve transparent composition order, complete pipeline cache keys, and populated mip levels.
- Read [GPU contracts](references/wgpu-contracts.md) for data flow, concurrency, unsafe, and source evidence.
