# Question: Permission or exclusion for imported template distribution

Status: open — distribution blocker; independent implementation may continue

- Date asked: 2026-10-25
- Owner or likely answerer: Architect owns artifact exclusion and evidence;
  the human owner obtains permission from the template rights holder if needed.
- What one question needs an answer? Before packaging, can we establish an
  applicable permission grant for imported agentic-project-template-v4 material,
  or demonstrate that all such files and derived template material are excluded
  from every distributed artifact?
- Why does it block or affect the slice? D001 imported revision
  169a74c66140743008bef374f2c7d0389ee81cb9. THIRD_PARTY_NOTICES.md records no
  standalone license and no grant in the checked README/pyproject sources.
  D011 licenses original RelayForge work only. Packaging/distribution cannot
  proceed on an inferred upstream grant. Local development and independent
  implementation are not blocked by this workflow gate.
- Current guess, explicitly not authority: Prefer a native-only, explicit
  artifact allowlist that excludes template scripts, workflow documentation,
  prompts, governance, and other imported/derived material. Exclusion must be
  proven by provenance review and actual artifact-content inspection, including
  source archives and installers, rather than assumed from file extensions.
- Closure evidence required: Either a recorded grant tied to the source and
  intended distribution, or reviewed packaging inputs/allowlist proving exclusion
  before packaging plus a passing inspection of each resulting artifact before
  distribution. Preserve third-party notices for anything that remains included.
- Authority: Explicit owner instruction to track the missing grant as a
  distribution blocker and continue independent implementation work meanwhile.

## Answer

- Answered by: pending
- Date answered: pending
- Answer: unresolved
- Decision or roadmap artifact updated: D012; release-contract.md G1/G6 gate
