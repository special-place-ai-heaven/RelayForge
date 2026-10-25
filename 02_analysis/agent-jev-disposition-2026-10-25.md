# Agent-JEV: rejected candidate

Date: 2026-10-25
Status: rejected by owner; investigation closed on that instruction.
Decision authority: [D007](../00_brief/decisions.md#d007--reject-agent-jev).

Source: [malevrigns/agent-jev](https://github.com/malevrigns/agent-jev/tree/741c58f567681b78e4afa1e12d7491d00fda69e7)
at `741c58f567681b78e4afa1e12d7491d00fda69e7`, cloned in the disposable research
folder. Repository code is Apache-2.0. No dependencies, weights, service or hooks
were installed; no project tests or model evaluations were run.

Completed inspection established:

- It is a local trained typed classifier with a Python/PyTorch HTTP service. It
  does not supply RelayForge's empirical worker-model/effort registry. A separate
  experimental next-action router was not found wired into the serving/hook path.
  Sources: `agentjev/model.py:94-151`, `agentjev/routing_runtime.py:30-81`,
  `jev_service/engine.py`, `pyproject.toml:5-20`.
- The bare release route rejects paths above its default 2,048-token limit;
  config-bearing training checkpoints can truncate and report a counter. Artifact
  identity therefore changes input semantics. Sources: `jev_service/contract.py:68-80`,
  `jev_service/engine.py:117-156,185-193`.
- Broad benchmark targets are public teacher distributions, not observed coding
  success. Narrow generated coding-gate evaluations still report false passes on
  held-out runners and sensitivity to wording. Sources:
  `typed_decisions/prepare_data.py:12-26`, `typed_decisions/protocol.json:18-21`,
  `coding_gates/README.md:33-55`.
- The inspected host hook is a Claude Bash/Write/Edit guard, not native worker
  routing. It sees limited command/path evidence and defers on service failures.
  Sources: `agentjev_hook.py:69-92,117-146`.

Detailed model-encoder inspection was stopped when the owner rejected adoption.
These are source observations, not a completed runtime qualification. Keep this
note to prevent repeated evaluation; do not schedule a local-backend experiment
or import components from this project unless the owner reopens the decision.
