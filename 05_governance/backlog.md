# Backlog

Carried P3 findings and explicitly deferred work live here, one line per item.
Do not use this file as slice state or a substitute for `roadmap.yaml`.

<!-- findings:begin -->
| ID | Severity | Disposition | Source | Summary |
| --- | --- | --- | --- | --- |
| M001-S01-01 | P2 | closed_by_review | 05_governance/reviews/m001/M001-S01_r1_review.md (0fa47e4b087ad0fedf684fcc2ddec5cb172fcd2c17ed1e81f323a4722821692b) | Inventory combines “unsupported” and “unknown,” and routing does not specify how unresolved or merely configured capabilities affect eligibility. Distinguish these states and define safe eligibility behavior so an unverified capability cannot qualify a profile by implication. |
| M002-S01-001 | P1 | closed_by_review | 05_governance/reviews/m002/M002-S01_r1_review.md (36e0871170a1e07dc60a8bb9f68952d81480549e2d376f9edb2023067db68d82) | The canary plan says both to keep source in a disposable TEMP directory and delete it after retaining evidence, and that the canary coder owns tracked workspace files and a maintained verifier launcher. It does not explain how those files persist in the checkout or how the clean-checkout verifier exercises them. This leaves the immediate next admission without a reproducible source and verification arrangement. |
<!-- findings:end -->
