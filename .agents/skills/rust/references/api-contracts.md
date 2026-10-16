# API and data contracts

## Validated state and compatibility

- Preserve validated state through private fields, constructors, defaults, setters, conversion, serde,
  and test helpers. Do not derive deserialization onto a representation that bypasses its invariants.
- Bind validated indices to the collection/version they authorize; revalidate after replacement or removal.
- Use typestate for useful compile-time transitions; use a runtime state machine for retries, persistence,
  or concurrent actors when that represents the domain better. Neither proves remote state is still live.
- Treat public lifetimes, auto traits, returned-future bounds, blanket impls, and exhaustiveness as contracts.
- Use an associated type for one implementation-selected related type; use a generic trait parameter when
  multiple implementations for different inputs are required. Use ?Sized for actual unsized borrowers.
  Use higher-ranked callbacks for arbitrary borrow lifetimes instead of needlessly requiring static data.
- Check dyn compatibility separately from trait validity. Use a local newtype for foreign-trait/foreign-type
  extensions; consider downstream blanket impls, sealing, and default methods before changing a public trait.
- Use must_use for results whose loss predictably breaks intended use; never make memory safety depend on it.
- Distinguish contained collection logic errors from undefined-behavior obligations.
- Keep Eq/Hash/Ord consistent. Do not mutate stored keys' comparison/hash behavior through interior
  mutability.
- Use Deref only for pointer-like semantics; do not use it to hide unrelated conversions or imitate
  inheritance.

## Arithmetic, ranges, and text

- Check narrowing, start-plus-count, element-size multiplication, shifts, and exceptional division before
  allocation/indexing. Verify pointer-width-dependent behavior on supported targets, not only a 64-bit host.
- Choose checked, modular, or saturating arithmetic from domain semantics, not release overflow settings.
- Validate equal lengths before zip and forbidden remainders after chunks_exact; do not silently lose records.
- Define NaN, infinity, signed-zero, tolerance, and ordering behavior. total_cmp orders values; it does not
  validate finiteness. Reject NaN explicitly where a comparison must establish a valid tolerance.
- Decode unaligned/wire bytes with checked endian operations. Do not cast packed bytes to typed references.
- Distinguish byte offsets, Unicode scalar values, and grapheme boundaries in cursor/truncation APIs.
- Use checked text ranges for external offsets. Choose reject versus lossy replacement deliberately;
  account for retained input in borrowed deserialization and allocation behavior in owned conversion APIs.

## Failure, callbacks, serialization, and macros

- Preserve source errors and enough operation context to choose retry, rejection, or reconciliation.
  Return consumed inputs with errors when callers must recover ownership; do not unwrap environmental or
  untrusted failures. Keep internal expect conditions close to the facts that establish them.
- Define intentional panic conditions; callbacks, hashing, formatting, comparisons, cloning, and Drop
  can panic even inside otherwise simple container operations.
- Define wire versions, unknown tags/fields, duplicate-key policy, maximum sizes, and normalization.
  Test semantic round trips; do not promise byte-identical output if equivalent encodings are normalized.
- Prefer generics when concrete callers benefit; permit dyn at runtime-selected service/plugin boundaries.
  Compare dispatch granularity, binary size, and build cost before replacing either design.
- Preserve eager/lazy fallback evaluation and side effects when replacing branches with combinators.
- Prefer functions/generics or const fn when syntax generation adds no capability. Do not assume const fn
  forces runtime calls into compile-time evaluation.
- Evaluate macro expression inputs once; use defining-crate paths for exported macros and test external
  callers, renamed dependencies, visibility, and editions. Hygiene does not prove generated unsafe sound.

Document ownership/invalidation, panic/error conditions, cancellation, and ordering when types cannot
express them. Add a runnable public example when it clarifies one of those contracts; use compile_fail for
an intended rejection. Do not require examples that merely restate syntax.

Sources: [dyn compatibility][traits], [conversions][conv], [slice APIs][slice], [macro hygiene][macro],
[serde validation][serde], [API compatibility][semver].

[traits]: https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility
[conv]: https://doc.rust-lang.org/std/convert/
[slice]: https://doc.rust-lang.org/std/primitive.slice.html
[macro]: https://doc.rust-lang.org/reference/macros-by-example.html#hygiene
[serde]: https://serde.rs/try-from.html
[semver]: https://doc.rust-lang.org/cargo/reference/semver.html
