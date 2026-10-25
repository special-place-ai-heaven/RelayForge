Implemented a scratch-only candidate Git baseline: candidate blobs and trees go to owned scratch, original objects are read through alternates, and private metadata, index, and baseline commit point to the exact candidate tree. Existing diff checks remain enabled.

**Changed files**

- `scripts/canary_verify.py`
- `02_analysis/canary-results.md`

**Commands run**

- `python -B scripts/canary_verify.py --self-test` — passed; private baseline cleanliness and real source/index/HEAD invariance passed.
- `python -B scripts/hermetic_verification.py` — failed before candidate materialization at `scripts/ledger.py check`, reporting drift in `scripts/canary_verify.py`.
- `git status --short --untracked-files=all` — passed; scoped to owned paths.
- SymForge syntax validation — passed.

**What could not be verified**

- The full Rust/Cargo checks and authoritative full verification after the architect records this correction; the pre-coded full attempt stopped before the canary.

**Deviations from the prompt**

- The prescribed full command was attempted once and stopped at the expected pre-coded ledger drift. The architect must record the corrected source and run authoritative verification. The failure and r2 whitespace-check evidence are documented in `02_analysis/canary-results.md`.
