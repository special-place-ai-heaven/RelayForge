Previous verification failed. Complete receipt: `05_governance/reviews/m007/M007-S01_r1_verification.json` (sha256 `dc885b417ccf30164f7ab48023122e938dc590bf0e66715aacb9d75b7480f2f6`)

stdout tail:
```text
roadmap: ok
ledger: ok
CANDIDATE TREE d14bd898b707e76ad4de89c62df9c03a3a75c834 files=375
COMMAND <absolute-path> check
EXIT 0: <absolute-path> check
ledger: ok
PRIVATE CANDIDATE LEDGER CHECK PASS: original Git ancestry is visible
CANDIDATE CLEAN PASS: tree=d14bd898b707e76ad4de89c62df9c03a3a75c834; private baseline=eca6e97f53ca225cdb31c7563130951e9c7c908e; scratch-only metadata/index/objects
COMMAND <absolute-path>
EXIT 1: <absolute-path>
roadmap: ok
ledger: ok

```

stderr tail:
```text
warning: M002-S02: slice entry exceeds 2 KB
warning: M003-S01: slice entry exceeds 2 KB
warning: M004-S01: slice entry exceeds 2 KB
test_append_only_resolution_passes_artifact_integrity (__main__.AuthorityHistoryTests.test_append_only_resolution_passes_artifact_integrity) ... ok
test_changed_old_content_and_truncation_fail_both_integrity_paths (__main__.AuthorityHistoryTests.test_changed_old_content_and_truncation_fail_both_integrity_paths) ... ok
test_inherited_git_selectors_do_not_redirect_fixture (__main__.AuthorityHistoryTests.test_inherited_git_selectors_do_not_redirect_fixture) ... ok
test_missing_file_wrong_hash_and_wrong_path_fail (__main__.AuthorityHistoryTests.test_missing_file_wrong_hash_and_wrong_path_fail) ... ok
test_normalized_append_preserves_original_grant (__main__.AuthorityHistoryTests.test_normalized_append_preserves_original_grant) ... ok
test_ordinary_evidence_drift_remains_strict (__main__.AuthorityHistoryTests.test_ordinary_evidence_drift_remains_strict) ... ok
test_original_on_unreachable_branch_does_not_grant_authority (__main__.AuthorityHistoryTests.test_original_on_unreachable_branch_does_not_grant_authority) ... ok
test_prompt_receives_original_authority_without_later_decision (__main__.AuthorityHistoryTests.test_prompt_receives_original_authority_without_later_decision) ... ok
test_unchanged_authority_uses_current_file (__main__.AuthorityHistoryTests.test_unchanged_authority_uses_current_file) ... ok

----------------------------------------------------------------------
Ran 9 tests in 4.228s

OK
warning: M002-S02: slice entry exceeds 2 KB
warning: M003-S01: slice entry exceeds 2 KB
warning: M004-S01: slice entry exceeds 2 KB
test_append_only_resolution_passes_artifact_integrity (__main__.AuthorityHistoryTests.test_append_only_resolution_passes_artifact_integrity) ... ok
test_changed_old_content_and_truncation_fail_both_integrity_paths (__main__.AuthorityHistoryTests.test_changed_old_content_and_truncation_fail_both_integrity_paths) ... ok
test_inherited_git_selectors_do_not_redirect_fixture (__main__.AuthorityHistoryTests.test_inherited_git_selectors_do_not_redirect_fixture) ... ok
test_missing_file_wrong_hash_and_wrong_path_fail (__main__.AuthorityHistoryTests.test_missing_file_wrong_hash_and_wrong_path_fail) ... ok
test_normalized_append_preserves_original_grant (__main__.AuthorityHistoryTests.test_normalized_append_preserves_original_grant) ... ok
test_ordinary_evidence_drift_remains_strict (__main__.AuthorityHistoryTests.test_ordinary_evidence_drift_remains_strict) ... ok
test_original_on_unreachable_branch_does_not_grant_authority (__main__.AuthorityHistoryTests.test_original_on_unreachable_branch_does_not_grant_authority) ... ok
test_prompt_receives_original_authority_without_later_decision (__main__.AuthorityHistoryTests.test_prompt_receives_original_authority_without_later_decision) ... ok
test_unchanged_authority_uses_current_file (__main__.AuthorityHistoryTests.test_unchanged_authority_uses_current_file) ... ok

----------------------------------------------------------------------
Ran 9 tests in 7.715s

OK
product verification failed: verification scratch is outside the canary-owned temporary tree
verification command 6 failed: argv=['<absolute-path>', '-B', 'scripts/product_verify.py'] cwd=<absolute-path> exit=1
product verification failed: command failed with exit 1: <absolute-path>
verification command 6 failed: argv=['<absolute-path>', '-B', 'scripts/product_verify.py'] cwd=<repo> exit=1

```
