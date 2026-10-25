# Inventory and deterministic eligibility

M005-S01 advances G3 using the existing Rust library, CLI and SQLite store.
Statistics/ranking follow in a separate admitted slice; G4 supplies live harness
observations. No new dependency, provider call, dispatch, environment mutation,
tool prerequisite bundle, or packaging is admitted.

## Bounded inventory

Add version-1, closed JSON types and immutable stored registry snapshots.
Reuse the current metadata validation and error/privacy conventions. Bound each
document to 1 MiB, at most 64 profiles and 64 capability facts per profile;
bound all strings and collections. Reject unknown fields, versions, duplicate
profile/capability keys, invalid timestamps, raw/sensitive metadata and malformed
references before writes. Errors must not echo rejected values.

A snapshot identifies its revision, policy version, creation time, harness and
version, adapter and version, runtime/session scope, configuration fingerprint
and relevant workspace-state fingerprint. These are declared metadata, not
self-authenticating proof. Profiles identify exact profile/revision, native role,
model provider, configured model ID, resolved model ID/version (or explicitly
unknown), effort and supported efforts. Never collapse aliases or copy selected
identity into an unknown resolved identity. Include bounded tool/permission and
skill-version facts as capability requirements, without implementing tool bundles.

Each capability has an opaque ID and exact value, one of configured,
runtime_confirmed, unsupported or unknown, a bounded source/evidence reference,
observation/expiry times, and exact profile/revision, harness/configuration and
runtime scope. Source references identify evidence, not raw payloads. Reuse
safe relative or opaque references rather than accepting arbitrary URLs/paths.
Capability facts are not performance estimates. Conflicting duplicate facts are
invalid input, not last-write-wins evidence.

Store validated snapshots transactionally alongside existing events, preserving
event schema behavior and existing databases. Same snapshot ID and identical
typed contents is an idempotent no-op; changed contents for that ID is a conflict
with no writes. Changed relevant configuration needs a new snapshot ID. Read by
explicit ID; never silently select whichever snapshot was most recently loaded.

## Eligibility and current-state boundary

A closed version-1 request identifies request/task/session, snapshot and policy,
candidate profile IDs, current execution path, optional explicit user-selected
profile, required capability ID/value pairs, and workload labels with provenance.
Unknown labels remain explicit. Evaluator outcomes cannot supply routing labels.

The library evaluator receives a snapshot, request and a separate current-state
argument from its trusted host integration. Current state includes time, expected
snapshot/policy, harness/adapter versions, runtime/session scope, configuration
and relevant workspace fingerprint including dirty state. It is not deserialized
from the request or snapshot; do not implement a JSON 'trusted=true' escape hatch.
The library contract must explicitly say that API callers own trustworthy capture;
type names and matching digests are not authentication. G4 adapters must derive
these values independently and observe the runtime before supplying them.

Compare that independently supplied expectation with the snapshot and capability
scopes. Any stale snapshot/configuration, dirty-workspace mismatch, future or
expired evidence, unknown resolved execution identity or unsupported effort
prevents eligibility with an explicit reason. Missing trusted current state also
prevents eligibility. Validity uses the host-supplied time, not an untrusted request
clock. A capability meets a requirement only when fresh runtime_confirmed evidence
matches its exact current scope and requested value. Configured-only yields
unresolved_configured_only; absent/unknown/stale evidence yields explicit
unresolved reasons; unsupported yields excluded_unsupported. No model score or
successful prior task overrides these rules.

Return deterministic, bounded per-candidate reasons/evidence and an eligible set,
or no_eligible_candidate. Preserve input-independent stable profile ordering.
An explicit selection restricts the result to that profile; if it fails, return
its reason without substituting another. Unknown selected/candidate IDs are
explicit errors/unresolved outcomes, never aliases to existing profiles.
No ranking, recommendation adoption or dispatch occurs in this slice.

## Useful CLI and honest limitations

Extend the existing explicit-path CLI with inventory import/show and eligibility
diagnostics (use syntax consistent with the existing parser):

    relayforge inventory-import --db <db> --input <snapshot.json>
    relayforge inventory-show --db <db> --snapshot <id>
    relayforge eligibility --db <db> --snapshot <id> --input <request.json>

Actual parser syntax must be documented and exercised in fresh processes. Snapshot
import/show is usable now. Until G4 supplies native host capture, the standalone
eligibility command reports binding_unavailable and no eligible candidate; it
must not promote file assertions to trusted runtime observations. Library tests
exercise independently supplied current state and the positive eligible path.
This explicit interim limitation is not completion of G3/G4 or a substitute for
the real adapter smoke tests required by the release contract.

## Evidence

Add meaningful discovered unit/integration tests for immutable snapshot replay
and conflict/no-write; reopening the DB; schema/size/privacy rejection; exact
identity and effort; all four capability statuses; expiry/future timestamps;
wrong scope, missing state, snapshot/configuration/dirty-state mismatches;
explicit selection/no substitution; stable ordering and no eligible candidate.
Exercise import/show/eligibility through the built CLI in separate processes,
including its honest unavailable-binding result. Preserve all existing recovery,
ownership, privacy and acceptance tests. Add the required new test names to the
existing verifier discovery gates rather than permitting an empty test target.
Run focused checks while implementing and one architect-owned full verification.
Record observed results and limitations in inventory-results.md; claim no live
runtime confirmation, ranking effectiveness or release completion.
