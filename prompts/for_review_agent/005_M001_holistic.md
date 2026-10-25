# Review prompt: M001 — Native foundation contract and toolchain admission (round holistic)

Read `AGENTS.md` first. Do not change product files. Use the receipt as execution
evidence; do not rerun verification unless this prompt explicitly says so.

## Objective and acceptance

Judge holistic closure of M001 across its accepted slices.

### M001-S01

- Define inventory, route, observation, outcome and reconciliation contracts.
- Separate capability facts from measured model-effort-difficulty statistics.
- Include tool-call failures, recovery cost, uncertainty and crash recovery.
- Compare at most two non-Python toolchains; keep the final decision explicit.
- Name the canary, full-check and test-discovery gates for later admission.

## Non-goals

### M001-S01

- Product code, dependencies, hooks, services or paid model evaluations.
- Changing canonical decisions, foreground models or harness configuration.

## Read first

### M001-S01

- `00_brief/CONTEXT.md`
- `00_brief/intake.md`
- `00_brief/constraints.md`
- `00_brief/decisions.md`
- `02_analysis/CONTEXT.md`
- `02_analysis/foundation-brief.md`
- `02_analysis/actionable-findings.md`

## Implementation boundary

Allowed prefixes: ### M001-S01

`02_analysis/foundation-contract.md`

Forbidden: ### M001-S01

`00_brief/`, `05_governance/`, `prompts/`, `questions/answered/`, `roadmap.yaml`, `frutlups.toml`, `AGENTS.md`, `CLAUDE.md` and all other paths. These describe the coder's scope;
review remains product-read-only. Read the evidence artifacts named below using
your file-reading tools. Their categories describe volume, not authority.

## Declared verification

Focused:

### M001-S01

```text
python -B scripts/roadmap.py check
```

Full:

### M001-S01

```text
python -B scripts/hermetic_verification.py
```

Execution policy: runtime {}; timeout 120 seconds; observation process.

## Changed files

1 cumulative paths. Complete manifest: `05_governance/reviews/m001/M001_10b7451c1a3d68db_paths.json` (sha256 `10b7451c1a3d68dbe17d3a641e88650d49a9af76dd34be0c564f04d6eb32bf46`; read as a file, starting at line 1)

## Code diff

Bounded diff page: `05_governance/reviews/m001/M001_609cc9c9a38d3d94_diff.md` (sha256 `609cc9c9a38d3d94d1f8a467ccc1cba45a8534706a137b82f2d923086da1427a`; read as a file, starting at line 1)

```diff
git diff 03ce615371c285eda8a040ae0a5df2ea0013f3f6..HEAD --stat (selected paths)

No committed changes in this selected range. Ledger-only accepted work may remain in the working tree; the bounded per-slice evidence below includes it.
Per-slice diffs: current working tree against accepted base (bounded; complete manifests remain authoritative).
### M001-S01


Added file: 02_analysis/foundation-contract.md
# RelayForge native foundation contract

**Status: proposed design input for M001-S01.** This contract does not select a
product language, storage engine, dependency, adapter, model, or routing policy.
The core is local and deterministic: ordinary capture, aggregation, and
reconciliation make no model calls. Adapters report what their harness actually
exposes; missing evidence remains unknown.

## Boundary and identity

The core owns versioned records, deterministic eligibility and aggregation,
reconciliation, and a CLI/library boundary. Codex and Claude Code are the first
adapter targets. Each adapter translates native events into the same records
and declares its coverage; an unobserved native event is not a supported
capability. Routing never grants tools, permissions, credentials, or approval
authority, and it does not replace the foreground architect's decomposition or
verification.

Use stable opaque IDs for sessions, tasks, attempts, execution spans, tool
calls, events, recommendations, and reconciliation runs. Every child record
references its parent IDs. Record schema version, creation time, source
harness, and bounded evidence references. Preserve source/revision identifiers
when available. Do not persist secrets, raw private transcripts, or resolved
machine paths. A missing value is represented as unknown with a reason where
known; it is never silently converted to zero or inferred from a recommendation.

## Inventory and routing

Inventory records exact profile, model, model-version, effort, and adapter IDs.
For every capability, record its value, evidence source, observed-at time,
validity scope (profile, harness/configuration, and runtime), and one status:

- **configured** — declared by local/harness configuration;
- **runtime-confirmed** — direct evidence from the current runtime, for the
  exact profile and matching harness/configuration, establishes availability;
- **unsupported** — the adapter or runtime explicitly rejects or declares this
  capability unavailable for that scope; or
- **unknown** — evidence is absent, stale, contradictory, or cannot establish
  availability for that scope.

`configured` describes intent only; it is not runtime evidence. `unsupported`
is an evidenced negative. `unknown` is unresolved evidence, not a negative fact
and not support.

Inventory facts include supported native effort settings, required harness and
tool configuration, dispatch/continuation support, observable usage fields,
available events, and verification/reporting coverage. Capability status is
not a performance estimate. A successful prior task does not prove a capability
and a measured score cannot grant one.

A route request supplies task ID, harness/profile, task type, difficulty and
readiness labels with provenance, explicit required capabilities, constraints,
and candidate IDs. Deterministic eligibility is evaluated before ranking for
each candidate and each required capability:

- `runtime-confirmed` for the exact current profile and matching harness/config
  satisfies the requirement;
- `configured` alone leaves the requirement `unresolved_configured_only` and
  does not qualify the candidate;
- `unknown` leaves it `unresolved_evidence` and does not qualify the candidate;
- `unsupported` yields `excluded_unsupported` for that candidate; and
- a missing required capability, stale/mismatched evidence, or incompatible
  profile yields an explicit unresolved or excluded reason, respectively.

Only candidates with every required capability runtime-confirmed are eligible
for dispatch or ranking. Return the eligible set and a per-candidate list of
required capabilities, evidence status/scope, and exclusion or unresolved
reason. If none qualify, return `no_eligible_candidate` with those reasons and
request fresh evidence or operator resolution; do 

[Embedded text truncated; read the full artifact at `02_analysis/foundation-contract.md` (sha256 `6f9120efce565917f3db5aebe242da2b65b29f0d2a539a5b75f71577566fec2c`).]
```

## Verification receipt

### M001-S01

Verification passed. Complete receipt: `05_governance/reviews/m001/M001-S01_r2_verification.json` (sha256 `fcd936d5535fb68d294051301e2c786e7dc8e43edd1570fc777890ad5c88eea6`)

```json
{"schema":"frutlups.receipt/2","slice":"M001-S01","round":2,"t":"2026-10-25T13:20:59Z","base_commit":"03ce615371c285eda8a040ae0a5df2ea0013f3f6","tree_dirty_before":true,"commands":[{"label":"full","argv":["python","-B","scripts/hermetic_verification.py"],"exit":0,"secs":0.5,"stdout_tail":"roadmap: ok\r\nledger: ok\r\n","stderr_tail":"","timed_out":false}],"tree_clean_after":false,"ok":true,"manifest":{"path":"05_governance/reviews/m001/M001-S01_r2_manifest_ab7f313cd4f13d91.json","sha":"ab7f313cd4f13d915761c29d61a8226e3f87324fbe20ad5de6cb10ac4b50f5b9"},"witness":{"before":"e01c7362a8a42f0ad39fa4bad5d61f5d2da5f9321c7c743337cd2a65bb08c49e","after":"e01c7362a8a42f0ad39fa4bad5d61f5d2da5f9321c7c743337cd2a65bb08c49e","stable":true,"head":"03ce615371c285eda8a040ae0a5df2ea0013f3f6","index":"ee0007d1dd6df738bf5169d5b721041bace5fffb0930efcccdbd8485323a253e","product":"e670ba99a19c4aaa270a63aa4808890d1cdbafa50b198b28c5127c376e768802"},"runtime":{"python":"3.12.10","implementation":"CPython","source":"invoking_python","sha":"964fbaadf28592f4f81be6e9bccfdd48722d3d3c188e1d71d4d5dd72e0c2f8a1"},"observation":"process"}
```

Review: `05_governance/reviews/m001/M001-S01_r2_review.md` (sha256 `8b90242841c7c65be5a057e25745ea47bf583f437ff87e51b8e28ed620d853f2`)

## Prior findings

Source review: `05_governance/reviews/m001/M001-S01_r1_review.md` (sha256 `0fa47e4b087ad0fedf684fcc2ddec5cb172fcd2c17ed1e81f323a4722821692b`)
| M001-S01-01 | P2 | open | Inventory combines “unsupported” and “unknown,” and routing does not specify how unresolved or merely configured capabilities affect eligibility. Distinguish these states and define safe eligibility behavior so an unverified capability cannot qualify a profile by implication. |

## Output

Autonomous seats return the complete report for the runner to save. In manual
mode, write only `05_governance/reviews/m001/M001_holistic_review.md` when that tool is granted, or return it for
the architect to save.

Use this exact contract:

```markdown
# Review: M001 round holistic

## Findings
| id | severity | disposition | summary |
| --- | --- | --- | --- |

## Closure Decision
Objective status: achieved | not_achieved | indeterminate
Objective evidence: one sentence tied to acceptance and the receipt

## Verdict
Verdict: pass|needs_work|blocked - next: one move
```

Return the report as the plain text of your final message, in exactly this shape, not enclosed in a code fence. Use one allowed value on each choice line. A pass requires zero open P0-P2.
Every P0-P2 finding ID must start with the affected slice ID, for example `M001-S02-H1-F1`.

To change an older finding, add `## Finding updates` before Closure Decision with columns `source | sha | id | disposition | related`. Name the original report path, its SHA-256, exact finding ID and explicit disposition; related is linked IDs or `-`. An unrelated pass closes nothing. Only a human may waive findings.
