"""Append-only resolution authority retains its original Git-backed grant."""

import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock


ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))

import _common as common
import _integrity as integrity
import _protocol as protocol
import ledger
import prompt


GIT_REPOSITORY_SELECTORS = (
    "GIT_DIR",
    "GIT_WORK_TREE",
    "GIT_COMMON_DIR",
    "GIT_INDEX_FILE",
    "GIT_OBJECT_DIRECTORY",
    "GIT_ALTERNATE_OBJECT_DIRECTORIES",
)


def fixture_environment(source):
    return {key: value for key, value in source.items() if key not in GIT_REPOSITORY_SELECTORS}


class AuthorityHistoryTests(unittest.TestCase):
    def setUp(self):
        self.workspace = tempfile.TemporaryDirectory(prefix="relayforge-authority-")
        self.addCleanup(self.workspace.cleanup)
        self.root = Path(self.workspace.name)
        # The parent canary supplies a private Git worktree through environment
        # selectors; retain it only for the owner lookup, then isolate this fixture.
        owner = subprocess.run(
            ["git", "-C", str(ROOT), "log", "-1", "--format=%an%x00%ae"],
            check=True,
            capture_output=True,
            text=True,
        ).stdout.strip().split("\x00")
        isolated = mock.patch.dict(os.environ, fixture_environment(os.environ), clear=True)
        isolated.start()
        self.addCleanup(isolated.stop)
        subprocess.run(["git", "init", "-q", str(self.root)], check=True, capture_output=True)
        for key, value in zip(("user.name", "user.email"), owner):
            subprocess.run(
                ["git", "-C", str(self.root), "config", key, value],
                check=True,
                capture_output=True,
            )
        self.rel = "00_brief/decisions.md"
        self.path = self.root / self.rel
        self.path.parent.mkdir()
        self.original = b"# Decisions\n\nD001: original grant.\n"
        self.path.write_bytes(self.original)
        subprocess.run(["git", "-C", str(self.root), "add", "--", self.rel], check=True, capture_output=True)
        subprocess.run(
            ["git", "-C", str(self.root), "commit", "-q", "-m", "Record original authority"],
            check=True,
            capture_output=True,
        )
        self.ref = {"path": self.rel, "sha": common.evidence_sha_bytes(self.original)}

    def test_append_only_resolution_passes_artifact_integrity(self):
        self.path.write_bytes(self.original + b"\nD002: later decision.\n")
        event = {"schema": "frutlups.ledger/2", "ev": "resolved", "authority": [self.ref]}
        self.assertEqual([], ledger.artifact_errors(self.root, [event]))
        self.assertEqual([], integrity.errors(self.root, [event], {}))
        self.assertEqual(self.original, protocol.read_resolution_authority(self.root, self.ref))

    def test_unchanged_authority_uses_current_file(self):
        self.assertEqual(self.original, protocol.read_resolution_authority(self.root, self.ref))

    def test_inherited_git_selectors_do_not_redirect_fixture(self):
        poisoned = os.environ.copy()
        unrelated = str(self.root / "unrelated.git")
        for key in GIT_REPOSITORY_SELECTORS:
            poisoned[key] = unrelated
        wrong = subprocess.run(
            ["git", "-C", str(self.root), "rev-parse", "--show-toplevel"],
            env=poisoned, capture_output=True,
        )
        self.assertNotEqual(0, wrong.returncode)
        isolated = fixture_environment(poisoned)
        with mock.patch.dict(os.environ, isolated, clear=True):
            actual = subprocess.run(
                ["git", "-C", str(self.root), "rev-parse", "--show-toplevel"],
                check=True, capture_output=True, text=True,
            )
            self.assertEqual(self.root.resolve(), Path(actual.stdout.strip()).resolve())
            self.path.write_bytes(self.original + b"\nD002: later.\n")
            self.assertEqual(self.original, protocol.read_resolution_authority(self.root, self.ref))


    def test_normalized_append_preserves_original_grant(self):
        self.path.write_bytes(self.original.replace(b"\n", b"\r\n") + b"\r\nD002: later.\r\n")
        self.assertEqual(self.original, protocol.read_resolution_authority(self.root, self.ref))

    def test_changed_old_content_and_truncation_fail_both_integrity_paths(self):
        event = {"schema": "frutlups.ledger/2", "ev": "resolved", "authority": [self.ref]}
        for content in (b"# Decisions\n\nD001: altered grant.\n\nD002: later.\n", self.original[:-1]):
            with self.subTest(content=content):
                self.path.write_bytes(content)
                with self.assertRaisesRegex(ValueError, "immutable evidence drift"):
                    protocol.read_resolution_authority(self.root, self.ref)
                self.assertEqual(["drift: " + self.rel], ledger.artifact_errors(self.root, [event]))
                self.assertTrue(integrity.errors(self.root, [event], {}))

    def test_missing_file_wrong_hash_and_wrong_path_fail(self):
        self.path.write_bytes(self.original + b"\nD002: later.\n")
        wrong_hash = {**self.ref, "sha": "0" * 64}
        other = self.root / "00_brief/other.md"
        other.write_bytes(self.path.read_bytes())
        wrong_path = {**self.ref, "path": "00_brief/other.md"}
        absent = self.root / "00_brief/unrecorded.md"
        absent.write_bytes(self.path.read_bytes())
        unrecorded = {**self.ref, "path": "00_brief/unrecorded.md"}
        for ref in (wrong_hash, wrong_path, unrecorded):
            with self.subTest(ref=ref):
                with self.assertRaisesRegex(ValueError, "immutable evidence drift"):
                    protocol.read_resolution_authority(self.root, ref)
        self.path.unlink()
        with self.assertRaisesRegex(ValueError, "immutable evidence drift"):
            protocol.read_resolution_authority(self.root, self.ref)

    def test_original_on_unreachable_branch_does_not_grant_authority(self):
        initial_branch = subprocess.run(
            ["git", "-C", str(self.root), "branch", "--show-current"],
            check=True, capture_output=True, text=True,
        ).stdout.strip()
        subprocess.run(
            ["git", "-C", str(self.root), "checkout", "-q", "--orphan", "replacement"],
            check=True, capture_output=True,
        )
        self.path.write_bytes(self.original + b"\nD002: later.\n")
        subprocess.run(["git", "-C", str(self.root), "add", "--", self.rel], check=True, capture_output=True)
        subprocess.run(
            ["git", "-C", str(self.root), "commit", "-q", "-m", "Record replacement branch"],
            check=True, capture_output=True,
        )
        subprocess.run(
            ["git", "-C", str(self.root), "branch", "-D", initial_branch],
            check=True, capture_output=True,
        )
        with self.assertRaisesRegex(ValueError, "immutable evidence drift"):
            protocol.read_resolution_authority(self.root, self.ref)

    def test_ordinary_evidence_drift_remains_strict(self):
        self.path.write_bytes(self.original + b"\nD002: later.\n")
        events = (
            {"schema": "frutlups.ledger/2", "ev": "attempt_finished", "evidence": [self.ref]},
            {
                "schema": "frutlups.ledger/2", "ev": "coded", "slice": "M001-S01",
                "round": 1, "changed": [], "outcome": self.ref,
            },
        )
        for event in events:
            with self.subTest(event=event["ev"]):
                self.assertEqual(["drift: " + self.rel], ledger.artifact_errors(self.root, [event]))
                self.assertTrue(integrity.errors(self.root, [event], {}))

    def test_prompt_receives_original_authority_without_later_decision(self):
        self.original = b"# Decisions\n" + b"A" * 2500 + b"\nD001: original grant.\n"
        self.path.write_bytes(self.original)
        subprocess.run(["git", "-C", str(self.root), "add", "--", self.rel], check=True, capture_output=True)
        subprocess.run(
            ["git", "-C", str(self.root), "commit", "-q", "-m", "Record complete authority"],
            check=True, capture_output=True,
        )
        self.ref["sha"] = common.evidence_sha_bytes(self.original)
        self.path.write_bytes(self.original + b"\nD002: later decision.\n")
        event = {"ev": "resolved", "scope": "M001-S01", "authority": [self.ref]}
        current = {"round": 2, "resolution": "Resume on recorded authority"}
        item = {
            "id": "M001-S01", "title": "Repair", "objective": "Repair", "acceptance": [],
        }
        rm = {"forbidden": [], "verification": {"full": []}}
        with mock.patch.object(prompt.evidence, "findings_context", return_value=""), mock.patch.object(
            prompt.roadmap, "effective_prefixes", return_value=["scripts/"]
        ):
            envelope = prompt._envelope(self.root, rm, item, current, [event])
        self.assertIn("D001: original grant.", envelope["findings"])
        self.assertNotIn("D002: later decision.", envelope["findings"])
        self.assertIn(self.ref["sha"], envelope["findings"])
        values = {"open_findings": envelope["findings"]}
        artifacts = []
        prompt._context(values, "open_findings", item["id"], artifacts)
        self.assertEqual(1, len(artifacts))
        self.assertIn("D001: original grant.", artifacts[0][1])
        self.assertNotIn("D002: later decision.", artifacts[0][1])
        self.assertIn(artifacts[0][0], values["open_findings"])


if __name__ == "__main__":
    suite = unittest.defaultTestLoader.loadTestsFromTestCase(AuthorityHistoryTests)
    if not suite.countTestCases():
        raise SystemExit("authority history suite is empty")
    result = unittest.TextTestRunner(verbosity=2).run(suite)
    raise SystemExit(0 if result.wasSuccessful() and not result.skipped else 1)
