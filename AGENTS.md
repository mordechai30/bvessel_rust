# Project instructions

## Rust

- Prefer explicit validation, named intermediate values, and straightforward control flow over shorter
  idioms. Accept extra complexity only for a material requirement or measured benefit.
- Prefer suitable safe APIs. Permit necessary sound FFI and narrow reviewed unsafe abstractions when safe
  alternatives materially complicate code or dependencies. Do not reject a dependency for internal unsafe.
- Reject style-only lint rewrites that obscure ownership, errors, or state transitions.
- Apply the project's rustfmt/style edition. Use the Style Guide's [indentation and width][width],
  [comment width][comments], and [doc-comment placement][docs] for default formatting.
- Use the `rust` skill for Rust planning, implementation, review, concurrency, unsafe, and CPU performance.

## wgpu and WGSL

- Use the `wgpu` skill for WebGPU through Rust wgpu/WGSL, including GPU design and host/shader contracts.
- Use both skills for mixed host/shader work; retain the Rust ownership contract at GPU handoff boundaries.

[width]: https://doc.rust-lang.org/style-guide/#indentation-and-line-width
[comments]: https://doc.rust-lang.org/style-guide/#comments
[docs]: https://doc.rust-lang.org/style-guide/#doc-comments
