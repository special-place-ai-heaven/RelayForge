# HN code-review discussion: evidence and limits

Date: 2026-10-25
Status: optional research evidence; no feature admission.

## Sources checked

- [HN discussion 49840300](https://news.ycombinator.com/item?id=49840300), read with crawl4ai.
- Linked project [egma-ai/jev-code-reviewer](https://github.com/egma-ai/jev-code-reviewer/tree/5bc56f2e8116af8888f98e4c0f1002956011f204),
  repository README, provider source and bundled demo metadata at
  `5bc56f2e8116af8888f98e4c0f1002956011f204`.

This is a different project from devagrawal09/jev-review in the earlier roundup.
No clone, install, provider call or experiment was performed for this check.

## What can be established

The bundled [demo report](https://github.com/egma-ai/jev-code-reviewer/blob/5bc56f2e8116af8888f98e4c0f1002956011f204/demo/report.json)
records `jev-1.13.0`, generated `2026-09-19T00:41:41.176Z`. The
[provider code](https://github.com/egma-ai/jev-code-reviewer/blob/5bc56f2e8116af8888f98e4c0f1002956011f204/src/providers.mjs)
defaults to `jev-latest` with overrides, so a caller's version is not established
by that default. Individual HN commenters do not identify resolved versions or
publish controlled local-routing evaluations in the fetched discussion.

The [README](https://github.com/egma-ai/jev-code-reviewer/blob/5bc56f2e8116af8888f98e4c0f1002956011f204/README.md)
says priority is attention rather than correctness, limits default analysis to
the first 12 change units, keeps original code for unanalyzed changes, and starts
lower-priority files collapsed. The
[demo provenance](https://github.com/egma-ai/jev-code-reviewer/blob/5bc56f2e8116af8888f98e4c0f1002956011f204/demo/README.md)
discloses real JEV classifications plus prepared explanation text in the bundled
recording; it is not evidence of end-to-end review accuracy.

The thread includes process criticism, anecdotes about poor outcomes, classifier
skepticism and defenses of bounded classification. It supplies no common labeled
benchmark, version-controlled comparison or calibration study that settles
RelayForge's intended use. Motives such as fear of job loss cannot be inferred
from disagreement.

## Only actionable conclusion retained

Keep work bounded and evidence inspectable. A classifier can prioritize attention
or suggest a route; it cannot establish that omitted code is safe, verification
passed, or a worker will complete a task. Errors that hide important evidence can
cost more than a fast decision saves.

This reinforces the existing fixed-baseline, shadow-evaluation and observed-outcome
approach in [actionable findings](actionable-findings.md). It neither validates
nor disproves JEV worker routing. It adds no feature, blanket confidence threshold,
new approval flow, or reason to expand scope. Exact comparer-version and quality
claims remain unknown until a relevant local evaluation exists.
