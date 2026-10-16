# Ownership and retained state

## Boundaries and identity

- Choose an owned result when borrowing would retain a much larger input or prevent required mutation.
  Account for both copies and retained owners; do not clone merely to silence a borrow error.
- Use Cow only when unchanged data can borrow and changed data must own. Account for cloning on to_mut
  of a borrowed value and into_owned of a borrow; do not add the enum when both branches always allocate.
- Inspect large inline Copy values and future captures before treating moves/copies as cheap.
- Accept a growable container only when capacity/growth is part of the API; otherwise take a view.
  Use owned results when retention requires them, not generic ownership conversion without caller benefit.
- Split independent fields or slices through safe APIs. Check external indices and disjointness, including
  duplicate indices, empty regions, and zero-sized elements. Check disjoint-access APIs against MSRV.
- Tie stored/returned references to the actual owner. Moving a reference into retained work does not
  extend its lifetime; avoid artificially coupling unrelated input lifetimes.
- Prefer stable IDs or offsets over self-referential growable storage. Validate generation, bounds,
  invalidation, and UTF-8 boundaries before resolution. Handle generation exhaustion instead of silent reuse.
- Treat an offset into reused scratch storage as temporary, not a durable identity.
- For custom mutable iterators, never yield an element again while a prior yielded mutable reference may
  remain live. Shrink the remaining exclusive region instead of repeatedly borrowing the original whole.

## Shared ownership and conditional copying

- Treat Arc/Rc cloning as another owner of the same allocation, not an independent payload snapshot.
- Use get_mut when documented uniqueness suffices; use make_mut only when copy-on-write and weak-reference
  behavior match the model. Account for weak disassociation as well as payload cloning.
- Use Weak for non-owning back-references; handle failed upgrade and avoid strong reference cycles.
- Use recoverable RefCell borrowing for expected conflicts. Do not hide synchronization problems inside Arc.
- Select interior-mutability operations by their actual bounds; Cell is not restricted to Copy types.
- Prefer a single initialization primitive over a custom publication protocol; avoid recursive initialization.

## Storage retention and destruction

- Bound untrusted capacity estimates before reservation. Do not claim one successful try_reserve makes
  later allocations fallible or prevents every allocation failure in the enclosing operation.
- Use Box<[T]> for a finished fixed-length owned sequence when its conversion and capacity tradeoff fits.
- Reuse buffers with a retention limit for unusually large requests. Compare peak live and retained memory.
- Select inline containers from measured size distributions; account for larger instances even after spilling.
- Use arenas only when bulk lifetimes, destructor behavior, and escaping references fit the ownership model.
- Use take/replace on a borrowed field only when the replacement is domain-valid on every exit.
- Make destruction dependencies explicit. Rust struct fields drop in declaration order after Drop;
  locals and temporaries follow their own scopes, including edition-sensitive temporary lifetimes.
- Provide explicit fallible/async close where completion matters; a destructor cannot report those results.

## Address stability

- Use pinning only for a real pointee address-stability contract. Moving the Pin handle is not moving
  its pointee; Unpin and structural projection determine which replacement is permitted.
- Preserve pinned-field validity through safe accessors, projection, state replacement, and destruction.
- Prefer maintained projection helpers or separate owned storage to hand-written self-reference.

Sources: [drop scopes][drop], [Cow promotion][cow], [Arc uniqueness][arc], [pinning][pin],
[Vec guarantees][vec], [borrow splitting][split].

[drop]: https://doc.rust-lang.org/reference/destructors.html
[cow]: https://doc.rust-lang.org/std/borrow/enum.Cow.html#method.to_mut
[arc]: https://doc.rust-lang.org/std/sync/struct.Arc.html#method.make_mut
[pin]: https://doc.rust-lang.org/std/pin/
[vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html#guarantees
[split]: https://doc.rust-lang.org/nomicon/borrow-splitting.html
