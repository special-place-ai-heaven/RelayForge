# M005-S02 accounting results

The core stores an immutable task-context record before the first attempt and
reports physical attempts, parent repairs, explicit acceptance, failures, and
usage/cost/elapsed accounting. The CLI exposes the same read-only reports in a
fresh process:

```text
relayforge context-import --db run.db --input task-context.json
relayforge accounting --db run.db [--task TASK_ID]
relayforge compare --db run.db --input comparison-request.json
```

`task-context.json` is a version-1 `TaskContext`: it names the task and session,
the inventory snapshot, initial profile and revision, policy, observation time,
and task-type, difficulty, and readiness labels. Each label carries provenance
or an explicit unknown state. Import is idempotent for identical content and
rejects changed content or a first import after an attempt has begun. Existing
tasks without context remain unclassified; database and event ordering do not
imply their age or cohort.

The accounting report preserves every observed attempt and failure. A physical
attempt with multiple legacy `UsageCost` rows is ambiguous because the event
schema does not identify deltas or snapshots. Totals retain known subtotals by
unit and mark the full chain unknown for missing, ambiguous, mixed-unit, or
overflowing components. Acceptance is taken from explicit `TaskAccepted`
events; a later work attempt can make current status unresolved while retaining
historical acceptance.

Comparison requests name a fixed baseline, evidence window, and existing S01
eligibility request. Comparisons use exact known label provenance, policy,
harness, adapter, and configuration cohorts, and profile identity dimensions.
The Wilson 95% intervals and exact same-unit cost-per-accepted-task comparison
are descriptive only. The library comparison requires caller-supplied current
state for positive eligibility. The standalone CLI has no trusted runtime
binding and reports `binding_unavailable`; it cannot select a runtime or promote
a profile. Event scans are capped at 10,000 records and fail rather than
truncate.

Focused tests cover context replay and late-import rejection, repair-chain
accounting, ambiguous/mixed/overflow values, exact eligibility and cohort
comparison, uncertainty, and fresh-process CLI paths. These comparison fixtures
are synthetic; they do not establish native adapter behavior or production
calibration.
