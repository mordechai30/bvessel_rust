# Rust compiler, edition, and API migrations

- Separate compiler upgrades, MSRV changes, edition changes, style-edition changes, and dependency updates.
  Change only the authorized contract; a newer installed compiler does not authorize the other changes.
- Check API stabilization annotations and target availability before adopting a newer safe operation.
  Prefer an established compatible safe API over raising MSRV solely to match a source example.
- For an edition migration, review generated fixes and edition-sensitive temporary/destruction scopes,
  macro fragment behavior, extern declarations, and unsafe attributes. Test old consumers where promised.
- Preserve public auto traits, returned-future bounds, coherence, exhaustiveness, panic behavior, and wire
  representation when replacing an implementation. Treat a bound or enum-variant change as an API decision.
- Recheck the resolved dependency feature graph and transitive MSRV, not just the manifest's version range.
  Test consumer-style resolution for libraries instead of relying only on the library's own lockfile.
- Replace leak/reclaim patterns with paired ownership-transfer APIs when changing raw ownership code.
  Check actual support for Box NonNull methods and Vec parts methods; do not infer safety from stabilization.
- For raw DST layout queries, inspect size_of_val_raw, align_of_val_raw, and Layout::for_value_raw rather
  than forming an otherwise invalid reference. Meet the exact metadata/extent conditions of the chosen API.
- Use String::from_utf8_lossy_owned or FromUtf8Error::into_utf8_lossy only for an intentional replacement
  policy on a supporting compiler. Retain strict UTF-8 rejection for protocols that require valid text.
- Recheck boxed-array iteration consumption and borrowing when adopting IntoIterator improvements.
  Do not turn a newly available consuming operation into accidental ownership loss.
- Treat raw_borrows_via_references diagnostics as prompts to remove needless references, not proof that a
  later raw dereference is valid. Check current support for negative impls, SIMD, variadics, and async APIs.

Use [edition migration][edition], [Cargo MSRV][msrv], [raw layout][layout], and [owned UTF-8][utf8]
for the relevant transition. Resolve exact per-method support before using a release-specific API.

[edition]: https://doc.rust-lang.org/edition-guide/rust-2024/index.html
[msrv]: https://doc.rust-lang.org/cargo/reference/rust-version.html
[layout]: https://doc.rust-lang.org/std/alloc/struct.Layout.html#method.for_value_raw
[utf8]: https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_lossy_owned
