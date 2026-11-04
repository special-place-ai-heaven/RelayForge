# Native identity and Codex observation

M007 advances G4 in two ordered slices using the existing Rust library, CLI and
SQLite store. D023 is the authority for the identity boundary. No new dependency,
paid inference, dispatch, global hook or credential access is admitted.

## S01: implement D023 in the core

Make the library match D023 and nothing more. Write each failing test first.

- Eligibility may return a candidate whose resolved model id and version are both
  unknown, only when every other check passes. The candidate result carries an
  explicit identity status (unverified or verified). An unverified candidate is
  labelled unverified and ranks below any verified candidate for the same request.
- Keep the paired rule: a resolved id and version are both present or both absent.
  Never copy a configured value into a resolved field; add a test that proves it.
- Keep configured-only required capabilities unqualified, and keep every other
  existing eligibility reason unchanged.
- Comparison and accounting stay strict. An unknown observed identity still excludes
  a cohort, and an unverified profile never enters a ranking. Replace the misleading
  context_first_selection_mismatch reason for an unverified profile with an
  identity-specific reason, without loosening any check.
- Expose the identity status in the existing CLI eligibility output. Preserve
  existing databases, event schemas and current positive and negative tests.
- Adoption recording is out of scope; it belongs to a later slice.

## S02: Codex zero-inference probe

Add a CLI-side probe that observes Codex through its local app-server protocol over
stdio, builds a bounded registry snapshot, and imports it through the existing
inventory path. Keep process spawning out of the core library.

- Probe sequence: initialize, initialized notification, model/list, ephemeral
  thread/start, orderly shutdown. Never send turn/start. Zero model inference.
- Use an isolated owned working directory, bounded timeouts and an owned child that
  is always cleaned up. Do not read credentials, edit global Codex configuration or
  install hooks. If model/list needs a login that is not present, stop and record
  the blocker; do not fabricate output.
- Map only what the protocol reports. Model id, default effort and supported efforts
  are configured facts. Thread model and effort are configured settings, not
  per-turn telemetry. Leave resolved model id and version unknown and mark capability
  evidence configured, never runtime_confirmed.
- Derive the harness version and current-state values independently of the snapshot,
  as the inventory plan requires. Redact or drop raw private fields.
- Ship a versioned coverage table in 02_analysis/codex-adapter-results.md listing
  each field the probe observed, its source message, and what remains unavailable.
  Record a fresh-process smoke receipt: installed Codex CLI version, the sequence
  actually run, outcome, and an explicit statement that no turn/start was sent.
- Fixtures can test parsing, but they are not the smoke receipt. If the real probe
  cannot run here, report the slice as blocked; do not substitute a mock.
- Extend product_verify.py only as far as needed to discover and run the new tests,
  and keep the live smoke separate from the hermetic full command.

## Dependency and limits

S02 depends on S01. The Claude adapter and adoption recording follow as separate
slices. This plan does not complete G4, accept adoption evidence, or allow a claim
that a recommendation was applied.
