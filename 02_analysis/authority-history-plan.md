# Historical authority repair

M006-S01 repairs an observed workflow defect before further native-adapter
decisions. Appending to decisions.md currently makes `ledger.py check` fail on
the historical M004-S02 resolution authority. The old decision text is unchanged
and retained in reachable Git history. Do not rewrite ledger events, frozen
envelopes, reports, original decision text or recorded hashes.

## Required behavior

Keep ordinary evidence references strict. For resolution-authority references
only, permit a current document to extend the originally cited document when:

1. The original content with exactly the recorded normalized SHA-256 can be
   recovered from reachable repository Git history for the same safe relative
   path (not arbitrary/unreachable blobs or another path).
2. The current normalized content has that entire original content as an exact
   prefix. Missing files, altered old sections, truncation or different content
   must still fail. An unchanged current file continues using the existing path.
3. Consumers receive the original cited content, not new appended authority.
   Appending a decision cannot retroactively enlarge an old resolution's grant.

Use one consistent narrowly named authority reader across integrity validation
and prompt/resolution consumption. Do not add a general ignore-drift mode or make
all evidence fall back to history. Preserve existing schema/old ledger bytes,
path safety, and failures when provenance cannot be established. Bound historical
lookup or deduplicate it within one operation without stale cross-operation
cache claims. No dependency or host/global Git configuration change is needed.

## Regression evidence

First reproduce the current append-only authority failure in a disposable Git
fixture. Test successful unchanged and append-only reads returning original
authority, plus rejection of changed old content, missing file, wrong hash/path,
unreachable history and absent provenance. Ordinary artifact/outcome evidence
must continue rejecting live-file drift. Exercise actual resolution/integrity
consumption rather than testing an unused helper only. Use the existing verified
owner identity for task-owned fixture commits; no invented identity or trailers.

Add a dedicated stdlib unittest file and include its explicit nonempty suite in
the maintained full verification command. Run focused tests and the existing
product/canary full verification; root will then record the next decision by
appending to the unchanged register and observe ledger check pass. Keep this
workflow repair outside native artifact packaging under Q001.

No product Rust changes, decision-content changes, historical evidence rewrites,
autonomous runner enablement or packaging are admitted by this slice.
