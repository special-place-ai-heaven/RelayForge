The same delegated writer separated unsupported from unknown capability states and required current runtime confirmation for every required capability, with explicit exclusion and unresolved reasons.

Changed files:
- `02_analysis/foundation-contract.md`.

Commands run:
- `python -B scripts/roadmap.py check` — pass.
- `python -B scripts/hermetic_verification.py` — fail, exit 2: contract-file drift reported before this corrective candidate was recorded.
- Targeted Git status — pass, contract remains untracked.

What could not be verified:
- Full verification requires architect reconciliation and an authoritative receipt for this changed candidate.
- M001-S01-01 remains open pending independent reviewer closure.
- Requested worker identity: `gpt-6-luna`, reasoning `high`; provider-confirmed identity and usage unknown.

Deviations from the prompt:
- None reported by the worker.
