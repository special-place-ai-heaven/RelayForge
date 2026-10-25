# RelayForge foundation brief

This bounded brief prepares the foundation-contract slice. It summarizes the
owner's requirements and dated evidence; it does not select a product language
or claim an implementation. Read [the decision register](../00_brief/decisions.md)
for accepted decisions and [the roadmap](../roadmap.yaml) for admitted work.

## Outcome to optimize

Select an eligible model, native thinking effort, and worker profile that
completes the task reliably at the lowest measured total cost. Count parent
reasoning, worker attempts, tools, verification, failed recovery, repair, and
handoff. A higher-priced model can cost less when it avoids repeated failures.
Report completion rate alongside cost per accepted result and latency.

Compare exact model/effort/harness/instruction/tool/skill configurations.
Difficulty labels retain their initial estimate and provenance. Public
benchmarks and advertised prices support discovery but cannot certify local
coding performance. The Astra-effort versus Sol-max crossover is **unknown**;
no universal ordering or 100% reliability is established.

## Required boundaries

- An embeddable core and CLI serve Codex and Claude adapters first. Other
  harnesses, skill routing, MCP/tool narrowing, and execution gateways are later
  candidates whose added value must be measured.
- The foreground agent decomposes work and verifies completion. JEV answers
  bounded questions over registered eligible IDs; it does not generate code,
  invent profiles, or grant authority.
- Separate capability facts, observed outcomes, and routing policy. Observe the
  actual model and effort where exposed; missing runtime identity stays unknown.
- Record sessions, tasks, attempts, execution spans, tool outcomes, costs, and
  recovery links. Deliverable acceptance and operational failure are separate:
  eventual success does not erase earlier wrong calls or wasted spend.
- Several confirmed model-caused wrong tool calls constitute operational
  failure. The proposed threshold of three is unapproved and uncalibrated.
  Infrastructure faults and ambiguous causes retain separate attribution.
- Ordinary local event capture, counters, and consolidation require zero model
  calls. Use incremental records and cached statistics; call JEV only where
  unresolved routing or recovery decisions justify the cost.
- Persist during work. Reconcile at completion and next startup, deduplicate
  events, retain interrupted outcomes as unknown, and protect other active
  sessions from being falsely marked abandoned.
- Adapt rankings only within approved candidates and policy. Do not expand
  permissions, install tools, or change foreground settings through telemetry.
  Keep credentials, raw prompts/code/tool output, and private telemetry out of
  tracked artifacts.

## Observed gaps and unknowns

The 2026-10-25 audit found advisory JEV integrations with inconsistent candidate
families, no validated common model/effort registry, and written lane rules that
conflict with inspected foreground defaults. Claude had native subagents and
a JEV prompt hook; Codex's effective profile registration and dispatch were
unverified. Existing external JEV code contains useful inventory, routing, and
adapter contracts; presence on disk does not prove runtime wiring.

Both harnesses need verified event coverage, actual execution receipts, and
outcome attribution. Calibration, savings, cost accounting, allowed native
bindings, foreground mapping, and promotion thresholds remain unresolved.

## Foundation slice output

Specify the minimum contracts and compare at most two non-Python toolchain
options. Rust is a proposal; the owner has not selected a product language.
Python 3.11+/PyYAML run the retained template workflow only. Storage is also
undecided; SQLite is a candidate. Admit the disposable exact-toolchain canary
only after the relevant decision is recorded. No RelayForge runtime is built.

Use [phase-one spec](phase1-spec.md) for named contracts and R1–R37 acceptance
proposals; use [evidence brief](evidence-brief.md) for source provenance and
historical audit detail. Open relevant sections when needed, not both full
documents at every session start.
