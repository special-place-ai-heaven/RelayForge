# Review: M007-S01 round 2

## Findings
| id | severity | disposition | summary |
| --- | --- | --- | --- |
| M007-S01-F1 | P3 | open | `ComparisonReport.eligibility` (`accounting.rs:198`) is the full, unfiltered `EligibilityResult`. Its `eligible_profile_ids` and `outcome` can list an unverified profile as eligible. The report's top-level `eligible_profile_ids` and `outcome` say otherwise (`accounting.rs:1123-1127`). The report also never says why an eligible-unverified profile was left out of comparison. Consider a comparison reason such as `unverified_identity_excluded`, or document the split. |
| M007-S01-F2 | P3 | open | "Verified" is defined in two places with different spellings: `lib.rs:1079` (both fields `Some`) and `accounting.rs:623` (either field `None`). They agree today but can drift. Consider one `InventoryProfile::identity_status()` used by both. |
| M007-S01-F3 | P3 | open | Test strength. The CLI assertion (`storage_contract.rs:1232`) accepts either `verified` or `unverified`. The CLI path always passes `current=None`, so it can never show an eligible-unverified candidate. Nothing pins the `#[serde(default)]` legacy-JSON behaviour. The `snapshot == before` check (`lib.rs:2029-2030`) is near-vacuous, since `evaluate_eligibility` takes `&InventorySnapshot`. None of these hides a defect. |
| M007-S01-F4 | P3 | open | The receipt has `tree_clean_after:false` alongside `ok:true`. This is probably the intentionally dirty candidate worktree, but the architect should confirm it on acceptance. The receipt also has a single `full` command, with `product_verify.py` running inside it rather than as a separate `--focused` entry. |

What I checked, from the diff and the receipt:
- **Unverified candidates:** `identity_status` is derived from data at `lib.rs:1079-1084`. The `unknown_execution_identity` eligibility reason is removed, so every other check still applies. The test at `lib.rs:1982-2021` proves the ranking `["profile-z","profile-a"]`. A configured-only capability on an unverified profile still gives `unresolved_configured_only` and `eligible=false`.
- **Ranking:** verified candidates come first and ties follow profile-id order (`lib.rs:1202-1210`). An explicitly selected unverified profile is eligible and labelled, and comparison drops it.
- **Comparison:** only eligible and Verified candidates enter the cohort (`accounting.rs:1123-1127`), so an unverified profile never reaches a ranking. The test at `accounting.rs:2277-2338` would fail if that filter were removed.
- **Accounting:** `accounting.rs:623` now returns `unknown_execution_identity` before the selection check. The observed-`Unknown` exclusion and the identity-mismatch check are untouched, so nothing was loosened.
- **Paired rule:** `validate_snapshot` (`lib.rs:933`) is unchanged, and `lib.rs:2022-2042` tests half-pairs in both directions. No code path copies a configured value into a resolved field.
- **CLI:** `main.rs:190` serialises `result.candidates` directly, so `identity_status` appears with no CLI edit, including on the early-continue branches.
- **Serialization:** `#[serde(default)]` lets older JSON deserialise under `deny_unknown_fields`, and the default is the safe `Unverified`. Event schemas, `InventorySnapshot` and the database tables are untouched.
- **Scope:** product changes are confined to `08_pkg/relayforge-core/{src,tests}`. No new dependency.
- **r1 failure:** `product_verify.py` failed with "verification scratch is outside the canary-owned temporary tree". That is an environment failure, not a product defect.

## Closure Decision
Objective status: achieved
Objective evidence: Eligibility now returns labelled, verified-first-ranked unverified candidates only when all other checks pass (`lib.rs:1079-1210`). Comparison and accounting stay strict with an identity-specific reason (`accounting.rs:623`, `:1123-1127`). The r2 full receipt is `ok:true` with exit 0. I did not observe any execution myself.

## Verdict
Verdict: pass - next: architect records the review, carries F1-F4 to the backlog (F1 and F2 are candidates for S02 or the adoption slice) and accepts M007-S01.
