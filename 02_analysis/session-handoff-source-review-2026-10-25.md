# JEV session handoff: article review

Date: 2026-10-25
Status: architect research intake; no new feature admission or accepted policy.

## Source and evidence limits

[AI coding agent session compaction with Jev](https://codeaholicguy.com/2026/09/30/ai-coding-agent-session-compaction-with-jev/),
published 2026-09-30; read using crawl4ai on 2026-10-25.
This review covers the article, not a source audit or live run of AI DevKit.
No package was installed and no session content was sent to a model.

The article describes AI DevKit exporting typed handoff artifacts: JEV labels
session events, then code selects and formats them. Labels cover instructions,
decisions, changes, command/validation evidence, blockers, next steps, memory
candidates and discard. It describes session adapters for multiple harnesses,
including Codex and Claude, with Markdown and JSON output.

Its single Codex example reports 55 adapter messages: approximately 21.6k input
tokens became 5.9k Markdown tokens, about 73% smaller. The author separately
compares against a 130.6k end context and warns that the 5.6 MB raw rollout is
an unfair baseline. Counts use o200k_base estimates. The reported 0.36-second
classification timing and vendor latency/calibration claims are unverified here.
This example measures artifact size, not downstream task success or total savings.

## What this adds to existing research

The [Ten Levels source review](ten-levels-jev-source-review-2026-10-25.md)
examines advice about when to compact and where live work begins. This article
adds a different pattern: select operational material for a new agent, reviewer,
or resumed session, then construct an extractive artifact in code. Selection does
not require JEV to generate summary prose.

Inference: a handoff builder operating on supported exports could be useful even
when a host exposes no compaction hook. That does not establish automatic native
context replacement, reliable event coverage, or adapter compatibility. Those
need separate qualification. Runtime native compaction and exported handoff
creation must have distinct capability flags and execution receipts.

## Proposed RelayForge implications

These are design proposals for existing observation/reconciliation contracts,
not changes to roadmap or accepted decisions.

### Preserve two different views of the same evidence

A continuation view should be small: current objective, constraints and their
provenance, accepted decisions, relevant changes, verification receipts, unresolved
issues and the next action. Include stable evidence references so a fresh reader
can inspect more when necessary. Code should preserve known mandatory fields
without asking a classifier whether they matter.

The performance record needs attempted work, failure/retry counts, costs, causal
uncertainty, actual model/effort spans and recovery links even when a compact
handoff omits repetitive content. Do not discard failure accounting because an
exploration was abandoned or the final deliverable succeeded. Retain minimized
metadata and necessary evidence references under the existing privacy policy;
this is not a proposal to archive all raw transcripts.

A compact handoff is a derived view. It must not overwrite the authoritative
outcome record, template ledger, decision register, or verification receipts.

### Prefer deterministic extraction before semantic selection

Start from available structured events. Deduplicate repeat receipts and link
changed files/revisions locally. Use JEV only for genuinely ambiguous material,
batching bounded independent questions where practical. The article does not
establish that classifying every message is necessary or cheapest.

Version the selection policy and record coverage, input cursor, source revision,
selected event IDs and skipped/unavailable evidence. Keep uncertain material
retrievable. A classifier's exclusion must not silently become proof that no
constraint, failure or blocker existed.

Track the evidence funnel: source events, adapter-visible events, retained
evidence, and actual resume input. Record the handoff/compaction policy version
with the worker's configuration. Otherwise a better continuation policy can be
mistaken for better model/effort performance, or adapter losses for model failure.

### Resolve changes and contradictions explicitly

Event-level importance labels alone cannot determine which instruction is current
or whether a blocker was resolved. Preserve ordering, supersession and resolution
links. Treat user authority, imported documents and tool output as different
sources; selecting text for a handoff does not promote its authority. Memory
candidates remain proposals governed by the relevant store's write policy.

The artifact should distinguish observed verification from plans, claims and
missing evidence. If a session adapter omits tool results, it must report that
gap rather than manufacture validation evidence from assistant messages.

### Keep privacy checks before provider transmission

Asking a model whether text contains a secret does not prevent that text from
already reaching the provider. Minimize/redact data locally before model calls,
then apply the configured data policy. An optional semantic sensitivity label
is an additional signal, not the credential-protection boundary.

### Measure continuation quality, not compression alone

A later experiment should compare a deterministic handoff baseline against a
bounded JEV-assisted selection on the same source evidence. Measure constraint
and blocker retention, provenance/receipt correctness, stale decisions, missing
failure accounting, next-agent completion and rereads, total model/tool cost,
latency and recovery overhead. Keep unknown or missing adapter coverage explicit.

A smaller handoff can cost more if it causes a missed constraint or repeated work.
Source-stage token counts and the receiving model's billed usage must remain
separate; a reduction in one artifact is not a measured reduction in a full task.

## Effect on the next session

This reinforces the planned durable event record and reconciliation boundary.
It does not require more general online research before the foundation contract
can begin. Handoff selection can remain a later bounded experiment after reliable
observation and provenance exist. Product language, exact toolchain and native
Codex/Claude coverage remain unresolved; M001-S01 is still unstarted.
