# Review prompt: M006 — Preserve historical resolution authority across decision appends (round holistic)

Read `AGENTS.md` first. Do not change product files. Use the receipt as execution
evidence; do not rerun verification unless this prompt explicitly says so.

## Objective and acceptance

Judge holistic closure of M006 across its accepted slices.

### M006-S01

- Implement 02_analysis/authority-history-plan.md with no old ledger, report, envelope, hash or decision-text rewrites.
- Only resolution authority may use verified same-path reachable Git content with the recorded hash and exact current-content prefix; return original authority to consumers.
- Tampering, missing provenance and ordinary evidence drift still fail; all integrity and prompt consumers use the same narrow rule.
- Reproduce the original failure first, add nonempty positive/negative integration tests and include them in the maintained full command; full product/canary checks remain passing.
- Record observed behavior and limitations in 02_analysis/authority-history-results.md without changing native product behavior or Q001.

## Non-goals

### M006-S01

- Product changes, new dependencies, global Git changes, general drift bypasses, old evidence rewrites or packaging.

## Read first

### M006-S01

- `02_analysis/CONTEXT.md`
- `02_analysis/authority-history-plan.md`
- `docs/operating.md`
- `docs/upgrading.md`

## Implementation boundary

Allowed prefixes: ### M006-S01

`scripts/_protocol.py`, `scripts/_integrity.py`, `scripts/_evidence.py`, `scripts/prompt.py`, `scripts/ledger.py`, `scripts/hermetic_verification.py`, `scripts/test_authority_history.py`, `tests/test_authority_history.py`, `02_analysis/authority-history-results.md`

Forbidden: ### M006-S01

`00_brief/`, `05_governance/`, `prompts/`, `questions/answered/`, `roadmap.yaml`, `frutlups.toml`, `AGENTS.md`, `CLAUDE.md` and all other paths. These describe the coder's scope;
review remains product-read-only. Read the evidence artifacts named below using
your file-reading tools. Their categories describe volume, not authority.

## Declared verification

Focused:

### M006-S01

```text
python -B scripts/test_authority_history.py
```

Full:

### M006-S01

```text
python -B scripts/hermetic_verification.py
```

Execution policy: runtime {}; timeout 900 seconds; observation process.

## Changed files

8 cumulative paths. Complete manifest: `05_governance/reviews/m006/M006_e9fbe2d9e8c32f88_paths.json` (sha256 `e9fbe2d9e8c32f883360bcdc732a0aae2cd3cfb98857afa17cb35856a120983d`; read as a file, starting at line 1)

## Code diff

Bounded diff page: `05_governance/reviews/m006/M006_38b7b10a3a39a3fa_diff.md` (sha256 `38b7b10a3a39a3fad43cf5a4db72d9c84e77b611a9b1d44800b4db2a5013722d`; read as a file, starting at line 1)

```diff
git diff 7d3818a900bce7674b57ed2cf9ffad09c2d57d26..HEAD --stat (selected paths)
 02_analysis/authority-history-results.md |   9 ++
 scripts/_integrity.py                    |   5 +-
 scripts/_protocol.py                     |  63 ++++++++++
 scripts/hermetic_verification.py         |   1 +
 scripts/ledger.py                        |   6 +
 scripts/prompt.py                        |  14 ++-
 scripts/test_authority_history.py        | 207 +++++++++++++++++++++++++++++++
 7 files changed, 298 insertions(+), 7 deletions(-)

Per-slice diffs: current working tree against accepted base (bounded; complete manifests remain authoritative).
### M006-S01

diff --git a/scripts/_integrity.py b/scripts/_integrity.py
index 35ac72e..7d47c75 100644
--- a/scripts/_integrity.py
+++ b/scripts/_integrity.py
@@ -256,7 +256,10 @@ def errors(root, events, rm):
                     projection.add(ref, previous)
                 continue
             for ref in p.event_references(event):
-                _reference(root, ref)
+                if event["ev"] == "resolved" and ref in event["authority"]:
+                    p.read_resolution_authority(root, ref)
+                else:
+                    _reference(root, ref)
             if event["ev"] == "prompt":
                 issued[key] = envelope(root, event["envelope"], sid, event["round"])
             elif event["ev"] == "coded":
### M006-S01

diff --git a/scripts/_protocol.py b/scripts/_protocol.py
index 9c459a9..524bd77 100644
--- a/scripts/_protocol.py
+++ b/scripts/_protocol.py
@@ -5,6 +5,7 @@ from __future__ import annotations
 import copy
 import json
 import re
+import subprocess
 from datetime import datetime
 from pathlib import Path
 
@@ -247,6 +248,68 @@ def read_reference(root, ref, limit=2 * 1024 * 1024):
     return value
 
 
+def read_resolution_authority(root, ref, limit=2 * 1024 * 1024):
+    """Return the original normalized authority, never later appended text."""
+    reference(ref)
+    rel = ref["path"]
+    path = c.repo_path(root, rel)
+    message = f"immutable evidence drift: {rel}"
+    need(path.is_file() and not path.is_symlink(), message)
+    identity = c.file_key(path)
+    need(path.stat().st_size <= limit, message)
+    with path.open("rb") as stream:
+        current = stream.read(limit + 1)
+    need(len(current) <= limit and c.file_key(path) == identity, message)
+    normalized_current = current if b"\x00" in current else current.replace(b"\r\n", b"\n")
+    def current_unchanged():
+        with path.open("rb") as stream:
+            return stream.read(limit + 1) == current and c.file_key(path) == identity
+
+    if c.evidence_sha_bytes(current) == ref["sha"]:
+        need(current_unchanged(), message)
+        return normalized_current
+
+    def git(*args):
+        try:
+            return subprocess.run(
+                ["git", "--no-replace-objects", "-C", str(root), *args],
+                stdout=subprocess.PIPE,
+                stderr=subprocess.DEVNULL,
+                timeout=5,
+                check=False,
+            )
+        except (OSError, subprocess.TimeoutExpired) as exc:
+            raise ValueError(message) from exc
+
+    history = git("log", "-n", "256", "--format=%H", "HEAD", "--", rel)
+    need(history.returncode == 0, message)
+    for commit in history.stdout.splitlines():
+        if not re.fullmatch(rb"[0-9a-f]{40}|[0-9a-f]{64}", commit):
+            continue
+        object_name = commit.decode("ascii") + ":" + rel
+        size = git("cat-file", "-s", object_name)
+        if size.returncode:
+            continue
+        try:
+            if not 0 <= int(size.stdout.strip()) <= limit:
+                continue
+        except ValueError:
+            continue
+        original = git("show", object_name)
+        if original.returncode or c.evidence_sha_bytes(original.stdout) != ref["sha"]:
+            continue
+        normalized_original = (
+            original.stdout
+            if b"\x00" in original.stdout
+            else original.stdout.replace(b"\r\n", b"\n")
+        )
+        need(normalized_current.startswith(normalized_original), message)
+        need(current_unchanged(), message)
+        return normalized_original
+    raise ValueError(message)
+
+
+
 def outcome(root, ref, sid, round_no):
     value = read_reference(root, ref, 16 * 1024)
     fields = {
### M006-S01

diff --git a/scripts/hermetic_verification.py b/scripts/hermetic_verification.py
index 475312e..3eef883 100644
--- a/scripts/hermetic_verification.py
+++ b/scripts/hermetic_verification.py
@@ -18,6 +18,7 @@ COMMANDS: list[object] = [
     ["python", "-B", "scripts/ledger.py", "check"],
     ["git", "diff", "--check"],
     ["git", "diff", "--cached", "--check"],
+    ["python", "-B", "scripts/test_authority_history.py"],
     ["python", "-B", "scripts/product_verify.py"],
 ]
### M006-S01

diff --git a/scripts/ledger.py b/scripts/ledger.py
index 7f102d4..18856b2 100644
--- a/scripts/ledger.py
+++ b/scripts/ledger.py
@@ -462,6 +462,12 @@ def artifact_errors(root, events):
     errors = []
     for event in events:
         for ref in protocol.event_references(event):
+            if event["ev"] == "resolved" and ref in event["authority"]:
+                try:
+                    protocol.read_resolution_authority(root, ref)
+                except (OSError, ValueError):
+                    errors.append(f"drift: {ref['path']}")
+                continue
             path = c.repo_path(root, ref["path"])
             if path.is_symlink() or not path.is_file() or c.sha(path) != ref["sha"]:
                 errors.append(f"drift: {ref['path']}")
### M006-S01

diff --git a/scripts/prompt.py b/scripts/prompt.py
index cf79d1d..308d5ce 100644
--- a/scripts/prompt.py
+++ b/scripts/prompt.py
@@ -9,6 +9,7 @@ from pathlib import Path
 import _common as c
 import _evidence as evidence
 import _integrity as integrity
+import _protocol as protocol
 import ledger
 import roadmap
 
@@ -208,6 +209,13 @@ def _envelope(root, rm, item, current, events=None):
         context += "\n\nRecorded resolution: " + (
             resolution if isinstance(resolution, str) else _json(resolution)
         )
+    for event in events or ():
+        if event["ev"] != "resolved" or event["scope"] != item["id"]:
+            continue
+        for ref in event["authority"]:
+            original = protocol.read_resolution_authority(root, ref).decode("utf-8")
+            context += "\n\nResolution authority (original cited content): "
+            context += _ref(root, ref["path"], ref["sha"]) + "\n" + original
     return {
         "schema": "frutlups.envelope/2",
         "slice": item["id"],
@@ -430,12 +438,6 @@ def coding(root, sid, allow_dirty=False, *, preview=False, backend="manual"):
                 + "\n"
                 + evidence.read_excerpt(root, current["notes"], 2048)
             )
-    for event in events:
-        if event["ev"] == "resolved" and event["scope"] == sid:
-            authority = "Resolution authority: " + ", ".join(
-                _ref(root, ref["path"], ref["sha"]) for ref in event["authority"]
-            )
-            envelope["findings"] = authority + "\n\n" + envelope["findings"]
     values = _values(envelope)
     values["outcome_contract"] = _finish(backend, sid, current["round"])
     artifacts = []
### M006-S01

diff --git a/scripts/test_authority_history.py b/scripts/test_authority_history.py
new file mode 100644
index 0000000..2d4ebfc
--- /dev/null
+++ b/scripts/test_authority_history.py
@@ -0,0 +1,207 @@
+"""Append-only resolution authority retains its original Git-backed grant."""
+
+import os
+import subprocess
+import sys
+import tempfile
+import unittest
+from pathlib import Path
+from unittest import mock
+
+
+ROOT = Path(__file__).resolve().parents[1]
+sys.path.insert(0, str(ROOT / "scripts"))
+
+import _common as common
+import _integrity as integrity
+import _protocol as protocol
+import ledger
+import prompt
+
+
+GI
```

[Inline excerpt bounded; read the full diff page above.]

## Verification receipt

### M006-S01

Verification passed. Complete receipt: `05_governance/reviews/m006/M006-S01_r3_verification.json` (sha256 `9218aab680cce6028fec015d1f23fe5d1a4724d8d6f563b04e249e59472f4990`)

Review: `05_governance/reviews/m006/M006-S01_r3_review.md` (sha256 `1ba12cd3e98fd401257596ff3d1fce48e148f51df13d39cd63ad6c633b2b5024`)

## Prior findings

Complete findings and recovery context: `05_governance/reviews/m006/M006_b5660625d9a38229_context.md` (sha256 `b5660625d9a38229827b8fe5f725caa35d4a2e5a6d06fb546b42d88afb0ba9e7`; read as a file, starting at line 1)

Previous verification failed. Complete receipt: `05_governance/reviews/m006/M006-S01_r2_verification.json` (sha256 `deb89f91f9f6e0e9ba15ccdfce956f3fcadac4fb5cffc832a5b5b2fa3ae92aff`)

stdout tail:
```text
roadmap: ok
ledger: ok
CANDIDATE TREE 80a9b2668e6b520597de3efb6cc1ee9a22394235 files=348
COMMAND <absolute-path> check
EXIT 0: <absolute-path> check
ledger: ok
PRIVATE CANDIDATE LEDGER CHECK PASS: original Git ancestry is visible
CANDIDATE CLEAN PASS: tree=80a9b2668e6b520597de3efb6cc1ee9a22394235; private baseline=f33bdc1dea7daf6653c1bed2ecde4de255820a05; scratch-only metadata/index/objects
COMMAND <absolute-path>
EXIT 1: <absolute-path>
roadmap: ok
ledger: ok

```

stderr tail:
```text
Traceback (most recent call last):
  File "<absolute-path>", line 45, in setUp
    subprocess.run(
  File "<absolute-path>", line 571, in run
    raise CalledProcessError(retcode, process.args,
subprocess.CalledProcessError: Command '['git', '-C', '<absolute-path>', 'commit', '-q', '-m', 'Record original authority']' returned non-zero exit status 1.

======================================================================
ERROR: test_normalized_append_preserves_original_grant (__main__.AuthorityHistoryTests.test_normalized_append_preserves_original_grant)
----------------------------------------------------------------------
Traceback (most recent call last):
  File "<absolute-path>", line 45, in setUp
    subprocess.run(
  File "<absolute-path>", line 571, in run
    raise CalledProcessError(retcode, process.args,
subprocess.CalledProcessError: Command '['git', '-C', '<absolute-path>', 'commit', '-q', '-m', 'Record original authority']' returned non-zero exit status 1.

======================================================================
ERROR: test_ordinary_evidence_drift_remains_strict (__main__.AuthorityHistoryTests.test_ordinary_evidence_drift_remains_strict)
----------------------------------------------------------------------
Traceback (most recent call last):
  File "<absolute-path>", lin

[Context excerpt bounded; read the complete context file above.]

## Output

Autonomous seats return the complete report for the runner to save. In manual
mode, write only `05_governance/reviews/m006/M006_holistic_review.md` when that tool is granted, or return it for
the architect to save.

Use this exact contract:

```markdown
# Review: M006 round holistic

## Findings
| id | severity | disposition | summary |
| --- | --- | --- | --- |

## Closure Decision
Objective status: achieved | not_achieved | indeterminate
Objective evidence: one sentence tied to acceptance and the receipt

## Verdict
Verdict: pass|needs_work|blocked - next: one move
```

Return the report as the plain text of your final message, in exactly this shape, not enclosed in a code fence. Use one allowed value on each choice line. A pass requires zero open P0-P2.
Every P0-P2 finding ID must start with the affected slice ID, for example `M001-S02-H1-F1`.

To change an older finding, add `## Finding updates` before Closure Decision with columns `source | sha | id | disposition | related`. Name the original report path, its SHA-256, exact finding ID and explicit disposition; related is linked IDs or `-`. An unrelated pass closes nothing. Only a human may waive findings.
