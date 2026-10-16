# Responsibilities map: one AGENTS, one Rust skill, one wgpu skill

## Purpose and status

Describe the user's requested replacement structure. Preserve important, nontrivial information while
removing trivial explanations, repeated guidance, and overlapping entry files.
This document specifies responsibilities; creating it does not perform consolidation.

Use exactly these entry files within the reviewed replacement structure:

- `/Users/motyzas/src/bvessel_rust/AGENTS.md` for mandatory project policy.
- `/Users/motyzas/src/bvessel_rust/.agents/skills/rust/SKILL.md` for Rust semantics and tooling.
- `/Users/motyzas/src/bvessel_rust/.agents/skills/wgpu/SKILL.md` for WebGPU through Rust wgpu and WGSL.

Do not retain AGENTS_detailed.md as a second project entry. Preserve its unique, useful content in
repository references or in the appropriate skill. Consolidate the two reviewed Rust skills into
`rust` and the two reviewed GPU skills into `wgpu` after separate implementation approval.

Assume smart, well-informed agents. Preserve information that changes a material decision, establishes
an easily missed contract, expresses the user's preference, or enables a required operation.
Do not remove a difficult contract merely because it can be found somewhere in upstream documentation.

## Resource layout

Keep project references and skill resources separate. Do not create a shared folder for skills.
Use direct child folders so entry files can link straight to the needed material.
The following paths describe proposed locations, not files already created by this map.

```text
bvessel_rust/
  AGENTS.md
  references/                 # Verified project-specific detail
  examples/                   # Project-specific nontrivial examples, if needed
  .agents/
    skills/
      rust/
        SKILL.md
        references/           # Rust contracts, tools, and concise source evidence
        examples/             # Useful Rust tests and migration examples
        scripts/              # Preserved operational tools
        assets/               # Required supporting assets only
      wgpu/
        SKILL.md
        references/           # Rust wgpu/WGSL design and implementation contracts
        examples/             # Useful host/shader examples and validation cases
        scripts/              # Required GPU tooling only, if present
        assets/               # Required shader/data assets only
```

Do not create empty resource folders for appearance. Do not move script dependencies apart from their
required resources. Do not use sibling-skill paths or forwarding references to simulate consolidation.

## Detailed ownership table

Use **Project**, **Rust**, and **wgpu** for the three entry owners below.
Keep detailed bodies in the named owner's resources; keep only task selection and essential policy upfront.

| Area | Owner | Entry-file job | Detail or operation owned by its resources |
| --- | --- | --- | --- |
| User policy | Project | Enforce safety, clarity, approvals | Explain only project-specific exceptions |
| Compatibility | Project | Keep declared project contracts | Record actual pins, targets, runtime, matrix |
| Task selection | Project | Select Rust and/or wgpu by task | Avoid mandatory loading of both skills |
| Project architecture | Project | Identify local constraints | Record subsystem ownership and data flow |
| Project lifecycle | Project | Require actual local behavior | Define local startup, recovery, shutdown |
| Repository procedures | Project | State mandatory local checks | Record local commands and evidence |
| Ownership and lifetimes | Rust | Select ownership contracts | Retention, invalidation, IDs, pinning, RAII |
| APIs and dispatch | Rust | Preserve public contracts | Bounds, errors, conversions, runtime traits |
| Rust concurrency | Rust | Bound and supervise work | Pools, affinity, guards, cancellation, atomics |
| Unsafe Rust and FFI | Rust | Justify narrow boundaries | Validity, aliasing, ownership, unwind, reclaim |
| C++ migration | Rust | Load comparisons only when relevant | Keep material semantic differences |
| Toolchain/dependencies | Rust | Select inventory/checks | Status, compatibility, update policy |
| Rust checks | Rust | Select project-supported checks | Cargo, formatting, lint, tests, audit, MSRV |
| Rust diagnostics | Rust | Select tools by limiting cost | CPU, heap, async, timing procedures and limits |
| Script operation | Rust | Select the script interface | Inputs, outputs, artifacts, failures |
| GPU architecture | wgpu | Define workload and consumers | CPU/GPU division, residency, scheduling |
| GPU inputs | wgpu | Select by lifetime/access pattern | Constants, overrides, uniforms, storage, instances |
| Host/shader ABI | wgpu | Verify the byte contract | Rust layout/serialization, WGSL strides and ranges |
| Upload and copies | wgpu | Validate transfers | Queue writes, staging, texture pitches, dirty ranges |
| Shared queue order | wgpu | Distinguish CPU/GPU completion | Batch ownership, sequencing, cancellation |
| Compute phases | wgpu | Require valid dependencies | Dispatch scopes, barriers, bounds, ping-pong steps |
| Pipelines and state | wgpu | Check reuse compatibility | Layouts, cache keys, bindings, bundles, indirect |
| Textures and attachments | wgpu | Keep contents/quality | Formats, views, MSAA, mips, color and alpha |
| Precision | wgpu | Separate arithmetic from storage | f16 negotiation, encodings, tolerances, fallback |
| Mapping and readback | wgpu | Reuse only after completion | Mapped views, slot budgets, generation checks |
| Recovery | wgpu | Preserve rebuildable ownership | Surface outcomes, resize, loss, device caches |
| Unsafe GPU paths | wgpu | Keep validated defaults | Trusted shaders, HAL, experiments, initialization |
| GPU measurement | wgpu | Measure equivalent workloads | CPU/GPU timing, transfers, frame latency |
| GPU verification | wgpu | Select failure tests | ABI values, multiple steps, delayed/stale callbacks |
| Source evidence | Each owner | Load relevant contracts | Primary links, inspected revisions, review limits |

## AGENTS.md: project policy and project detail

Keep AGENTS.md short and mandatory. Preserve the user's explicit preferences and constraints even when
an agent knows the underlying language. Retain approval, safety, clarity, compatibility, and honest
reporting requirements in one place.

Place verified repository-specific detail in the root `references/` folder. Absorb useful material
from AGENTS_detailed.md there, but classify it first:

- Keep actual project architecture, lifecycle rules, supported configurations, and local procedures here.
- Move generic but nontrivial Rust contracts to the Rust skill.
- Move reusable WebGPU/wgpu contracts to the wgpu skill.
- Remove trivial explanations and exact or semantic duplicates after preserving their unique conditions.

Do not invent project facts to fill an architecture reference. Do not retain C++ comparisons in project
policy unless the repository has a specific migration contract that requires them.

## Rust skill: semantics, implementation decisions, and tool operation

Combine valuable content from rust-full and rust-instructions. Keep this skill independent of wgpu.
Use separate topic references within the same skill to distinguish semantic proofs from tool commands.

Preserve difficult ownership, API, concurrency, unsafe, performance, and migration decisions:

- Preserve retained-borrow, invalidation, generational identity, pinning, and forgotten-guard obligations.
- Preserve Send/Sync distinctions, suspended-state bounds, deliberate async guards, and thread affinity.
- Preserve admission and byte budgets, pending producers, partial progress, cancellation, and supervision.
- Preserve blocking-task termination limits, external-effect retry contracts, publication, and reclamation.
- Preserve unsafe validity, initialization, aliasing, lifetime, FFI, callbacks, and unwind obligations.
- Preserve numerical behavior, panic policy, deployment support, diagnostic symbols, and measured tuning.
- Retain C++ examples only where a material semantic difference helps an actual migration decision.

Preserve useful environment-checker and pre-commit tooling. Keep the exact documented interface,
status interpretation, dependency layout, and installation/update approval rules. State script execution
context and prohibit implementation reading unless explicitly requested. Do not prescribe every profiler,
watcher, pool, runtime, or library for every Rust task.

Place executable examples under this skill's own examples folder when they remain useful. Preserve
requirements, expected outcomes, and verification limits. Do not convert earlier test records into claims
that relocated or changed examples have been tested.

## wgpu skill: WebGPU specifically through Rust

Combine GPU design, Rust API operation, WGSL contracts, debugging, and measurement in one skill.
Use Rust wgpu for host examples. Use WebGPU/WGSL specifications for portable contracts; do not translate
browser JavaScript signatures literally into Rust or require unrelated browser frameworks.

Preserve the complete CPU-to-GPU data path:

- Validate logical counts, checked byte sizes, resource usages, ranges, and writer/reader ownership.
- Choose constants, pipeline overrides, uniforms, indexed storage, instance streams, or immediate data
  from lifetime, access, capability, and measured cost.
- Verify raw byte layouts or shader-aware serialization. Separate Rust byte validity, WGSL layout,
  vertex decoding, binding-offset alignment, and copy/mapping alignment.
- Preserve matrix/array padding, packed formats, flags, logical counts versus capacity, and format decoding.
- Upload distinct command records. Preserve queue-write timing, staging-view lifetime, and ordered copies.
- Establish shared-queue sequencing. Distinguish prepared CPU data, submission, mapping, and GPU completion.
- Keep GPU-produced data resident when its next consumer is a shader or indirect command.

Preserve compute and rendering validity:

- Preserve compute-dispatch versus whole-render-pass scopes, aliasing limits, and automatic transitions.
- Preserve per-step ping-pong advancement, global phase dispatches, neutral inactive lanes, and uniform
  barriers.
- Preserve eager select argument evaluation; use actual control flow to protect out-of-range loads.
- Preserve complete device-bound pipeline identities, binding compatibility, and pass-local state
  invalidation.
- Preserve bundle state effects, exact indirect records, feature gates, and transparent composition order.
- Preserve texture/view ranges, format capabilities, MSAA resolves, attachment lifetime, and mip population.
- Preserve linear/sRGB and alpha contracts, copy pitches, compressed blocks, and required result storage.

Preserve precision, lifetime, unsafe, and recovery detail:

- Separate f16 arithmetic, half-float textures, compact vertices, and packed halves.
- Preserve adapter/device/shader negotiation, variant layouts, binary16 encoding, and f32 fallback behavior.
- Preserve range, subnormal, conversion, accumulation, and repeated-update error considerations.
- Preserve native callback progress versus browser event-loop behavior where Rust targets differ.
- Preserve mapped-slot completion, bounded readback, stale device/resource/request rejection, and view
  lifetimes.
- Distinguish safe-handle dropping, explicit destruction, mapped reuse, and raw HAL ownership.
- Preserve trusted/passthrough shader proofs, initialization requirements, and experimental-feature risks.
- Preserve surface outcomes, zero-size suspension, resize, device loss, and device-generation cache
  invalidation.

Keep architecture alternatives and exact implementation contracts in distinct topic sections, not duplicate
files. Preserve CPU/GPU/transfer/frame measurement distinctions, query alignment and feature requirements,
backend diagnostic limits, comparable output quality, and startup versus steady-state evidence.

## Duplication and progressive disclosure rules

Give each difficult contract one maintained explanation. Integrate correction documents into their topic
owners. Keep a concise provenance section only where its source or revision materially helps verification.

Keep a short risk selector in SKILL.md only when it changes which reference the agent must load.
Do not repeat its full contract elsewhere. Repeat primary-source URLs as needed without copying explanations.
Keep project-specific instances of generic contracts in project references; do not restate the generic rule.

Do not add shared skill folders, cross-skill resource paths, alias skills, or forwarding documents.
Link each required resource directly from its own entry file. Keep scripts and examples independent enough
that normal use does not require reading implementation files or traversing other entry files.

## Preservation and acceptance criteria

Before retiring material, record its source section and destination topic. Classify each item as retained,
merged, trivial, duplicate, or obsolete. For a merge, preserve unique conditions, exceptions, failure modes,
capability restrictions, and useful examples. Do not use file deletion or byte reduction as proof of coverage.

Check every useful nontrivial item against its destination. Preserve script interfaces and required assets
before relocating tools. Identify unresolved source conflicts instead of silently choosing different
semantics.
Distinguish historical review/toolchain metadata from actual project requirements.

Validate names, descriptions, direct links, heading anchors, resource paths, and 110-character source width.
Keep each SKILL.md under 200 lines and 10 KB, and AGENTS.md under 500 lines and 20 KB; prefer much less.
Do not fill these limits. Ensure the wgpu skill retains portable WebGPU and target-specific Rust distinctions.

Use coverage/link/scope checks for the documentation change. Re-run relevant example checks after executable
examples or their dependencies move/change; do not add unit tests that merely copy document wording.
Report tests, GPU execution, and benchmarks only when they actually run.

## Unresolved decisions

- Confirm whether consolidation also retires other Rust/GPU skills outside the four reviewed skill folders.
- Confirm retirement of superseded entries only after the preservation inventory and replacement pass review.

## Concrete next steps

1. Approve the responsibility distribution and resolve the remaining consolidation scope.
2. Inventory unique nontrivial content, examples, scripts, assets, and evidence from the reviewed inputs.
3. Assign every retained item to Project, Rust, or wgpu and review the preservation record.
4. Create the three entries and their directly owned resources under separately approved implementation steps.
5. Verify coverage, links, examples, script interfaces, size, width, and Rust/GPU separation.
6. Retire superseded material only after its retained content and references are verified.
