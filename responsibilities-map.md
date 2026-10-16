# Responsibilities map: six-document proposal

## Purpose and status

Describe the preceding proposal in detail. Use it to compare responsibilities before consolidation.
This is a review record, not an instruction to implement the six-document structure.
The requested replacement has one AGENTS.md, one Rust skill, and one wgpu skill.

Cover the six reviewed entry files and their supporting references. Do not treat this map as authority
for changing other skills. Do not rename, remove, or migrate existing files as part of creating the maps.

Assume capable, well-informed agents. Retain user preferences, project constraints, subtle contracts,
version-sensitive operations, and verification procedures. Remove basic teaching and repeated rules.

## Entry files and their distinct jobs

Use these labels in the table:

- **AGENTS:** `AGENTS.md`.
- **Detailed:** `AGENTS_detailed.md`.
- **Rust:** `.agents/skills/rust-full/SKILL.md`.
- **Tools:** `.agents/skills/rust-instructions/SKILL.md`.
- **GPU design:** `.agents/skills/WebGpuFull/SKILL.md`.
- **wgpu:** `.agents/skills/w-gpu-instructions/SKILL.md`.

| Entry | Responsibility | Decisions or actions | Boundary |
| --- | --- | --- | --- |
| AGENTS | Mandatory project policy | Enforce safety, clarity, approvals, and compatibility | No tutorials |
| AGENTS | Task routing | Select relevant Rust/GPU guidance | No reference chains |
| AGENTS | Project constraints | Preserve pins, targets, runtime, and public contracts | No tool catalogue |
| Detailed | Project architecture | Record subsystem ownership and data flow | No generic architecture |
| Detailed | Project lifecycle | Define project-specific startup, recovery, and shutdown | No API handbook |
| Detailed | Product matrix | Record supported project features, targets, and checks | No invented support |
| Detailed | Project procedures | Explain nontrivial repository-specific operations | No repeated policy |
| Rust | Ownership boundaries | Choose ownership, borrows, IDs, and snapshots | No basic move lesson |
| Rust | Concurrency semantics | Define admission, lock scope, cancellation, and completion | No tool setup |
| Rust | Unsafe abstractions | Prove validity, aliasing, and safe-client invariants | No GPU contracts |
| Rust | Public API contracts | Keep lifetimes, bounds, errors, and dispatch contracts | No Cargo tutorial |
| Rust | C++ migration | Explain material semantic differences with focused examples | Only when relevant |
| Tools | Environment inventory | Inspect tools, pins, declared/resolved crates, and status | No auto-update |
| Tools | Build and checks | Select supported build, lint, test, and audit checks | Preserve workflows |
| Tools | Diagnostic operation | Configure relevant CPU, heap, async, and timing tools | No mandatory suite |
| Tools | Script interfaces | Specify inputs, outputs, execution context, and failures | No script reading |
| GPU design | Work placement | Choose CPU/GPU division and persistent GPU data | No Rust API recipes |
| GPU design | Parameter strategy | Compare uniforms, storage, instances, immediates | No layout tables |
| GPU design | Scheduling | Choose passes, batching, reuse, bundles, and indirect work | No method catalogue |
| GPU design | Precision strategy | Compare storage, arithmetic, quality, and bandwidth | No encoding recipe |
| GPU design | Performance design | Test workload hypotheses at equivalent quality | No universal ranking |
| wgpu | Host/shader ABI | Validate offsets, strides, ranges, usages, and encoded bytes | No Rust basics |
| wgpu | Upload and queue order | Bind command data to intended batches | No snapshots by guess |
| wgpu | Compute validity | Check scopes, bounds, barriers, dispatches, and ping-pong | No design essay |
| wgpu | Completion and recovery | Map, reclaim, reject stale data, recover | No frame-count proof |
| wgpu | Unsafe GPU paths | Review unchecked shaders, HAL, experiments | No generic FFI lesson |

Keep Detailed short if little unique project-specific information remains. Do not add general teaching
merely to justify its existence. These roles distinguish the preceding proposal; they do not describe
how cleanly the current files already follow it.

## Semantic guidance versus tool operation

Assign the meaning of a contract to Rust and the operation of a checking tool to Tools.

- Explain task detachment, cancellation safety, partial progress, and observed termination in Rust.
- Specify test commands, supported configurations, diagnostics, and evidence limits in Tools.
- Explain when an async guard must span suspension in Rust. Include lock order, reentry, and cancellation.
- Specify relevant async diagnostic setup in Tools without repeating the lock explanation.
- Define unsafe proof obligations in Rust. Explain how to run supported Miri, sanitizer, or model checks in
  Tools.
- Preserve existing runtimes and pools. Do not turn a tool list into required dependencies.

## GPU design versus implementation

Assign alternative selection to GPU design and exact validity requirements to wgpu.

- Compare dynamic uniforms, indexed storage, instance streams, and immediate data in GPU design.
- Specify the chosen path's ABI, alignment, binding ranges, offsets, and transfer ordering in wgpu.
- Compare precision, bandwidth, accumulation, and output quality in GPU design.
- Specify granted features, WGSL declarations, binary16 encoding, and fallback validation in wgpu.
- Choose data residency and scheduling in GPU design. Specify scopes, mapping, and completion in wgpu.
- Explain when pipeline reuse, batching, bundles, or indirect work can help in GPU design.
- Define complete pipeline identity, pass state, and indirect record validity in wgpu.

Use both skills when a task requires design and implementation. Do not make either entry load the other
entry before reaching a reference. Cite primary contracts directly when an external API must be checked.

## Reference ownership in the preceding proposal

| Reference subject | Maintained owner | Retain | Exclude |
| --- | --- | --- | --- |
| Project procedures | Repository `references/` | Verified local constraints | Generic Rust/GPU advice |
| Ownership and retained data | Rust skill | Subtle lifetimes, IDs, retention | Elementary ownership lessons |
| Async and concurrency | Rust skill | Bounds, cancellation, supervision | Runtime installation |
| Unsafe and FFI | Rust skill | Proofs, liveness, aliasing, unwind | GPU-specific unsafe APIs |
| C++ comparisons | Rust skill | Material migration differences | Routine language comparisons |
| Tool inventory and scripts | Tools skill | Interfaces and status meanings | Script implementation copies |
| Check and profiling workflows | Tools skill | Conditional commands and limits | Mandatory tool catalogues |
| GPU architecture choices | GPU design skill | Access patterns and cost tradeoffs | Descriptor recipes |
| GPU performance experiments | GPU design skill | Equal workloads and evidence | Tutorial speedup promises |
| GPU ABI and parameters | wgpu skill | Encodings, ranges, transfers | Generic Rust ownership |
| GPU queue and lifecycle | wgpu skill | Ordering, completion, recovery | Fixed-delay reuse rules |
| Shader and command validity | wgpu skill | Features, bounds, scopes, unsafe | Architectural repetition |

Allow both AGENTS documents to use `/Users/motyzas/src/bvessel_rust/references`.
Keep each skill's resources inside its own directory. Do not create shared skill folders or cross-skill
reference links. Place resources directly in `references/`, `examples/`, `scripts/`, or `assets/` below
an entry file. Link required resources directly from that entry file.

Do not keep identical references solely because two entries discuss related topics. Give each contract
one owner. Repeat primary-source URLs when useful; do not copy explanatory bodies with them.

## Removal and preservation rules

- Integrate unique corrections into their topic references; remove separate correction layers afterwards.
- Remove root copies of generic Rust skill material after redirecting its responsibilities to the proper
  skill.
- Keep GPU implementation contracts out of GPU design references after preserving them under wgpu.
- Remove standard collection descriptions, elementary iterator lessons, and routine move/borrow examples.
- Preserve examples that demonstrate a subtle failure, a proof boundary, or a material migration difference.
- Preserve exact script interfaces and diagnostic limitations; these are operational requirements.
- Remove completed authoring instructions, repeated policy paragraphs, and revision narratives from normal
  loading.
- Retain concise provenance where it explains a disputed contract or version-dependent example.
- Do not interpret the earlier requirement to preserve every heading as a reason to preserve duplication.

## Review criteria

Check that every document has a distinct task and every difficult contract has one maintained explanation.
Keep essential user policy in AGENTS. Keep only a short selector or warning in an entry when the full
contract belongs in a direct reference. Avoid forwarding documents and multi-entry navigation chains.

Check local links, anchors, resource placement, width, entry-file size, and scope. Use structural checks
for this documentation work; do not add unit tests that merely mirror the text. Validate retained or
changed executable examples separately before making execution claims.

## Decisions

The user replaced the six-entry proposal with the three-entry structure described in
`responsibilities-map-new.md`. Do not implement the superseded distribution.
Before consolidation, confirm the disposition of other Rust/GPU skills outside the six reviewed entries.

## Concrete next steps

1. Review the replacement ownership map before authorizing consolidation.
2. Inventory nontrivial source content and assign each item to its replacement owner.
3. Preserve unique contracts, examples, script interfaces, and source evidence before removing duplicates.
4. Validate the replacement structure and coverage before retiring any old entry.
