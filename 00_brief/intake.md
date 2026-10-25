# RelayForge project intake

## Problem and users

Coding-agent owners cannot reliably choose models and thinking effort from names,
prices, or public benchmarks alone. Tool mistakes, repeated failed attempts,
recovery, duplicated context, and parent repair can erase apparent savings.

RelayForge is an adaptive routing and performance layer for coding harnesses.
Its users are the owner and the coding agents working in those harnesses.
The desired outcome is reliable accepted work at the lowest measured total cost,
with decisions grounded in observed model-plus-effort performance.

## Scope

- Project horizon: an embeddable core and CLI for capability inventory, bounded
  JEV routing, session observation, adaptive performance records, and recovery.
- Admitted milestone: M001, native foundation contract and toolchain admission.
- Current run boundary: initialize this template and project authority only.
  No product code, installed hooks, live telemetry, or model evaluation is claimed.
- First future slice: M001-S01 writes a bounded foundation contract and compares
  at most two non-Python toolchain options. The architect records any later
  accepted language/toolchain decision; a coder cannot decide it by implication.
- First product-code admission: one disposable canary with the exact accepted
  compiler/runtime, a maintained full check, test discovery, and teardown.
  This is planned work after the foundation decision, not part of initialization.
- Initial harnesses: Codex and Claude Code. OMP and Cursor are deferred adapters.
- Existing material: dated research/configuration findings and phase-one design
  in 02_analysis/, plus the upstream template's structure, scripts, and rules.

## Required behavior

Discover eligible native profiles and supported effort settings. Distinguish
configured capability, runtime-confirmed availability, and measured results.
Compare exact model/version + effort + task difficulty + harness/tool configuration.

Track session, task, attempt, and model/effort spans. Count model-caused wrong tool
calls, operational failures, first-attempt success, eventual acceptance, abandoned
work, retries, recovery, and parent repair. A rescued task retains its earlier
failure history. Distinguish model faults from unavailable MCPs/infrastructure.

Find task-specific quality/cost crossovers, including whether a stronger model
at lower effort can beat another model at maximum effort for less completed-work
cost. The Astra-versus-Sol-max crossover remains unknown, not a fixed mapping.
Track sample size, evidence provenance, uncertainty, and drift.

Keep ordinary collection and consolidation local with zero extra model calls.
Use deterministic routing when evidence resolves the choice; ask JEV for bounded
unresolved choices and recovery recommendations. The foreground architect
decomposes work and verifies results. JEV never grants permissions.

Persist small events during work. Consolidate at completion and session end,
and reconcile unfinished records at the next start without double-counting.
Interrupted work remains interrupted/unknown until evidence establishes outcome.
Routine observations update derived statistics/ranking, not model permissions.

## Operating facts

- Initial development environment: Windows PowerShell. Other environments need
  their own qualification; no portability beyond observed checks is claimed.
- Workflow tooling: the template requires Python 3.11+, PyYAML 6.0.3..<7, and Git.
  Those scripts are development tooling, not RelayForge's product runtime.
- Product language: undecided; Python is excluded by the owner. Rust is a proposal.
- External references: TypeSafe JEV and the existing external Python router are
  research/integration inputs, not copied product dependencies or selected runtime.
- Full initialization verifier: python -B scripts/hermetic_verification.py.
  Its commands validate roadmap, ledger integrity, and Git whitespace. This is
  scaffold verification, not a product build/test or live harness qualification.
- Checks for a future product slice must include actual product dependencies and
  required test discovery before that slice is issued.
- Integration gate: exercise the real selected CLI/library path in fresh
  processes, then verify native Codex/Claude event and dispatch coverage.
  A hook recommendation is not proof that the selected worker ran.
- Integration/docs/test writes: architect admits their exact paths in roadmap;
  coders cannot broaden their own boundary.
- Budgets for this initialization: zero paid model-comparison runs, zero live
  routing experiments, zero service changes, zero publication. No new product
  dependency is selected. Future experiments need explicit operational budgets.
- Scripts and commands must remain bounded; required verification runs once,
  with repeats only for changed inputs or a concrete failure.
- Initial local Git baseline is part of template setup. Pushes, PRs, and remote
  publication are outside this initialization.

## Reading the project

AGENTS.md and initialization/architect.md define entry and roles. Each numbered
workspace's CONTEXT.md is the local entry point. The first slice reads the short
02_analysis/foundation-brief.md; the full research/spec are scoped reference
material, not mandatory context for every task. The owner will request a
fresh-session bootstrap/handoff prompt separately.
