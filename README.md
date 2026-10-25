# RelayForge

Evidence-driven routing for coding agents.

RelayForge is a planned adaptive routing and performance layer for coding
harnesses. It will measure which combination of model, thinking effort, worker
profile, tools, and skills completes a task reliably at the lowest total cost.
Codex and Claude Code are the first integrations.

The objective is accepted work: failures, wrong tool calls, retries, parent
repair, and handoff cost all matter. A stronger model at lower effort may beat a
cheaper model at maximum effort; RelayForge must establish that from comparable
outcomes rather than infer it from model names or advertised token prices.

**Status:** project scaffold and design. No RelayForge runtime, collector, router,
CLI, or harness integration has been implemented. The selected product language is
Rust 1.99.1, with SQLite storage, under D010. Python is retained for the existing
project workflow scripts. The foundation contract has been accepted; consult
ledger status for current implementation progress.
See [environment setup](ENVIRONMENT.md).

## Intended functionality

These are the product horizon. The [roadmap](roadmap.yaml) controls which work is
actually admitted; the first foundation slice defines contracts and toolchain
admission before implementation.

| Capability | Intended behavior |
|---|---|
| Capability discovery | Inventory native roles, exact model identities, supported effort levels, skills, plugins, MCPs, tools, and permissions; distinguish configured from runtime-confirmed availability. |
| Profile registry | Join versioned execution profiles with sourced model facts, prices, outcome evidence, and explicit unknowns. |
| Task routing | Filter candidates deterministically, then use JEV for bounded choices where uncertainty justifies its overhead. Honor explicit user choices. |
| Delegation support | Map a bounded subtask to an eligible native worker. The foreground orchestrator retains decomposition, review, and acceptance responsibility. |
| Continuous observation | Record compact local session, task, attempt, execution-span, tool-outcome, timing, and usage metadata. |
| Tool competence | Measure invalid calls, unsuitable tools, repeated failures, redundant reads, and effective use of the available code-intelligence and terminal tools. |
| Outcome and recovery accounting | Keep first-attempt completion, eventual acceptance, operational failure, interruption, and recovery costs distinct. |
| Recovery recommendations | Compare changing strategy, effort, or model with another attempt using evidence and bounded retry policies. |
| Adaptive rankings | Update measured performance within approved candidates, accounting for task difficulty, sample size, uncertainty, and configuration changes. |
| Model/effort comparisons | Identify supported replacements and tradeoffs using full workflow cost, completion rate, quality, and latency. |
| Crash reconciliation | Persist during work and reconcile unfinished records on completion or next startup without double-counting or inventing success. |
| Decision explanations | Link a route to its evidence, actual worker invocation, outcome, and complete recovery chain. |
| Later integrations | Evaluate skill recommendations, tool/MCP narrowing, additional harnesses, and execution gateways after the initial routing loop earns its cost. |

The proposed core is an embedded library with a CLI boundary and separate harness
adapters. Ordinary observation and incremental accounting use local software with
zero additional model calls. JEV is a bounded decision model; generative workers
perform the work. MCP packaging or a persistent service is optional, justified
only by measured needs. Rankings cannot expand permissions or install tools.

## Start a fresh agent session

Open this repository in Codex or Claude Code. The root [AGENTS.md](AGENTS.md)
contains the operating constitution; [CLAUDE.md](CLAUDE.md) provides Claude's
entry point to the same rules.

1. Follow [RelayForge session entry](AGENTS.md#relayforge-session-entry). A broad
   build/continue request without an issued prompt enters as architect through
   [initialization/architect.md](initialization/architect.md). This repository is
   already initialized; a fresh session resumes existing authority and state.
2. Read the canonical brief, roadmap, and [operating loop](docs/operating.md),
   then run the named roadmap/ledger checks and status commands. Resume the
   reported step within the current request's scope.
3. Read the [analysis index](02_analysis/CONTEXT.md),
   [foundation brief](02_analysis/foundation-brief.md), and
   [actionable findings](02_analysis/actionable-findings.md) before planning work.
   Detailed research and specifications are optional lookups for a named question.
4. Preview and issue the template's generated coding or review prompt for
   admitted work, then hand it to the selected agent. Its `Read first` files,
   role, and writable boundary define that agent's task. The architect records
   the handoff, verification, review, and acceptance through the operating loop.

The [release contract](00_brief/release-contract.md) defines completion. D009
authorizes the architect to admit successive slices, make routine decisions,
commit accepted local work and continue through release review. Use ledger status
for the current step. A fresh agent can operate the manual loop within scope;
these entry points do not install or qualify an autonomous runner.

The project preserves the folder structure, scripts, and manual `/2` workflow of
[agentic-project-template-v4](https://github.com/jequihua/agentic-project-template-v4).
Follow its [operating guide](docs/operating.md) for prompt issuance, verification,
review, and acceptance. No frutlups installation is needed for manual work;
frutlups 0.3.2 does not support `/2`. Autonomous use requires the compatibility
gates in the [upgrade guide](docs/upgrading.md).

## Project references

For a fresh session, ask: "Read AGENTS.md and
prompts/GOAL_RELAYFORGE_BOOTSTRAP.md and 00_brief/release-contract.md. Pursue the
full release campaign, resume from the actual ledger state, and continue after
each accepted milestone until the release gates are satisfied."
Use the harness's native goal mechanism if available; mentioning a file alone
does not activate it. The [workflow graph](docs/workflow-graph.md) maps the
existing transitions and identifies which optional controls are inactive.

| Question | Read |
|---|---|
| What are we building and why? | [Project intake](00_brief/intake.md) |
| What are the hard limits? | [Constraints](00_brief/constraints.md) |
| What has been accepted? | [Decision register](00_brief/decisions.md) |
| What work is admitted next? | [Roadmap](roadmap.yaml) and ledger status |
| What does the next foundation slice need? | [Analysis context](02_analysis/CONTEXT.md) and [foundation brief](02_analysis/foundation-brief.md) |
| What are the proposed detailed contracts? | [Phase-one specification](02_analysis/phase1-spec.md) |
| What research and audit evidence supports them? | [Evidence brief](02_analysis/evidence-brief.md) |
| How do agents run the project workflow? | [Operating guide](docs/operating.md) and [environment](ENVIRONMENT.md) |

## Workflow checks

After the workflow environment is available, inspect the current state:

```powershell
python scripts/roadmap.py check
python scripts/ledger.py check
python scripts/ledger.py status
```

These are template workflow commands, not a RelayForge product interface. The
authoritative full verification command belongs to `verification.full` in
`roadmap.yaml`; run it through the slice verifier. Validation of this scaffold
does not establish routing quality, savings, or live harness behavior.
The initialization checks cover roadmap validity, ledger validity, and Git diff
whitespace; they do not run product tests.

## Manual slice loop

```powershell
python scripts/prompt.py M001-S01
python scripts/ledger.py coded M001-S01
python scripts/verify.py M001-S01
python scripts/prompt.py M001-S01 --review
python scripts/ledger.py record <review-report>
python scripts/ledger.py accept M001-S01
```

The architect hands the generated coding/review prompts to the chosen agents.
Optional `coded --notes <path>` binds saved notes. Add `--commit` to the original
accept command when a commit is authorized; acceptance otherwise changes only
the ledger. **Before the next slice, inspect and commit the accepted product and
loop evidence yourself when using ledger-only acceptance.** New slices require a
clean starting tree; `--allow-dirty` is deliberate architect attribution.
If a requested commit is interrupted, `ledger.py recover` diagnoses the
pending intent and `recover --execute` performs only its missing Git work.
`prompt.py M001-S01 --preview` renders the actual prompt without writes.
Explicit blockers and recorded resolutions are exceptional boundaries, not extra
steps for every edit. See `docs/operating.md` for their commands and holistic review.

## Sources of truth

- plan and boundaries: [roadmap.yaml](roadmap.yaml)
- decisions: [00_brief/decisions.md](00_brief/decisions.md)
- loop history: `05_governance/ledger.jsonl`
- blockers: `questions/open/`

`python scripts/ledger.py status` prints the current state and next step;
`index` renders the historical table. Do not maintain duplicate state files.

Local venvs, caches, credentials, run output, `project.local.toml` and
`frutlups.local.toml` are ignored. Keep machine paths and secrets out of tracked
evidence. `docs/project_checks.md` covers runtime selection, required test
discovery, fresh-process user paths and optional raw-evidence/research guidance.
