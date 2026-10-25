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
request fresh evidence or operator resolution; do not dispatch or silently
fall back to a configured profile. JEV, statistics, and prior successful work
cannot override this gate. Once the eligible set is established, the local
ranker uses only matched, versioned observations and reports sample size, age,
uncertainty, and the baseline used. With insufficient performance evidence, it
returns the eligible set and `insufficient_evidence`; it does not invent a
winner.

An optional future JEV question is versioned and bounded to those eligible IDs,
the unresolved choice, relevant aggregate evidence, and a named judge model.
Log the question, candidates, policy version, answer, and later outcome. A
recommendation is advisory. Record adoption separately as `suggested`,
`applied`, `request_sent`, or `provider_confirmed`; record actual model, effort,
and usage only when exposed by a receipt. Pin a selected profile across tool
continuations. Change it only at an explicit decision boundary, with reason and
new recommendation/selection linkage. Shadow recommendations must be measured
before they can affect dispatch.

## Observations and outcomes

An observation stream links:

`session → task → attempt → execution span → tool call / verification / recovery`

Persist small metadata events as work happens, before dispatch where possible.
An attempt records selected and observed execution identities separately,
start/end state, retry/escalation linkage, and evidence provenance. A span
attributes work to the model/effort only when supported by runtime evidence;
parent reasoning, harness work, verification, and repair get their own spans
when attributable. Unknown attribution and usage remain unknown.

Each tool call records requested tool and operation, caller/adapter, timestamp,
result (`success`, `failure`, `not_run`, or `unknown`), and failure category.
Categories distinguish model-caused invalid/wrong calls, permission or policy
rejections, unavailable tools, infrastructure failures, timeouts, and
unresolved causes. Preserve every failed call after recovery. The proposed
three-wrong-call threshold is not approved and is not a contract default.

Task difficulty, readiness, and task type are separate labels, each with value,
source, and confidence/provenance. They must not be relabeled from outcome.
Record deterministic checks (command/version, result, and evidence reference),
human/task acceptance, and downstream integration separately. Technical checks
passing does not imply acceptance or integration.

An outcome retains first-attempt success, eventual acceptance, abandonment,
retry/escalation count, wrong calls, recovery actions, parent repair, and
verification. Attach cost and elapsed time to each attributable span, separating
model, tool, verification, recovery, and parent effort. Unknown cost stays
unknown. Compare completion rate, latency, and total cost per accepted task on
matched tasks; a task with no accepted result is not a saving. Preserve all
earlier failures when a task is rescued.

Maintain two distinct data sets:

1. **Capability facts:** configured/runtime-confirmed support and adapter
   coverage, with evidence and freshness.
2. **Measured performance:** observations grouped by exact model/version,
   effort, difficulty/readiness provenance, harness/tool configuration, and
   policy version. Store sample count, success/failure counts, accepted-task
   cost and latency distributions, missingness, evidence links, and uncertainty
   (including interval/method and data window when computed).

These statistics are not capability facts. Sparse, stale, mismatched, or
selection-biased samples are marked accordingly; benchmark priors are not
presented as RelayForge measurements. Drift triggers a visible stale/uncertain
status and review, not an automatic capability change.

## Durable events and reconciliation

Before dispatch, persist an idempotent event that records the intent and
attempt ID. Persist dispatch result and each observation under stable event IDs;
deduplication uses event ID plus source identity, not approximate timestamps.
Record deterministic verification evidence before marking a task complete.
Completion requires both the required check evidence and an explicit acceptance
outcome; integration remains a separate field.

At session end, consolidate derived statistics from durable events. At process
start, reconciliation scans only records not already closed, and is safe to
repeat. It deduplicates, links late receipts, and reports unresolved dispatches
as `interrupted` with outcome/identity/cost `unknown` unless evidence resolves
them. It must not declare every interruption a failure, double-count events,
rewrite accepted evidence, or take ownership of another active session. An
operator can inspect and resolve ambiguous records with a new attributed event;
the prior event remains immutable.

The storage format and transaction mechanism remain open. Any implementation
must demonstrate durable-before-dispatch ordering, atomic event append or
transaction, idempotent replay, and recovery after abrupt process termination.
Collection, aggregation, and reconciliation remain local and make zero model
calls.

## Toolchain candidates (not a selection)

| Candidate | Useful fit to prove | Cost or uncertainty to resolve |
|---|---|---|
| **Rust** (current proposal) | Native compiled core/CLI; strong type boundaries and memory safety; Cargo offers a unified build/test workflow. | Owner has not approved the language; learning and compile iteration cost, Windows packaging, adapter integration, and required dependencies need a real canary. |
| **Go** | Native compiled core/CLI with a comparatively direct build/test workflow; worth comparing for Windows delivery and adapter work. | Owner has not requested or selected it; runtime/GC and dependency/adapter fit, packaging, and maintenance cost need the same canary evidence. |

No benchmark, measured productivity comparison, or user decision is available
here. Rust remains a proposal, Go is a comparison candidate, and storage
(including SQLite versus a simpler append-only local format) also remains
undecided. The architect must record the owner's explicit language/toolchain
and storage choice, or the options and required evidence, before product-code
admission. This document itself admits no code or dependency.

## Proposed next admission gates

These are concrete proposed gates for the architect to admit after the owner
decides the exact toolchain and storage. They are not authorization to run the
canary in this docs slice.

1. **Canary:** `M001-CANARY-01 — native event durability and recovery`. In a
   disposable exact-toolchain workspace, build a minimal library plus CLI that
   writes a session, task, pre-dispatch attempt, tool failure, later verification
   and acceptance; terminate it between dispatch intent and result; restart and
   reconcile twice. Show stable IDs, preserved failure history, unknown
   interrupted outcome before evidence, no duplicate events on the second
   reconcile, and distinct capability versus measurement records. Keep adapter
   fixtures synthetic; make no live routing or model calls. Record exact compiler,
   runtime, dependency versions, commands, elapsed build/test time, and recovery
   results. Delete the disposable workspace after recording the result.
2. **Full check:** the admitted slice names an exact, repeatable command using
   locked/pinned toolchain and dependencies. It must run formatting/lint or
   equivalent, compile/build all targets, unit and integration tests, and any
   schema/fixture validation from a clean checkout. The command exits nonzero on
   any failure and runs without network after dependencies are provisioned.
   Record the observed command and toolchain receipt; scaffold verification is
   not a product full check.
3. **Test discovery:** before issuing the canary, name the toolchain's command
   that lists/discovers tests and demonstrate it finds every intended unit and
   integration test in a clean checkout. The full-check command must execute
   that discovered set. Record discovered and executed counts; an empty or
   skipped suite fails the gate.

Only after all three gates pass with the owner-approved exact toolchain,
dependencies, and bounded write paths should the architect admit native adapter
work. Real Codex/Claude event and dispatch coverage remains a later integration
gate using fresh processes and observed native receipts; this synthetic canary
cannot establish it.
