# Unsafe performance contracts

Use these exceptions only for a measured material execution-time or memory benefit over suitable safe code.
Keep source examples as mechanisms to assess, not implementation requirements or promised speedups.

## Unchecked indexing

- Establish every index against the actual slice before the hot kernel, or derive validity from its owner.
  Retain that guarantee when data is replaced or resized; do not repeat an already established check per
  access.
- Use a safe wrapper when it owns and enforces the invariant. Otherwise expose an unsafe caller contract:

```rust
/// Sums values through indices already validated by the caller.
/// Avoids per-access bounds checks and preserves index order.
///
/// # Safety
/// Every index in `idx` must be less than `values.len()`.
unsafe fn sum_at(values: &[f64], idx: &[usize]) -> f64 {
    let mut total = 0.0;
    for &i in idx {
        // SAFETY: The caller guarantees every index is within this slice.
        total += unsafe { *values.get_unchecked(i) };
    }
    total
}
```

- Compare with safe indexed and iterator implementations in an optimized build. Do not assume checks remain
  in generated code or that removing them improves the application's limiting cost.
- Follow the [get_unchecked contract](https://doc.rust-lang.org/std/primitive.slice.html#method.get_unchecked).
  Never use `debug_assert!` as the sole enforcement of a safety precondition.

## Initialize spare capacity once

- Compare safe `with_capacity` plus `push`/`extend` before manual initialization; these avoid a zero-filled
  intermediate too. Use `vec![0; n]` when zero is the actual required result.
- Reserve before taking pointers. Write valid elements through `spare_capacity_mut` or a proved raw interface;
  publish only the initialized prefix with `set_len`, never the allocation's entire capacity by assumption.
- Handle partial writes and panicking element construction. Reclaim initialized elements appropriately and
  keep uninitialized elements unavailable to reads and destruction; account for zero-sized types.
- Follow [Vec::set_len](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.set_len) and
  [MaybeUninit](https://doc.rust-lang.org/std/mem/union.MaybeUninit.html).

## Share disjoint output across workers

- Try safe split slices, scoped borrowing, or the existing pool's disjoint-output APIs first. Do not require
  each worker to allocate its own vector when one safely partitioned allocation suffices.
- For a raw-pointer wrapper, prove access permissions, nonoverlapping writes, allocation liveness, and no
  reallocation through every worker's completion. Make manual `Send`/`Sync` bounds fit all exposed operations.
- Preserve exclusive access even without simultaneous CPU instructions. Observe worker termination on error
  and panic before reclaiming storage; do not rely on a forgotten guard to restore memory safety.
- Follow [slice splitting](https://doc.rust-lang.org/std/primitive.slice.html#method.split_at_mut) and
  [Send's unsafe contract](https://doc.rust-lang.org/std/marker/trait.Send.html).

## Trust an iterator length

- Use safe `size_hint`, `ExactSizeIterator`, and reservation where sufficient. Do not treat those safe traits
  as a memory-safety guarantee for unchecked output writes.
- For an unsafe trusted-length interface, prove the exact yielded extent, overflow behavior, allowed
  divergence, and initialization cleanup. Establish both the producer's promise and the consumer's handling.
- Keep production stable-compatible: standard-library `TrustedLen` is nightly-only. Do not adopt an unstable
  trait or a crate-specific replacement merely to match a tutorial.
- Check [TrustedLen](https://doc.rust-lang.org/std/iter/trait.TrustedLen.html) and
  [ExactSizeIterator](https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html).

## Reinterpret data without copying

- Prefer established typed FFI/export or checked byte-cast APIs. Do not hand-write a conversion already
  supplied by a suitable safe interface; a small descriptor copy can still preserve zero-copy payloads.
- Prove representation, valid destination values, pointee alignment, initialization, provenance, lifetime,
  aliasing, allocator pairing, and exactly-once destruction. Equal byte sizes alone do not prove conversion.
- For by-value `transmute`, retain valid source/destination values and do not depend on padding preservation.
  For allocation conversion, prove layout and deallocation alignment separately from element byte counts.
- Encode wire/file formats explicitly when native representation does not match their endian/layout contract.
  Treat Rust/WGSL layout as a separate ABI; a compact CPU type is not automatically shader-compatible.
- Follow [transmute](https://doc.rust-lang.org/std/mem/fn.transmute.html) and
  [Rust layout guarantees](https://doc.rust-lang.org/reference/type-layout.html).

## Use architecture intrinsics

- Inspect safe-loop vectorization before introducing intrinsics. Select only instructions available on the
  deployment CPU, with guarded runtime selection and a valid fallback where targets differ.
- Prove each intrinsic's pointer, alignment, extent, and target-feature requirements; preserve the numerical
  tolerance/order required by the caller. Check the selected development backend supports the intrinsics too.
- Follow [std::arch](https://doc.rust-lang.org/std/arch/index.html).

## Source interpretation

- Use [the Polars video](https://www.youtube.com/watch?v=l6tisoOzTuk) as a transcript-based catalogue of the first
  five mechanisms. Do not attribute its whole-library comparison with Pandas to an isolated unsafe change.
- Use [bounds-check guidance](https://nnethercote.github.io/perf-book/bounds-checks.html) and
  [machine-code inspection](https://nnethercote.github.io/perf-book/machine-code.html) for optimization evidence.
- Preserve Rust's validity and ownership contracts throughout every exception; unsafe changes who proves them.
