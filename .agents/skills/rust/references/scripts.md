# Bundled script interfaces

Resolve paths below from the skill root. Run all scripts outside sandbox restrictions; do not read
implementations unless explicitly asked. Preserve project pins and obtain authorization for installs/updates.

## Environment inventory

Run bash scripts/check-rust-environment.sh from the skill root. It delegates to the adjacent Python helper.
Use Python 3.11+; set PYTHON to the intended interpreter. If an interpreter/module fails, check system,
Homebrew, and configured virtual environments before declaring the tool unavailable.

- Omit project selection for machine-only inventory.
- Use --project /absolute/project/path or --manifest-path /absolute/path/Cargo.toml for project inventory.
  Select one intended project; do not combine selectors for different projects.
- Add --online for stable Rust and non-yanked stable registry release queries; offline status can be unknown.
- Add --json for structured stdout instead of the console report.
- Use --timeout 10 for a per-command/request deadline in seconds, not an overall inventory deadline.
- Expect exit 0 for completed inventory, 1 for missing/broken required tools, and 2 for invalid arguments.
- Inspect every result: exit 0 does not mean optional tools exist or all versions are current.
- Read stdout for the report and stderr for failed probes. If Python is unavailable, the Bash prerequisite
  report is not a completed full inventory. Expect no installs, lockfile edits, compilation, or project
  artifacts.
- Distinguish MISSING, BROKEN, OUTDATED, NEWER_AVAILABLE, UNKNOWN, NOT_DECLARED, LOCKED, and DECLARED.
  Treat newer availability as information, not a compatible required update; declared/locked is not
  built/active.
- Separate executable tools from library dependencies and declared constraints from lockfile resolution.
  Do not propose crates.io updates for git/path/alternate-registry dependencies or install libraries globally.
- Keep rustfmt/Clippy/cargo-fix aligned with the selected compiler; do not compare their versions to library
  crate releases. Preserve the package manager owning an executable and offer compatible exact commands.
- After an authorized update, rerun inventory and affected supported tests, Clippy, and audit. Check actual
  builds for linker/target readiness and a diagnostic run for profiler readiness; presence is insufficient.

## Pre-commit check

Resolve scripts/pre-commit-rust.sh from the skill root, then invoke its absolute path with bash from the
Cargo workspace root, without arguments. Integrate into the existing hook manager rather than replacing hooks.
Expect workspace rustfmt/Clippy diagnostics, nonzero failure status, and normal Cargo cache/build artifacts.
Do not expect source rewrites or automatic staging. Use supported wrapper configurations for feature/target
matrices; retain expanded CI/MSRV/security checks. Do not run profilers, watchers, benchmarks, or fixes per
commit.

## Script regression tests

Use python3 scripts/test_environment.py from the skill root when verifying the copied checker, with a
Python 3.11+ interpreter. Expect unittest results and a nonzero exit on failure; tests use temporary fixtures.
Keep check_rust_environment.py adjacent to its shell wrapper and tests. These files are implementation
resources, not agent reading prerequisites. Do not silently claim a checker test covers all live tool
backends.
