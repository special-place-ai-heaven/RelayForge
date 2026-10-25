# M004-S01 CI mechanism evidence

The workflow and portable verifier changes are the S01 candidate. The
architect-owned authoritative receipt is
`05_governance/reviews/m004/M004-S01_r1_verification.json`, created after the
coder reports focused checks and the ledger records coded work. This note is
not an execution receipt.

The CI trust gate and its negative fixtures are checked by
`python -B scripts/ci_contract_check.py`; the product stages are checked by
`python -B scripts/product_verify.py --focused`. The authoritative full command
remains `python -B scripts/hermetic_verification.py`.

Worker-local Windows observations for this candidate:

- The workflow contract check passed and rejected 14 unsafe fixtures, including
  a copied-fork push, fork PR gate removal, write token, mutable action, shallow
  checkout, missing timeout and altered runner labels/stages.
- Focused product verification passed on native `x86_64-pc-windows-msvc`.
  It discovered and executed 5 unit, 13 integration/CLI and 1 doctest with
  zero failures or ignored tests; formatting, Clippy, all-targets build,
  SQLite 3.53.1 and the actual CLI path also passed.
- Focused canary verification passed with 7 unit/integration tests and 1
  doctest, SQLite 3.53.1, deliberate crash exit, restart replay and acceptance
  assertions. Its candidate materialization self-test passed the ledger
  ancestry, clean scratch and source/index/HEAD stability checks.

Linux and hosted-Windows CI qualification is reserved for M004-S02. Runner
availability, isolation and five simultaneous runner slots remain unobserved
until real run evidence is recorded there.
