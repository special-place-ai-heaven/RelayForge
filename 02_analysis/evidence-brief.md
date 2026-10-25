# RelayForge evidence brief — Codex + Claude first

> Imported from the pre-project research brief dated 2026-10-25. Findings below
> describe that audit and its source material; they are not a claim that RelayForge
> implements them or that the same configuration is active on a new machine.
> Accepted project decisions live in [the decision register](../00_brief/decisions.md).
> The later owner direction takes precedence over earlier implementation suggestions:
> RelayForge's product runtime will be non-Python, with the language still undecided.
> Python belongs to the retained project workflow scripts. Existing Python JEV code
> is a reference/integration to inspect, not a product-language decision.
> For the current bounded design entry point, read [foundation brief](foundation-brief.md);
> consult [phase-one spec](phase1-spec.md) for the complete proposed contracts.

**As of:** 2026-10-25
**Purpose:** preserve the research and read-only configuration findings as inputs to a later JEV design spec.
**Scope:** Codex and Claude are phase one. OMP and Cursor are deferred. No harness configuration was changed.

## 1. What the system is meant to do

The owner’s durable goal is to use JEV for bounded decisions across harnesses: choose the best available worker/profile, model, and thinking effort for a task; recommend a relevant skill; and, where worthwhile, narrow tool/MCP choices. The goal is lower cost and context use without losing task quality, with work routed to the least costly profile that meets the quality bar.

The intended split is:

- **JEV:** classify, score, choose among declared candidates, and report confidence.
- **Generative worker:** reason, write, edit, call tools, and return open-ended content.
- **Harness adapter:** translate a JEV choice into an agent/profile/skill/tool action that the harness actually supports.
- **Registry/evaluation data:** tell JEV which choices exist and what they cost, support, and achieve in this system.

A skill can describe an invocation workflow; an MCP server can expose a callable service; a plugin can package skills, MCP configuration, or hooks. None of those is itself the shared decision logic. The most portable design is a shared JEV core and registry with harness-specific adapters. The first useful behavior to implement is task-to-worker-profile routing. Tool/MCP filtering can follow after its context savings and error costs are measured.

## 2. Critical finding

JEV exists locally as a router with per-harness policies, but the inspected policies do not form a current capability registry or reliably enact a model/effort choice across Codex and Claude.

The problem has five parts:

1. **Different triggers:** Claude calls JEV automatically from a UserPromptSubmit hook; Codex relies on the manual route instruction.
2. **Different choice sets:** Codex policy exposes persona only; Claude exposes persona and skill. Neither policy exposes the model-role/profile decision needed for model-plus-effort routing.
3. **No clear enactment contract:** both JEV policies are advisory. A hook can put a recommendation into context, but the harness or orchestrator still has to invoke the selected agent/profile.
4. **No measured route quality:** the inspected Codex and Claude policies have no harness-specific evaluation set. Cursor has separate evaluation data, but that is not proof of Codex or Claude performance.
5. **The written lane rules conflict with the live defaults:** the owner’s role table names Astra as Codex chair and Fable as Claude chair, while the current main settings are Sol medium and Sonnet medium. The intended mapping needs one canonical definition before any routing rule can be trusted.

This configuration evidence is a plausible explanation for JEV feeling random; it is not a measured root-cause diagnosis.

## 3. Control plane: what can make a decision happen

| Control | What it controls | Current Codex | Current Claude |
|---|---|---|---|
| Prompt trigger | When JEV runs | Manual route instruction and Jev skill; no hooks.json and no inline hooks table found | Active UserPromptSubmit command hook matching Jev; a second UserPromptSubmit command hook is also present |
| Candidate families | What JEV may choose | persona only | persona and skill; no model-role/profile family |
| Harness roles | What a choice can invoke | 15 agent TOML files exist on disk, all pinned to Astra; global config has no custom agents role declarations | 24 user agents exist, each with model and effort frontmatter |
| Main model | Default worker if no profile is invoked | gpt-6.1-sol, medium | Sonnet alias, medium |
| Hook effect | What the integration can directly change | Native hook support exists, but none is configured; a command hook can supply context, not directly switch model/effort | UserPromptSubmit runs before the model processes a prompt and can supply context; the inspected JEV integration is advisory |
| Tool surface | MCPs that may be available | Config says terminal-commander, SymForge, crawl4ai-local, EverOS, agentmemory, remindb-vault, Citadel state, codebase-memory, and node_repl are not marked disabled; Charlotte, Obsidian, and Stitch are disabled | Runtime list reports agentmemory, crawl4ai-local, remote crawl4ai, EverOS, remindb-vault, SymForge, and terminal-commander connected; Charlotte, Obsidian, and Stitch disabled |
| Runtime certainty | Whether configured state is confirmed live | Codex CLI was not available in this shell, so local config/profile discovery and effective MCP state were not verified in a running Codex CLI | claude --version reports 2.1.306 and claude mcp list returned the statuses above |

### Codex details

- Main configuration: gpt-6.1-sol at medium effort.
- Fifteen agent profiles exist under the Codex agents directory. Every inspected profile uses gpt-6-astra; effort values range from low to xhigh. That is expensive relative to the owner’s desired Luna workhorse lane, unless these files are loaded through a separate agent runtime or deliberately reserved for frontier work.
- The global config has no custom agents role declarations, default subagent model, or default subagent effort. Current Codex documentation allows custom role declarations with per-role config files and descriptions. This is a configuration gap, not a platform limitation.
- The Codex Jev policy chooses personas only. It does not define a model-role or skill family.
- The Codex Jev instruction is manual, and the local Codex hook config is absent. Codex supports UserPromptSubmit command hooks that can add context, but the hook must still route through a named agent/profile for the model and effort to change.
- The Codex global config has remindb-vault enabled by default or not explicitly disabled, contrary to the supplied AGENTS.md rule that Codex should disable it. The runtime status is unverified here. The extra Citadel state, codebase-memory, and node_repl servers also need an explicit keep/disable decision.
- The currently inspected Codex skills include Jev, project-team, and ledger-repair, plus system skills such as openai-docs and skill-creator. Ponytail is not installed under the Codex plugin or skill directories inspected.

### Claude details

- Main setting: model alias Sonnet, effort medium. This is an alias, so the exact provider model ID should be recorded when resolved if the registry is to attach current capability and price facts.
- Twenty-four user subagents are present. They already provide native model and effort profiles: Sonnet/medium is common, two Sonnet profiles use high, most Opus profiles use high, security-reviewer uses xhigh, and ui-designer uses medium.
- Claude’s official subagent format supports model, effort, tools, skills, hooks, and permission settings. Therefore separate logical lanes can be implemented as native subagents.
- Jev is automatically called by a UserPromptSubmit command hook. The Claude JEV policy has persona and skill families, but its explicit route rules are empty and it has no model-role/profile family. This can produce advice without a configured mapping to a named model/effort profile.
- Claude has both crawl4ai-local and the remote crawl4ai connected. The supplied AGENTS.md describes remote crawl4ai as session-only. The connected remote server is therefore a likely tool-surface mismatch and may incur external cost; confirm its exact billing/usage before relying on it.
- Ponytail is installed as a Claude plugin, version 4.9.0. Its upstream package includes skills and lifecycle hooks.

### Role correction

Custom roles are possible on both sides:

- Codex supports custom agent role declarations in its config and role-specific TOML config layers.
- Claude subagents support per-agent model and effort fields.

So the earlier concern about a role name being inherently unavailable was too strong. A JEV candidate must still correspond to a role/profile registered in that harness. For example, a Jev candidate named workhorse should resolve to an actual Codex role or Claude subagent with a valid model and effort, not just a string in a Jev policy.

## 4. Shared registry contract the spec should define

The registry should use a harness-neutral logical profile ID, then map it to a native, selectable profile in each harness. JEV must choose only from the current harness’s registered choices.

At minimum, each profile needs:

- logical lane and profile ID, such as scout, workhorse, frontier, or judge;
- harness and native agent/profile name;
- exact model ID, provider, supported effort levels, and alias-resolution timestamp;
- allowed MCP/tool set, skill set, permissions, and any task restrictions;
- model capability and price facts with source and as-of date;
- measured task success, latency, cost per successful task, escalation/fallback rate, and sample size;
- default/fallback profile when JEV is uncertain, unavailable, or below a decision-specific confidence threshold.

Do not treat model capability tables or vendor pricing as sufficient proof that a profile is cheap and reliable in this harness. Static metadata describes what a model claims to support. Evaluations show what the combined model, harness, skills, tools, and permissions achieve on the owner’s tasks.

JEV should return a bounded result such as a profile ID, probabilities, confidence, and decision evidence. The adapter should validate that the ID is currently registered, invoke the native profile, and record the outcome. It should never accept arbitrary model IDs from free-form JEV text.

## 5. Research findings to preserve in the spec

### Boris Cherny / Claude Code

The supplied YC Root Access interview transcript describes repeated ablation: remove system-prompt instructions and tools, run the product with a new model, and add back only what repeatedly addresses a real failure. It says prompts and tool sets change with models and that instructions should not be retained merely because they helped an older model.

**Design implication:** keep always-loaded guidance small, measure repeated behavior, and reevaluate role prompts and JEV routing when model aliases resolve to new versions. This is an interview transcript and reported engineering practice, not an independent routing benchmark.

Source: [Boris Cherny: Building Claude Code](https://www.ycrootaccess.com/p/boris-cherny-building-claude-code)

### TypeSafe Jev and the Composio guide

Jev is a decision model for typed, bounded outputs: choices, yes/no or Noul decisions, and scores. TypeSafe’s API accepts a state and one or more named questions and returns typed answers with probabilities; the API exposes GET /v1/models for model discovery. It is a good fit for selecting among known worker profiles, classifying a tool request, or deciding whether to escalate. It is not a replacement for generating code, explanations, or plans.

TypeSafe’s launch page reports 70–500 ms responses and input pricing of $0.042 per million tokens with free output. Treat those as vendor-reported, time-sensitive claims and attach an as-of date. A constrained schema prevents an out-of-schema answer; it does not guarantee the chosen answer is correct.

The Composio guide’s example use cases include ticket routing, field extraction, warehouse classification, agent-harness decisions, browser decisions, and live-chat moderation. These are useful candidate patterns, not proven deployments in this owner’s harnesses.

Sources: [TypeSafe launch post](https://typesafe.ai/blog/introducing-system-one-models-and-jev), [TypeSafe API documentation](https://api.typesafe.ai/redoc), [Composio Jev guide](https://composio.dev/content/typesafe-jev-guide-and-best-practical-usecases)

### JevAgentHarness paper

The preprint benchmarks decision delegation from isolated questions through running agents. Its abstract reports that the best isolated decision model completed only 69.5% of agent tasks, and delegating tool choices without a fitted threshold reduced task success by 19 points. JevAgentHarness fits a separate confidence threshold for each decision point using the agent’s own trajectories, then leaves low-confidence cases to the LLM.

The paper reports only 7.0–10.5% cost savings for the tested coding-agent configurations, with success varying by harness. Larger savings in the paper apply to different settings, especially an AgentDecision-4B model trained on agent states; they should not be attributed to Jev generally or assumed for this Codex/Claude setup.

**Design implication:** calibrate per decision type and harness; test end-to-end success, latency, and cost; retain an LLM fallback. There is no universal confidence cutoff such as 0.87.

Source: [JevAgentHarness: Benchmarking, Deploying and Training Decision Models That Share Control with LLMs](https://arxiv.org/html/2610.21412v1)

### User-supplied X post and screenshots

The screenshots separate generation from decisions in a PR-review loop. They show approximately 24 seconds before and 11.8 seconds after, excluding the fix step; the annotated decision checks total about 0.83 seconds with Jev. The generation steps stay with Claude, and the 0.87 confidence threshold is described as tuned per decision.

**Evidence status:** useful architecture example from one author’s worked example, not a general performance guarantee. The paper above supports the need for thresholds and end-to-end validation.

Source: [Cyril XBT post](https://x.com/cyrilXBT/status/2101515130798297448)

### awesome-jev-tools

The user’s broken category link is no longer the live layout; Classification & Routing appears in the repository README and lists 109 entries. The repository also contains large lists for agent decisions, evaluation, calibration, verification, and integrations. Its own curation notes say that a README entry can be unproven and that code/API calls and traceable claims should be checked.

**Design implication:** use it as a pattern index, not as an endorsement or dependency list. Vet integrations against source code and a real evaluation before adding any.

Source: [awesome-jev-tools](https://github.com/v-modal/awesome-jev-tools)

### HarnessRouter

HarnessRouter is a self-hosted or hosted API for running agent harnesses as backends. Its README lists Codex and Claude Code among supported harnesses and describes sessions, streaming, artifacts, cancellation, and structured errors. It requires Docker and a provider API key for local setup. Its console fixes a harness for a saved setup, so it is a harness execution gateway rather than a drop-in Jev router for existing local interactive sessions.

Its published benchmark compares eight configurations on one controlled synthetic-data task. It reports a 475x cost range in its own credits and up to about 1.95x latency variation from harness choice at the same model. This supports testing the complete harness/model configuration; it does not predict results for the owner’s workloads.

**Fit:** potentially useful later as a cross-harness experiment or product backend, but not needed to fix Codex/Claude’s local Jev-to-agent routing first.

Sources: [HarnessRouter repository](https://github.com/HarnessRouter/harnessrouter), [published benchmark and methodology](https://harnessrouter.ai/benchmarks)

### Oh My Pi article — deferred

The supplied v18.4.4 article describes OMP’s model roles, judges, subagents, service tiers, and model metadata. The local OMP binary is now v18.12.6, so the article is historical context rather than an accurate current-state guide. OMP role aliases are configurable; a custom role is possible if registered in the harness configuration. Detailed OMP policy review is deferred by the current Codex/Claude-first scope.

Source: [Oh My Pi v18.4.4 update](https://aiidelist.com/blog/oh-my-pi-v18-4-4-update)

## 6. Ponytail and skill choices

- **Ponytail:** keep it as a coding behavior layer for minimal, YAGNI-oriented implementation. The Claude plugin is installed. The Ponytail package includes a Codex plugin manifest, skills, and hooks, but no Codex Ponytail plugin installation was found. Codex plugins can bundle skills and lifecycle hooks; plugin hooks require review/trust before running. Its README’s savings benchmark is self-reported, limited to 12 tasks with four runs, so use it as a style/control aid rather than a guaranteed cost multiplier.
- **setup-gbrain:** not the JEV registry. Its purpose is installing/initializing a repo knowledge brain and registering an MCP. That may help codebase knowledge; it does not provide live model capability, harness-role, skill/tool availability, or JEV outcome data.
- **openai-docs:** useful for current OpenAI model, effort, pricing, and Codex facts.
- **project-team:** useful when implementing the cross-harness work as bounded modules; not needed for the read-only audit.
- **skill-creator:** only needed if the shared core later needs a reusable skill front end.
- **gstack spec:** the installed workflow is oriented to writing/filing an issue, not a private internal design spec. This evidence brief should be the source input for an internal spec.
- **mcp-infrastructure:** the installed version contains stale host-specific instructions. Use the live configuration and current official docs as the source of truth.

## 7. Suggested implementation order

1. Resolve the written role-table conflict: decide which lane is the foreground chair and which profiles are the low-cost workhorse and scout.
2. Define the shared logical profile IDs and the Codex/Claude native binding for each one.
3. Add only the task-to-profile Jev choice first. In Codex, choose among native configured roles; in Claude, select a native subagent. Preserve direct main-thread work for small tasks.
4. Add or enable the Codex JEV hook only after its output and trust behavior are reviewed. Keep both hooks advisory/shadow until route outcomes are measured.
5. Build a small, representative evaluation set. Compare no-Jev baseline against Jev route on task success, cost per successful task, latency, fallback rate, and actual selected profile. Fit thresholds per decision.
6. Add skill selection next. Consider an MCP tool gatekeeper only if measurements show that the tool catalogue or bad tool selection is a material cost.
7. Extend to OMP and Cursor as separate adapters after Codex and Claude behavior is stable.

## 8. Sources for platform capabilities

- [Codex configuration reference](https://developers.openai.com/codex/config-reference) — custom agent roles, config-file layers, model and reasoning defaults, hooks, and MCP controls.
- [Codex hooks guide](https://learn.chatgpt.com/docs/hooks?site_variant=chatgpt) — supported events and command/MCP tool hook behavior.
- [Claude Code subagents](https://code.claude.com/docs/en/sub-agents) — user/project subagents and supported model, effort, tools, skills, and permission fields.
- [Claude Code hooks](https://code.claude.com/docs/en/hooks) — UserPromptSubmit behavior and hook context output.
- [Claude Code skills](https://code.claude.com/docs/en/skills) — on-demand skill behavior and plugin/skill integration.

## 9. Verification boundaries

- Local file/config facts above were read without displaying credential values.
- Claude CLI version and MCP connection status were read live.
- Codex local CLI runtime was not available in this shell; Codex configuration facts are file-level, not effective-runtime proof.
- No harness changes or tests were run.

## 10. HarnessRouter follow-up (README and benchmark rechecked 2026-10-25)

HarnessRouter is more than a benchmark harness: its Apache-2.0 Community Edition exposes an OpenAI Responses-compatible API implementing UHP, with harness selection, persistent sessions, streaming, files/artifacts, cancellation, and structured failures. A product-side JEV orchestrator could call that API to launch work on a configured Codex or Claude backend and collect execution records. The README lists 26 harness bases, including Codex, Claude Code, and Oh My Pi.

The boundary matters: this is a separate execution gateway, not a hook or transparent interceptor for an already-open local Codex/Claude session. It runs in its own Docker deployment and workspace environment with provider credentials configured there. The Console’s saved setup is tied to a harness; the API accepts a harness identifier per task. Its README does not establish that it imports this owner’s existing local profiles, plugin installs, MCP allowlists, or hooks unchanged. Those would need explicit compatibility checks before treating it as a route into the current setup.

**Fit for this design:** keep HarnessRouter as an optional later execution adapter/evaluation surface. First implement and measure native Codex/Claude JEV-to-profile routing. Later, compare (a) native adapters with (b) JEV calling UHP to dispatch a new task to a configured harness. Do not make HarnessRouter a prerequisite for local hook parity or the shared JEV registry. Evaluate extra deployment/configuration work, provider cost, latency, profile/tool parity, and task quality on the owner’s representative tasks.

The current public benchmark is one controlled synthetic-data task with eight configurations. It reports about a 475× credit range, 3.2× end-to-end latency range, and up to 1.95× latency variation from harness choice with model held constant. The page explicitly limits these results to that task and setup; the results motivate an owner-specific test, not an expected saving.

Sources: [HarnessRouter README and API overview](https://github.com/HarnessRouter/harnessrouter), [benchmark and methodology](https://harnessrouter.ai/benchmarks).

## 11. JEV implementation facts discovered during spec preparation

- SymForge returned `index_ready=false` for the JEV source, so the implementation inventory used bounded direct reads rather than an indexed code map.
- The existing `registry.py` exposes `md_agents`, `toml_agents`, and `skills` as name-to-description inventories; that is not yet the live model/profile capability and outcome registry described above.
- `prompt_hook.py` documents the current `shadow` mode as zero-network/log-only and `advisory` as context injection. A new shadow evaluation that calls JEV must be a separately named and explicitly enabled mode; silently changing the meaning of existing shadow would alter behavior and cost.
- `integrations/codex_route.py` already contains `build_candidates(live_agent_types)`, privacy gates, and `RouteResult`. Reuse and verify this adapter rather than assuming a new core is required. Its presence does not prove that Codex runtime triggers it.

These are source-level findings; they do not establish effective hook/runtime wiring in either harness.

## 12. Model/effort crossover, recovery, and session evidence (2026-10-25)

The owner refined the optimization target: for every eligible model, identify whether a stronger candidate at lower thinking effort can equal or exceed the preceding candidate at maximum effort while costing less to deliver accepted work. Public coding benchmarks are incomplete; tool use, repeated failures, wasted recovery tokens, and higher-cost parent repair matter. JEV should adapt to actual model-plus-effort failure and success rates, segmented by task difficulty and harness/tool configuration. Several wrong tool calls are considered a failure.

**Current evidence boundary:** no exact Astra effort was verified as an equal-or-better, cheaper replacement for GPT-6.1 Sol max. Official guidance says effort labels do not map exactly across generations and recommends comparison on the same inputs. Astra and GPT-6.1 Sol both list low, medium, high, xhigh, and max; supported labels identify candidates, not equivalent performance. Max and Ultra are different execution configurations: the latter uses subagents.

The official model catalog currently lists Standard uncached input/output prices of $10/$50 per million tokens for Astra and $2/$10 for GPT-6.1 Sol. These prices alone cannot answer the completed-task crossover question. Context caching, reasoning tokens, tool use, retries, and actual billing must be included.

Sources: [OpenAI model catalog](https://developers.openai.com/api/docs/models), [OpenAI model/effort guidance](https://learn.chatgpt.com/docs/models), [OpenAI model selection](https://learn.chatgpt.com/docs/model-selection), [deployment checklist](https://developers.openai.com/api/docs/guides/deployment-checklist).

Anthropic supplies useful effort-specific measurements, but none establishes the requested universal maximum-effort crossover. In its own 478-problem SWE-bench Pro subset, two runs per setting, Opus 5.5 low solved 87.4% at about $0.12 per solved task versus Sonnet 5.5 high at 87.6% and about $0.11. The small score difference is not evidence that either universally wins; the stronger-model low setting was not cheaper in that comparison. Its effort sweep also shows task-dependent returns. These are vendor measurements under their specified harness, cache, and billing conditions, not results with this owner's SymForge/Terminal Commander setup. Sonnet high is not Sonnet max. Publicly listed 5.5 models are not proof they are the native versions currently active in the inspected local configuration.

Sources: [Anthropic cost/intelligence measurements and methodology](https://platform.claude.com/docs/en/about-claude/models/optimizing-for-cost-and-intelligence), [Claude effort parameter](https://platform.claude.com/docs/en/build-with-claude/effort).

The phase-one spec now defines an outcome-driven ledger, pairwise crossover records, full recovery-chain cost, and session → task → attempt → actual model/effort span attribution. Wrong tool calls and accepted deliverables receive separate labels so eventual recovery cannot erase a costly operational failure. A proposed starting threshold of three confirmed model-caused wrong calls in one attempt is explicitly provisional, not a user-selected or measured value.

No paid model evaluation, harness change, or live telemetry instrumentation was performed during this documentation update.

## 13. Continuous observation and lifecycle coverage (2026-10-25)

The owner requests session/model/effort/difficulty success tracking, repeated wrong-tool-call failures, dynamic performance-based ranking, and inexpensive observation during work. Session-end analysis alone is insufficient because sessions can end abruptly.

Current [Codex hooks documentation](https://learn.chatgpt.com/docs/hooks) lists SessionStart, SessionEnd, PostToolUse, Stop, SubagentStart, and SubagentStop; SessionEnd does not run for subagents. Current [Claude hooks documentation](https://code.claude.com/docs/en/hooks) additionally lists PostToolUseFailure. This is documentation-level evidence, not a runtime payload verification. Codex failed-call metadata, actual model/effort attribution, and billing coverage still need to be checked in the installed runtime.

The spec recommends local metadata collection during work, incremental durable reductions, end-of-session consolidation, and startup reconciliation of pending events. Stable event IDs and transactional cursors prevent duplicate metrics; interrupted attempts remain unknown/interrupted until evidence determines their outcome. Normal observation and consolidation require zero additional model calls. JEV is reserved for unresolved routing/recovery decisions. A local embedded collector is the initial design; a persistent daemon is conditional on measured overhead, not a prerequisite.

Sections 15–17 of the phase-one spec now contain the refinement, data/behavior contracts, and R19–R37 acceptance criteria. These are design requirements; live tracking, hooks, and registry mutation were not installed during this documentation update.
