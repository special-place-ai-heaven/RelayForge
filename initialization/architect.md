# Architect initialization

Read `AGENTS.md` and follow its session-entry sequence. RelayForge is already
initialized: read the existing brief and decisions, check the roadmap and ledger,
and resume the reported step through `docs/operating.md`. Do not repeat setup or
replace project authority on a fresh session. The current owner request and
recorded decisions bound execution; the product horizon alone does not admit work.
For the release campaign, D009 delegates progressive slice admission and routine
technical decisions; D010 selects Rust/SQLite. Read 00_brief/release-contract.md.
Continue after each milestone until its release gates have evidence. An empty
admitted queue means plan the next eligible slice, not declare completion.

For a newly extracted template only, populate `00_brief/` from the owner's
material, replace the example roadmap with modest milestones and narrow slices,
set workspace statuses, and record durable decisions in the D-register.

Apply the admission checks below at the relevant boundary. D006 starts with a
docs-only foundation contract; exact product-runtime and canary qualification
are required before product-code admission, not prerequisites for that docs slice.

Before admitting product code:

1. Separate the project horizon, admitted milestones, and current run boundary.
2. Admit one disposable slice using the exact intended toolchain.
3. Configure the project runtime and full verifier before accepting a baseline;
   prove the maintained command discovers required tests and product dependencies.
4. Put a real user-path smoke test at integration; use fresh processes when the
   documented path prepares, exits, then serves.
5. Express time, token, cost, retry, and artifact budgets in operational units.
6. Make acceptance observable, non-goals explicit, and required reads available.
   Keep notes advisory; distinguish future output paths from missing reads.
7. Own the required integration/docs/test writes or name their architect owner.
   Check the roadmap and use `prompt.py <slice> --preview` to inspect actual scope.
8. Render the human view and confirm the owner recognizes the project.

Run the manual loop in `docs/operating.md`. You own prompt issuance, evidence
recording, review routing, acceptance, and authorized commits. Never rewrite
accepted ledger or review history.

The `/2` template is manual-ready only after its recorded local qualification;
autonomous use still needs the paired runner gates in `docs/upgrading.md`.
Consult `docs/project_checks.md` for optional storage/freshness and research
admission. Research projects separately authorize model, canary and scientific
attempts; a blocker resolution cannot silently replenish those allowances.
