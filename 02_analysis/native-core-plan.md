# Native core and CLI admission

M003 implements G1's build/library/CLI path and G2's durable metadata boundary.
It does not complete packaging, ranking, adapters, providers or evaluation.
The accepted canary remains isolated qualification evidence, not product code.

## Layout and dependencies

Use a root Cargo workspace with `08_pkg/relayforge-core` and
`07_app/relayforge-cli`; explicitly exclude `08_pkg/canary`. Pin Rust 1.99.1,
edition 2024 and the Windows MSVC target. Both crates use D011 metadata and
`publish = false`. Keep the root Cargo.lock tracked. Reuse rusqlite =0.40.0
with bundled SQLite and lock libsqlite3-sys to 0.38.0 (SQLite 3.53.1).
Serde =1.0.228 with derive and serde_json =1.0.154 are authorized for strict
typed JSON; stdlib has no JSON parser and hand-written parsing is unsuitable.
No other direct dependency, ORM, async runtime or service is admitted.
Locked transitive dependencies may be fetched before offline verification.

## Metadata and persistence contract

Use closed version-1 event types with unknown fields/versions rejected, bounded
input size and depth, stable opaque source/event/session/task/attempt/span IDs,
timestamps and explicit parent links. Keep capability inventory outside this
slice. Event types cover session open/close/attributed recovery, task creation,
physical attempt intent, execution identity/result, tool result, usage/cost,
verification and explicit task acceptance. Add a span identifier and role for
worker, parent repair, tool, verification or recovery attribution where needed;
do not build a generic plugin/event framework. Each physical provider attempt
has its own attempt ID, linked to one logical task and any predecessor.

Selected profile/model/effort and observed execution identity are separate.
Unknown identity/usage/cost have an explicit representation and reason; never
coerce them to zero or copy selection into observation. Store component usage,
cost and timing for later known-subtotal/unknown-total aggregation (D015), not a
claimed savings figure. Preserve tool failure category and all prior failures.
There are no prompts, transcripts, command arguments, raw outputs or arbitrary
JSON payload maps. Validate bounded identifiers/relative evidence references;
reject absolute paths, obvious credential-shaped values and sensitive/unknown
fields before persistence. Errors must not echo rejected values. Document that
this is a constrained metadata boundary, not universal secret detection.

Store immutable events transactionally in SQLite, with explicit database and
event schema versions. Reject unsupported/newer schemas without destructive
migration. Deduplicate by `(source_id, event_id)`: identical normalized typed
content is a no-op, changed content is a typed conflict with no writes; another
source may reuse an ID. Validate parent existence and session/task ownership
inside the transaction. A returned intent receipt means its transaction committed
before any caller may dispatch. This slice itself never dispatches a model/tool.

Session ownership is a durable explicit claim, not a guessed heartbeat timeout.
Concurrent claims yield one winner and a typed loser conflict. Another owner
cannot append to or take over a claimed session. A crashed claim remains
unresolved until an explicit attributed recovery event names the expected prior
owner and its reason/evidence. Do not infer that a different process means dead.
Reconciliation leaves active sessions alone, appends deterministic interruption
evidence only for explicitly recoverable sessions, and is repeatable without
duplicate events. A late linked result is new evidence, never rewritten history.
Recovery changes ownership only through the explicit resolution contract.

Tasks declare required check IDs. Completion requires passing evidence for every
required check for the accepted attempt and explicit actor/evidence acceptance;
a stale pass before a later failure is insufficient. Keep acceptance distinct
from check success, operational failures, dispatch outcome and integration.
An accepted task cannot silently start new work under the same task identity.

## CLI and verification

Provide documented `init`, `append`, `show` and `reconcile` commands with an
explicit database path. `append` takes one bounded versioned JSON event from
stdin, with fixed machine-readable JSON success/error responses and nonzero
failure exits. `show` returns bounded metadata and task/session state; require
explicit scope or bounded pagination, never dump an unbounded store. Recovery
uses a typed event through the same boundary. No daemon, global install, hook,
credential lookup, provider call or automatic ownership theft is allowed.

Track a stdlib workflow-only `scripts/product_verify.py` launcher. Reuse the
qualified candidate materializer from canary_verify.py rather than copying it.
The outer full command materializes once; the inner command runs both accepted
canary checks and product checks. Use separate scratch output/database roots,
one compile-heavy suite at a time and the existing 900-second deadline. Focused
product checks may bypass materialization explicitly; authoritative full checks
must include it, preserve source/index/HEAD and retain a compact product snapshot
identity summary in the bounded receipt. Do not rely on local untracked caches
for source or test discovery. Fetch dependencies explicitly, then run offline.

Required tests cover unknown fields/versions and secret-bearing input rejection;
parent/scope mismatch; identical replay and conflicting duplicate; atomic failed
append; durable intent after abrupt child exit; active-session protection and
explicit recovery; two-process ownership race; duplicate reconciliation and late
receipt; missing/failed/stale verification versus explicit acceptance; selected
versus observed identity and missing cost; retained failed attempts and repair
links. Inspect database/WAL and stdout/stderr for synthetic sensitive fixtures.
No raw runtime stores or resolved machine paths enter tracked evidence.

Run formatting, warning-free Clippy, build, nonempty discovered/executed unit,
integration and declared doc tests. Reject missing/skipped/zero required tests.
Exercise the actual built CLI in fresh processes: init, session/task/intent,
restart/recovery, replay, failed tool result, verification and acceptance. An
owned test helper may abort itself at before/after commit boundaries; do not
kill unrelated host processes or introduce product dispatch behavior for tests.
Record portable commands/results/versions and historical receipt references in
02_analysis/native-core-results.md without self-referential snapshot claims.
