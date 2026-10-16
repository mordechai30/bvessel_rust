# Consolidation coverage record

## Scope and method

Record the 2026-10-15 consolidation into AGENTS.md, rust, and wgpu. This is an audit record, not an
operational prerequisite. Keep old files intact; the prior AGENTS.md is renamed AGENTS_old.md byte-for-byte.
Other skills are outside this task. Existing AGENTS_detailed.md and responsibility maps remain unchanged.

Classify source content by its contract, not by word count or heading preservation. Merge repeated
statements into one owner; omit basic teaching and authoring history. Preserve unique conditions,
exceptions, failure cases, capabilities, numerical rules, operations, and useful executable fixtures.

## Source-to-destination ownership

Interpret Rust reference names below relative to .agents/skills/rust/references and GPU names relative
to .agents/skills/wgpu/references. These are inventory locations, not chains an agent must traverse.

| Input topic | Destination | Disposition |
| --- | --- | --- |
| AGENTS policy and project conventions | AGENTS.md | Retain preferences and mandatory contracts |
| Detailed entry Rust/GPU guidance | Corresponding skill topics | Merge; preserve old file unchanged |
| Root copies of generic Rust/GPU advice | Corresponding skill topics | No dependency on old copies |
| Rust memory-ownership | ownership.md | Retain identity, retention, drop, pinning, COW |
| Rust slices-strings | api-contracts.md; unsafe-ffi.md | Keep range, encoding, record and raw proofs |
| Rust api-correctness; idioms | api-contracts.md | Retain invariants, bounds, serde, macros, dispatch |
| Rust concurrency; async | concurrency.md | Merge execution, supervision, atomic and poll contracts |
| Rust unsafe; ffi-layout; safety | unsafe-ffi.md | Merge complete proof/ABI/foreign-lifetime obligations |
| Rust performance references | performance.md | Merge equivalent-behavior tuning and retention costs |
| Rust toolchain-verification | verification.md | Retain matrix, MSRV, no_std, risk-specific evidence |
| Rust checks and script interfaces | scripts.md | Retain arguments, statuses, outputs, hook context |
| Rust tools | tools.md | Retain task-conditional diagnostic operation and limits |
| Rust C++ comparisons | migration.md | Retain material differences; omit elementary programs |
| Rust modern-rust | unsafe-ffi.md; verification.md | Keep obligations; omit historical capability catalogue |
| Rust review-corrections | Assigned Rust topic | Integrate; no separate correction-reading layer |
| Rust sources and inventories | sources.md | Keep authority/revision limits, not authoring history |
| Rust evaluations | verification.md; concurrency.md | Retain contract cases; no unrun evaluation claims |
| Rust executable examples | Standard and async fixtures | Copy unchanged as optional fixtures |
| Rust checker/hook resources | rust/scripts | Copy four files unchanged; no old-skill dependency |
| GPU architecture and data residency | design.md | Retain workload decisions and resource ownership |
| GPU shader-settings | inputs.md | Retain selection, stage constants, immediate/version rules |
| GPU data-passage | data.md | Retain exact layout, binding, copy, staging contracts |
| GPU pipelines | pipelines.md | Retain compatibility, complete keys, recorded state |
| GPU passes-state | compute.md; pipelines.md | Split scopes/barriers from command-state contracts |
| GPU targets-textures | textures.md | Retain attachment, format, sampling, color, mip contracts |
| GPU half-precision | precision.md | Retain negotiation, encodings, numerical and fallback rules |
| GPU profiling-lifecycle | lifecycle.md; verification.md | Split lifecycle from diagnostics |
| GPU concurrency-safety | lifecycle.md; unsafe.md | Integrate ordering, completion, unchecked obligations |
| GPU source records | sources.md | Retain authority, inspected commits, version distinctions |
| Root GPU contract summary | Assigned GPU topic | Integrate its unique conditions; retain old source file |

## Focused preservation checks

- Check ownership costs, lifetime/identity invalidation, weak references, pinning, and destruction
  dependencies.
- Check Rust public/serde/macro contracts, integer/float/text boundaries, and external caller compatibility.
- Check whole-path admission, async guard scope, partial progress, cancellation, and observed shutdown.
- Check atomic publication/reclamation, ABA, weak CAS, custom wake races, and ordered-stream retention.
- Check unsafe allocation/provenance/initialization, ZSTs, validity under interruption, FFI
  ABI/callback/unwind.
- Check conditional tool operation, package-manager ownership, script interfaces, and target/MSRV evidence.
- Check GPU input selection and complete CPU-to-GPU layout/upload/bind/encode/submit/consume path.
- Check shared-queue batching, pending writes, mapped-slot reuse, stale readbacks, and CPU/GPU completion.
- Check shader scopes, partial groups, eager select, per-step ping-pong, pass/bundle state, and indirect data.
- Check format capabilities, compressed copy/mip rules, color/alpha, MSAA, transient attachments, and HDR
  output.
- Check f16 arithmetic/storage distinctions, variant layouts, precision budget, and fallback negotiation.
- Check unsafe GPU features, versioned Rust signatures, validation, query timing, surface/device recovery.

## Deliberate exclusions

Exclude elementary container/iterator/type lessons, repeated policy, historical source inventories,
creation instructions, fixed compiler mandates, universal performance settings, and repeated teaching
programs.
Keep substantive example contracts in the references and retained fixtures; do not copy every tutorial
snippet.
Do not count intentionally preserved old files as duplication inside the new active instruction set.

## Verification record

- Skill metadata validation: both new skills passed using the existing system Python with PyYAML.
- Copied environment-checker regression suite: 5 tests passed.
- Retained standard-library fixture: 11 tests passed with installed Rust 1.99.0.
- Retained Tokio fixture: 7 tests passed in a temporary package with cached Tokio 1.53.2.
- No new GPU executable examples or application changes were introduced; no GPU execution or benchmark is
  claimed.
- Check all original hashes, direct resource links, anchors, width/size limits, and duplicate operational
  text.
  Report structural checks separately from manual semantic coverage; neither is a complete correctness proof.
