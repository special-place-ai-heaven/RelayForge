"""Hermetic product checks for the native RelayForge core and CLI."""

from __future__ import annotations

import os
import re
import subprocess
import sys
import tempfile
import time
from pathlib import Path

import canary_verify as canary

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "Cargo.toml"
STAGES = ("fmt", "clippy", "unit", "integration", "doc")
REQUIRED = {
    "tests::inventory_schema_size_privacy_and_duplicate_keys_are_rejected",
    "tests::inventory_import_replay_conflict_and_reopen_are_immutable",
    "tests::eligibility_requires_separate_current_state_and_exact_confirmed_scope",
    "tests::capability_statuses_and_evidence_freshness_have_explicit_reasons",
    "tests::capability_statuses_require_applicability_before_classification",
    "accounting::tests::context_replay_conflict_late_rejection_and_legacy_age_are_explicit",
    "accounting::tests::accounting_counts_repair_cost_and_preserves_ambiguous_mixed_and_overflow_values",
    "accounting::tests::comparison_uses_eligible_exact_cohorts_wilson_baseline_and_honest_limits",
    "accounting::tests::comparison_excludes_mismatched_cohort_dimensions",
    "accounting::tests::comparison_excludes_profile_identity_mismatch",
    "accounting::tests::comparison_does_not_prefer_overlapping_wilson_intervals",
    "tests::eligibility_rejects_stale_state_dirty_workspace_and_never_substitutes_selection",
    "tests::unknown_fields_and_schema_versions_are_rejected",
    "tests::event_size_and_depth_are_bounded",
    "tests::session_open_is_a_durable_owner_claim",
    "tests::unknown_identity_and_cost_are_explicit_values",
    "acceptance_requires_latest_success_and_current_passing_checks",
    "acceptance_rejects_unknown_and_not_run_after_success",
    "actual_cli_fresh_processes_init_append_show_reconcile_accept",
    "committed_intent_survives_abrupt_child_exit_and_precommit_does_not",
    "explicit_recovery_protects_active_owner_and_reconcile_is_repeatable",
    "identical_replay_is_noop_and_conflicting_duplicate_writes_nothing",
    "immutable_rows_reject_mutation_and_sources_can_reuse_event_ids",
    "actual_cli_fresh_processes_inventory_import_show_and_binding_diagnostics",
    "actual_cli_fresh_processes_context_accounting_and_comparison",
    "parent_scope_mismatch_and_failed_append_are_atomic",
    "synthetic_sensitive_input_is_rejected_without_echo_or_storage",
    "two_process_claim_race_has_exactly_one_owner",
    "repair_attempts_preserve_predecessors_and_observed_identity",
}


def cargo(*args: str) -> list[str]:
    return ["cargo", "+1.99.1", *args]


def run_product(root: Path, scratch: Path, stages: tuple[str, ...] = STAGES) -> None:
    root = root.resolve()
    scratch = scratch.resolve()
    if scratch == root or root in scratch.parents:
        raise canary.VerificationError("product verification scratch must be outside source tree")
    env = os.environ.copy()
    env.update({"CARGO_TARGET_DIR": str(scratch / "cargo-target"), "CARGO_NET_OFFLINE": "true", "CARGO_BUILD_JOBS": "2"})
    deadline = time.monotonic() + canary.TIMEOUT_SECONDS
    if stages == STAGES:
        canary._run([sys.executable, "-B", "scripts/ci_contract_check.py"], cwd=root, env=env, deadline=deadline, scratch=scratch)
    rustc = canary._run(["rustc", "+1.99.1", "-vV"], cwd=root, env=env, deadline=deadline, scratch=scratch)
    hosts = re.findall(r"^host: (\S+)$", rustc.stdout, re.MULTILINE)
    if not rustc.stdout.startswith("rustc 1.99.1 ") or len(hosts) != 1:
        raise canary.VerificationError("pinned rustc did not report one native host target")
    target = hosts[0]
    suffix = ".exe" if os.name == "nt" else ""
    env["RELAYFORGE_CLI_PATH"] = str(Path(env["CARGO_TARGET_DIR"]) / target / "debug" / f"relayforge{suffix}")
    print(f"PRODUCT NATIVE TARGET {target}; CLI suffix={suffix!r}", flush=True)
    for command in (cargo("--version"), ["rustfmt", "+1.99.1", "--version"], cargo("clippy", "--version")):
        canary._run(command, cwd=root, env=env, deadline=deadline, scratch=scratch)
    installed = canary._run(["rustup", "target", "list", "--installed", "--toolchain", "1.99.1"], cwd=root, env=env, deadline=deadline, scratch=scratch)
    if target not in installed.stdout.split():
        raise canary.VerificationError(f"native Rust target is not installed: {target}")

    graph = canary._run(cargo("tree", "--manifest-path", str(root / "Cargo.toml"), "--workspace", "--locked", "--offline", "--target", target), cwd=root, env=env, deadline=deadline, scratch=scratch)
    if "rusqlite v0.40.0" not in graph.stdout or "libsqlite3-sys v0.38.0" not in graph.stdout:
        raise canary.VerificationError("product dependency graph does not lock rusqlite 0.40.0 and libsqlite3-sys 0.38.0")
    features = canary._run(cargo("tree", "--manifest-path", str(root / "Cargo.toml"), "--workspace", "--locked", "--offline", "--target", target, "--edges", "features", "-i", "libsqlite3-sys"), cwd=root, env=env, deadline=deadline, scratch=scratch)
    if 'feature "bundled"' not in features.stdout:
        raise canary.VerificationError("product SQLite dependency does not enable bundled")
    if stages == STAGES:
        canary._run(cargo("build", "--manifest-path", str(MANIFEST), "--workspace", "--all-targets", "--target", target, "--locked", "--offline"), cwd=root, env=env, deadline=deadline, scratch=scratch)
    if "fmt" in stages:
        canary._run(cargo("fmt", "--manifest-path", str(MANIFEST), "--all", "--", "--check"), cwd=root, env=env, deadline=deadline, scratch=scratch)
    if "clippy" in stages:
        canary._run(cargo("clippy", "--manifest-path", str(MANIFEST), "--workspace", "--all-targets", "--target", target, "--locked", "--offline", "--", "-D", "warnings"), cwd=root, env=env, deadline=deadline, scratch=scratch)
    for stage, selector, required in (
        ("unit", ["--workspace", "--lib"], {name for name in REQUIRED if name.startswith(("tests::", "accounting::tests::"))}),
        ("integration", ["--manifest-path", str(root / "08_pkg/relayforge-core/Cargo.toml"), "--test", "storage_contract"], {name for name in REQUIRED if not name.startswith(("tests::", "accounting::tests::"))}),
        ("doc", ["--workspace", "--doc"], set()),
    ):
        if stage not in stages:
            continue
        if stage == "integration":
            canary._run(cargo("build", "--manifest-path", str(MANIFEST), "--bin", "relayforge", "--target", target, "--locked", "--offline"), cwd=root, env=env, deadline=deadline, scratch=scratch)
            sqlite = canary._run([env["RELAYFORGE_CLI_PATH"], "sqlite-version"], cwd=root, env=env, deadline=deadline, scratch=scratch)
            if '"sqlite_version":"3.53.1"' not in sqlite.stdout.replace(" ", ""):
                raise canary.VerificationError("bundled runtime does not report SQLite 3.53.1")
            print("PRODUCT SQLITE RUNTIME 3.53.1", flush=True)
        base = cargo("test", *selector, "--target", target, "--locked", "--offline")
        discovered = canary._run([*base, "--", "--list"], cwd=root, env=env, deadline=deadline, scratch=scratch)
        found = canary.guard_discovery(discovered.stdout + discovered.stderr, required, f"product {stage}")
        print(f"PRODUCT {stage.upper()} DISCOVERY discovered={len(found)} required={len(required)}", flush=True)
        extra = ["--", "--nocapture", "--test-threads=1"] if stage != "doc" else []
        executed = canary._run([*base, *extra], cwd=root, env=env, deadline=deadline, scratch=scratch)
        counts = canary.parse_execution(executed.stdout + executed.stderr, found, f"product {stage}")
        print(f"PRODUCT {stage.upper()} EXECUTION discovered={len(found)} passed={counts[0]} failed={counts[1]} ignored={counts[2]}", flush=True)


def main() -> int:
    try:
        if "--focused" in sys.argv[1:]:
            with tempfile.TemporaryDirectory(prefix="relayforge-product-focused-") as name:
                scratch = Path(name).resolve()
                if not scratch.is_relative_to(Path(tempfile.gettempdir()).resolve()) or scratch == Path(tempfile.gettempdir()).resolve():
                    raise canary.VerificationError("focused scratch escaped system temp")
                run_product(ROOT, scratch)
            print("FOCUSED PRODUCT VERIFICATION PASS", flush=True)
            return 0
        if "--stage" in sys.argv[1:]:
            args = sys.argv[1:]
            index = args.index("--stage")
            if index + 1 >= len(args) or args[index + 1] not in STAGES:
                raise canary.VerificationError(f"stage must be one of {STAGES}")
            stage = args[index + 1]
            with tempfile.TemporaryDirectory(prefix=f"relayforge-product-{stage}-") as name:
                run_product(ROOT, Path(name), (stage,))
            print(f"PRODUCT STAGE {stage} PASS", flush=True)
            return 0
        if os.environ.get(canary.INNER_GUARD) == "1":
            canary.verify_inner()
            scratch = Path(os.environ["RELAYFORGE_CANARY_SCRATCH"]).resolve()
            product_scratch = scratch / "product"
            product_scratch.mkdir()
            canary._owned_scratch(product_scratch, scratch, ROOT)
            run_product(ROOT, product_scratch)
            print("INNER CANARY AND PRODUCT VERIFICATION PASS", flush=True)
            return 0
        with tempfile.TemporaryDirectory(prefix="relayforge-product-full-") as name:
            canary._candidate_run(Path(name).resolve(), report_blobs=False)
        print("CLEAN-CANDIDATE PRODUCT VERIFICATION PASS", flush=True)
        return 0
    except (OSError, ValueError, canary.VerificationError, subprocess.SubprocessError) as exc:
        print(f"product verification failed: {exc}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
