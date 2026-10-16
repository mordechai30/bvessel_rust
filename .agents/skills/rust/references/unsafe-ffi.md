# Unsafe abstractions and foreign boundaries

## Proof surface

- Prefer a suitable safe wrapper. Before using unsafe, record why safe APIs or established dependencies
  cannot meet the requirement without material complexity; keep the accepted abstraction narrow.
- Write caller obligations under Safety and connect each SAFETY comment to established facts. Enable
  unsafe_op_in_unsafe_fn; an unsafe function body still needs explicit unsafe operations and proofs.
- Review the entire safe surface: constructors, setters, generics, callbacks, destructors, panic paths,
  cancellation, and forgotten guards. Private modules do not reduce safe callers' guarantees.
- Never rely on a guard destructor or debug-only assertion to restore memory validity. Distinguish safety
  on interruption from stronger transactional rollback; catch_unwind cannot repair invalid state.
- Keep allocation liveness, pointer provenance, initialization, aliasing, and unique reclamation explicit.
  Null/range/alignment checks alone cannot establish arbitrary foreign-pointer validity.

## Raw access and initialization

- Form references only after validity is established; use raw borrows for packed/uninitialized addresses.
- For raw slices, prove one live allocation, non-null aligned pointer even for empty/ZST cases, initialized
  valid elements, no wrap, size <= isize::MAX, owner lifetime, and permitted access for the whole borrow.
  Normalize foreign null/empty ranges before construction; never join merely adjacent allocations.
- Preserve provenance in pointer/address manipulation. Wrapping arithmetic permits intermediate addresses,
  not dereference; add/offset/difference still have allocation and overflow obligations.
- Treat NonNull as non-nullness only. Use dangling sentinels only where the exact empty/ZST API permits them.
- Query raw DST layout without forming a reference only when metadata is valid: check slice lengths and
  representable extent, trait-object vtables, zero-length special cases, and extern-type limitations.
- Use UnsafeCell for permitted interior mutation; it does not allow data races, invalid values, conflicting
  mutable references, or unchanged niche layout in containing types. Volatile is not atomic synchronization.
  Establish atomic alignment, valid objects, and compatible access discipline; fences do not repair racing
  ordinary and atomic accesses.
- Track initialized prefixes in MaybeUninit storage and clean up on panic. Zero bits are not valid for every
  T; assume_init_ref/mut cannot initialize T. Publish Vec length only after writing valid elements.
- Reserve before taking raw pointers. Handle partial foreign writes, panicking constructors, and ZSTs;
  prefer safe extension over spare-capacity manipulation without a demonstrated need.
- Review ptr::read/write/copy ownership effects: avoid duplicate drops, lost old owners, and invalid overlap.
  ManuallyDrop retains T's validity requirements and can expose dropped state through safe derived methods.

## Ownership, variance, and affinity

- Pair Box/Vec raw or NonNull transfers with exactly one matching reconstruction, allocator, type, layout,
  length, and capacity. Do not reconstruct Vec<u8> from differently aligned element storage by byte count.
- Use ownership-transfer APIs instead of leaked-reference reclamation. Raw parts are not wire representations.
- Choose PhantomData for actual lifetime, drop checking, variance, and auto-trait behavior. NonNull is
  covariant; writable shared wrappers may need invariance. Check stability before negative impl recipes.
- Prove manual Send/Sync across every operation, allocator, callback, and destructor. A mutex cannot legalize
  moving or destroying a thread-affine foreign handle on the wrong thread.
- Use established reclamation for lock-free pointers; an acquired published pointer may still be freed later.

## FFI and ABI

- Use exact calling conventions, target C types, integer tags plus checked conversion, and opaque owners.
  Mark an exported function unsafe when a safe signature cannot enforce the caller's pointer obligations.
  Use validated IDs/registries when untrusted foreign callers must be able to submit invalid handles safely.
  repr(C) is not recursive field compatibility, a stable Rust ABI, valid bits, or a portable wire format.
- Check repr(transparent) restrictions and public representation commitments. Encode padding/endian-sensitive
  data explicitly; never create an intermediate reference to a misaligned packed field.
- Use raw unaligned reads only with valid bits and correct ownership; non-Copy reads can create double owners.
- Check foreign error codes and signed lengths before narrowing/allocation/decoding. Define success/failure
  output initialization and lifetime of returned errors; never return temporary String pointers.
- Preserve NUL contracts, CString owners, and matching destructors. A temporary argument lives through its
  call, not foreign retention; do not include a terminator in CString::new and unwrap it.
- Require explicit allocation/readability guarantees before scanning C strings; preserve reclaimed CString
  length restrictions and exactly-once ownership transfer.
- Define concurrent/reentrant callback behavior and unregister completion before freeing context. Passing
  Arc::as_ptr does not transfer a strong owner; retain it through the callback lifetime.
  Do not invoke reentrant foreign code under a non-reentrant lock it may reacquire; snapshot inputs or
  validate a state-machine transition around the call.
- Define unwind ABI and failure translation. C-unwind is not a default; catch_unwind does not catch abort
  or establish safe handling of arbitrary foreign exceptions. Preserve state before exposing failure.
  A Rust panic reaching a non-unwind ABI terminates; foreign unwinding through a non-unwind boundary can
  be undefined behavior. Select C-unwind only for an intended, supported cross-language unwind contract.
- Use edition-appropriate unsafe extern blocks/symbol attributes; prove exported symbol uniqueness.
- For required variadics, verify actual toolchain support, promoted C types/count, VaList lifetimes/copies,
  and supported target ABIs. Typed retrieval cannot verify the caller's actual argument list. Apply C
  integer promotions and float-to-double promotion; use typed slices for ordinary Rust interfaces.

Use documented obligations and conservative established rules: Rust's memory model is not fully finalized.
Treat experimental aliasing models and passing diagnostics as defect-finding evidence, not permission to
ignore a documented contract.

Sources: [UB][ub], [raw slices][slice], [MaybeUninit][init], [layout][layout], [unwinding][ffi],
[memory-model status][model], [UnsafeCell validity][cell].

[ub]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[slice]: https://doc.rust-lang.org/std/slice/fn.from_raw_parts.html#safety
[init]: https://doc.rust-lang.org/std/mem/union.MaybeUninit.html
[layout]: https://doc.rust-lang.org/reference/type-layout.html
[ffi]: https://doc.rust-lang.org/nomicon/ffi.html#ffi-and-unwinding
[model]: https://doc.rust-lang.org/reference/memory-model.html
[cell]: https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html#aliasing-rules
