# Honest accounting and fixed-baseline comparisons

M005-S02 advances the remaining G3 statistics/ranking requirements under existing
D009/D015 authority. Use current Rust/SQLite dependencies and S01 eligibility;
no provider calls, dispatch, automatic promotion, new dependency or tool bundles.
This slice supplies useful local reports now; G4 still owns real native capture.

## Pre-execution context and exact cohorts

Existing events contain no observation timestamps or workload context. Preserve
them and report their age/cohort as unknown; never infer either from file times,
selection or eventual outcome. Add a bounded closed version-1 task-context record
in an additive immutable SQLite table. It references an existing task/session and
inventory snapshot, the initial profile/revision, policy version, observation time,
and task-type/difficulty/readiness labels with provenance. Preserve explicit unknown
labels. Snapshot supplies exact harness/adapter/configuration and model/effort
identity. Import must occur before the task's first attempt; a late import is
rejected, preventing outcome-derived labels from being retrofitted for ranking.
Context replay is idempotent and same-key conflict is atomic/no-write.

Group comparisons only by exact task labels/provenance, policy, harness/adapter,
configuration and evidence window; keep profile revision/model/version/effort as
the candidate dimension. Do not merge aliases, different efforts or unknown
execution identities. A context inconsistent with the first attempt's selection
or known observed identity cannot supply comparable evidence. Retain it in the
report with its reason rather than silently discarding the task. Historical tasks
without context remain visible as unclassified; they cannot establish a crossover.

## Account for every physical attempt

Build read-only reports from durable core events, preserving task/attempt IDs,
predecessors, span roles, selected versus observed identities, latest result and
all previous failure/interruption/tool-error occurrences. Count parent repair,
retry, verification, tool and recovery attempts in the same logical task chain.
Acceptance requires the explicit stored TaskAccepted evidence, not worker success.
Report distinct accepted tasks, all attempted tasks, unresolved outcomes, NotRun,
operational failures and missing accounting separately. An earlier acceptance
event does not hide subsequently unresolved work; expose both historical acceptance
and current unresolved/inconsistent status and exclude it from comparisons.

Legacy UsageCost has no delta/cumulative marker. Exactly one record per attempt
can supply its known components; multiple distinct records are ambiguous and
must not be blindly summed or selected by recency. Preserve those observations,
mark the attempt ambiguous and exclude its values from the trusted subtotal.
Missing records/components and unknown identity remain explicit. For each dimension
(usage, cost, elapsed), sum only unambiguous known values using checked arithmetic,
grouped by exact unit. Label that value known_subtotal. A full-chain total is known
only when every physical attempt supplies one known component in the same unit.
Unknown, ambiguous, mixed-unit or overflowing totals remain unknown with reasons;
never coerce to zero, wrap integers, convert currencies or discard failed attempts.
Cost per accepted task includes the entire cohort's failed/repaired task costs;
zero accepted tasks makes that ratio undefined, not zero. Report missingness counts.

Reads must cover the requested population or return an explicit limit error;
never silently truncate to the existing show limit. Bound inputs, scan volume
and output (a declared 10,000-event cap is sufficient now). Deterministic ordering
and replay must produce the same accounting for the same events/query.

## Conservative comparison policy

Provide descriptive-v1 comparison against a caller-named fixed baseline profile,
identified in every result. First call S01 eligibility with separately supplied
current state; no statistical score overrides it. Restrict an explicit selection
without substitution. If no candidate qualifies, return no_eligible_candidate.
Unknown labels, unavailable baseline, incomplete cohorts/accounting, unknown or
mismatched execution identity and missing observation age prevent a performance
winner. Return eligible candidates and insufficient_evidence with reasons.

For complete matched cohorts, report sample counts, observation start/end and
as-of time, accepted-task proportion and its Wilson 95% interval (z =
1.959963984540054), complete per-unit cost per accepted task, operational-failure
counts and the fixed baseline comparison. This interval is descriptive under an
independent-task assumption, not calibrated routing confidence or causal evidence.
Do not introduce an arbitrary minimum-sample or automatic-promotion threshold.
Report an observed preferred alternative only if its accepted-rate lower bound
exceeds the baseline's upper bound and its complete cost per accepted task in the
same unit is no higher. Otherwise return insufficient_evidence, including overlaps.
Sort qualifying alternatives deterministically by observed cost per accepted task,
then profile ID; disclose the policy and observed basis. This is advisory output,
never adoption or a claim that a model will perform better on future tasks.

## CLI, tests and limits

Add documented fresh-process CLI paths to import context, report stored accounting,
and request a comparison using bounded JSON. Standalone comparison must preserve
S01's unavailable host-binding result; library tests exercise the independently
bound positive path. Do not pretend fixtures are native observations. Document
actual syntax and output examples plus unknown/ambiguous/mixed-unit behavior.

Required tests: retry and parent-repair full cost; success without acceptance;
failed tool despite eventual acceptance; missing and ambiguous accounting; mixed
units and checked overflow; unresolved and NotRun attempts; context replay/conflict,
late-label rejection and unclassified legacy data; exact cohort/identity separation;
fixed baseline, overlapping/empty/sparse uncertainty, missing eligibility and
explicit-selection behavior; one sufficiently separated synthetic comparison with
its provenance disclosed. Run actual CLI context/report/comparison processes and
retain all prior recovery/privacy/eligibility tests. Register new tests in the
verifier's required discovery set. Prefer a focused accounting module over adding
another large subsystem to lib.rs. Record observed results in accounting-results.md.
