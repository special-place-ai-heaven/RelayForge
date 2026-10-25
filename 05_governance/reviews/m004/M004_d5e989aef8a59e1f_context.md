Source review: `05_governance/reviews/m004/M004-S03_r1_review.md` (sha256 `9e807b3a0836d2920e5306779c98a093575a2e8deaed2f0d986f72494b852b29`)
| M004-S03-BOOT-001 | P2 | open | `scripts/ci_contract_check.py:126-128`: `late_checksum`’s pattern is over-escaped and does not remove the checksum before execution. It appends another checksum after execution, so the exact-script comparison rejects a changed script without exercising the reordered-checksum case. Build the fixture by moving the actual checksum line after installer execution. |

Source review: `05_governance/reviews/m004/M004-S04_r1_review.md` (sha256 `b38d809d3aea89a5ffc555115c0869fecbfb96c7223d16e9276fc9e1ca37c7f5`)
| M004-S04-ENV-001 | P2 | open | `scripts/ci_contract_check.py:79-81, 167-171`: validation checks only job-level wrapper values. A read-only probe added `RUSTC_WRAPPER=sccache` to the Windows full-verification step's environment; `validate` accepted it, allowing a step-level override to restore the observed failure. Reject nonempty wrapper values in every step environment and add a fixture. |

Resolution authority: `00_brief/decisions.md` (sha256 `46cd9399b2f3e71fd6dcb15e2be06acc808e75e91e00dd44d7a245f658216851`)

Unblock reason: Reviewed bootstrap and wrapper fixes are committed in e3e30a1; actual run 40213004003 has passed all five Linux stages. Resume unchanged qualification scope and collect hosted Windows conclusion.

Original blocker: `05_governance/reviews/m004/M004-S02_r1_outcome_12fed74cb06b40de.json` (sha256 `12fed74cb06b40de9f74813b6960b900b6099119b1397c0e5749c236c774b447`)
{"action":"Admit and review job-local Rust bootstrap correction, push, then resume actual-run qualification","actor":"architect","authority":["00_brief/decisions.md"],"evidence":[{"path":"02_analysis/ci-qualification.md","sha":"9e8cab495819a8f4a3142b477e6b9978e5ff049f81ab1e91e2a59c366f79407a"}],"invocation":"manual-M004-S02-r1","outcome":"blocked_environment","reason":"Actual Linux jobs stopped at provisioning because rustup was not installed; hosted Windows passed","remaining":["Linux qualification and complete per-job version evidence"],"requirement":"Actual successful Windows and all five Linux CI checks","round":1,"schema":"frutlups.outcome/2","slice":"M004-S02"}


Recorded resolution: Reviewed bootstrap and wrapper fixes are committed in e3e30a1; actual run 40213004003 has passed all five Linux stages. Resume unchanged qualification scope and collect hosted Windows conclusion.

Retained changes: `05_governance/reviews/m004/M004-S02_r1_manifest_bc43a4fc31123b58.json` (sha256 `bc43a4fc31123b58b85ccf7f6f1970c864d6022bd513d2c80c879c7aa248b7f4`)
