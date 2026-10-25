use relayforge_core::{
    AppendOutcome, AttemptOutcome, CapabilityFact, CapabilityStatus, ComparisonRequest,
    ComponentValue, ContextLabel, EligibilityRequest, Event, EventKind, EventRef, FailureCategory,
    InventoryProfile, InventorySnapshot, ObservedIdentity, RequiredCapability, SelectedIdentity,
    SpanRole, Store, TaskContext, TaskLabels, ToolOutcome, WorkloadLabel, open,
};
use rusqlite::Connection;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Child, Command, Output, Stdio};
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::{Duration, Instant};

fn db() -> (tempfileless::Temp, PathBuf) {
    let dir = tempfileless::Temp::new();
    let path = dir.path().join("events.db");
    (dir, path)
}

// A stdlib-only uniquely owned temporary directory for tests.
mod tempfileless {
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    pub struct Temp(PathBuf);
    impl Temp {
        pub fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "relayforge-test-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::create_dir(&path).unwrap();
            Self(path)
        }
        pub fn path(&self) -> &Path {
            &self.0
        }
    }
    impl Drop for Temp {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
}

fn append(
    store: &mut Store,
    source: &str,
    id: &str,
    parents: &[(&str, &str)],
    kind: EventKind,
) -> AppendOutcome {
    let mut event = Event::new(source, id, kind);
    event.parents = parents
        .iter()
        .map(|(s, e)| EventRef {
            source_id: (*s).into(),
            event_id: (*e).into(),
        })
        .collect();
    store.append(event).unwrap()
}

fn wait_child(mut child: Child, label: &str) -> Output {
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        if child.try_wait().unwrap().is_some() {
            return child.wait_with_output().unwrap();
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            panic!("owned test child timed out: {label}");
        }
        thread::sleep(Duration::from_millis(10));
    }
}

fn cli_executable() -> PathBuf {
    PathBuf::from(
        std::env::var("RELAYFORGE_CLI_PATH").expect("product verifier supplies the built CLI path"),
    )
}
fn opened(store: &mut Store, owner: &str) {
    append(
        store,
        owner,
        "session-open",
        &[],
        EventKind::SessionOpened {
            session_id: "session-1".into(),
            owner_id: owner.into(),
        },
    );
}
fn task(store: &mut Store, source: &str, required: Vec<String>) {
    append(
        store,
        source,
        "task-create",
        &[(source, "session-open")],
        EventKind::TaskCreated {
            session_id: "session-1".into(),
            task_id: "task-1".into(),
            required_check_ids: required,
        },
    );
}
fn intent(store: &mut Store, source: &str, attempt: &str, predecessor: Option<&str>) {
    append(
        store,
        source,
        &format!("intent-{attempt}"),
        &[(source, "task-create")],
        EventKind::AttemptIntent {
            session_id: "session-1".into(),
            task_id: "task-1".into(),
            attempt_id: attempt.into(),
            predecessor_attempt_id: predecessor.map(str::to_owned),
            span_id: format!("span-{attempt}"),
            span_role: if predecessor.is_some() {
                SpanRole::ParentRepair
            } else {
                SpanRole::Worker
            },
            selected: SelectedIdentity {
                profile: "p1".into(),
                model: "m1".into(),
                model_version: "v1".into(),
                effort: "high".into(),
            },
        },
    );
}

#[test]
fn event_schema_rejects_unknown_fields_and_versions() {
    let raw = r#"{"schema_version":1,"source_id":"s1","event_id":"e1","unexpected":"x","kind":{"type":"SessionOpened","data":{"session_id":"ss","owner_id":"s1"}}}"#;
    assert!(serde_json::from_str::<Event>(raw).is_err());
    let mut store = Store::memory().unwrap();
    let mut event = Event::new(
        "s1",
        "e1",
        EventKind::SessionOpened {
            session_id: "ss".into(),
            owner_id: "s1".into(),
        },
    );
    event.schema_version = 2;
    assert!(matches!(
        store.append(event),
        Err(relayforge_core::StoreError::UnsupportedSchema)
    ));
}

#[test]
fn parent_scope_mismatch_and_failed_append_are_atomic() {
    let mut store = Store::memory().unwrap();
    opened(&mut store, "owner1");
    let bad = Event {
        schema_version: 1,
        source_id: "owner1".into(),
        event_id: "task-bad".into(),
        parents: vec![EventRef {
            source_id: "owner1".into(),
            event_id: "session-open".into(),
        }],
        kind: EventKind::TaskCreated {
            session_id: "different-session".into(),
            task_id: "task-1".into(),
            required_check_ids: vec![],
        },
    };
    assert!(store.append(bad).is_err());
    assert_eq!(store.show(Some("session-1"), None, 10).unwrap().len(), 1);
    assert!(matches!(
        store.append(Event::new(
            "other",
            "other-event",
            EventKind::SessionOpened {
                session_id: "session-1".into(),
                owner_id: "other".into()
            }
        )),
        Err(relayforge_core::StoreError::Conflict(_))
    ));
}

#[test]
fn identical_replay_is_noop_and_conflicting_duplicate_writes_nothing() {
    let mut store = Store::memory().unwrap();
    opened(&mut store, "owner1");
    let event = Event::new(
        "owner1",
        "session-open",
        EventKind::SessionOpened {
            session_id: "session-1".into(),
            owner_id: "owner1".into(),
        },
    );
    assert!(matches!(
        store.append(event.clone()).unwrap(),
        AppendOutcome::Replayed { .. }
    ));
    let conflict = Event::new(
        "owner1",
        "session-open",
        EventKind::SessionOpened {
            session_id: "other-session".into(),
            owner_id: "owner1".into(),
        },
    );
    assert!(matches!(
        store.append(conflict),
        Err(relayforge_core::StoreError::Conflict(
            "event_identity_reused"
        ))
    ));
    assert_eq!(store.show(Some("session-1"), None, 10).unwrap().len(), 1);
}

#[test]
fn immutable_rows_reject_mutation_and_sources_can_reuse_event_ids() {
    let (_dir, path) = db();
    let mut store = open(&path).unwrap();
    append(
        &mut store,
        "source-a",
        "shared-id",
        &[],
        EventKind::SessionOpened {
            session_id: "session-a".into(),
            owner_id: "source-a".into(),
        },
    );
    append(
        &mut store,
        "source-b",
        "shared-id",
        &[],
        EventKind::SessionOpened {
            session_id: "session-b".into(),
            owner_id: "source-b".into(),
        },
    );
    append(
        &mut store,
        "source-a",
        "task-a",
        &[("source-a", "shared-id")],
        EventKind::TaskCreated {
            session_id: "session-a".into(),
            task_id: "task-a".into(),
            required_check_ids: vec![],
        },
    );
    let connection = Connection::open(&path).unwrap();
    assert!(connection.execute("UPDATE events SET event_json='{}' WHERE source_id='source-a' AND event_id='shared-id'", []).is_err());
    assert!(
        connection
            .execute(
                "DELETE FROM events WHERE source_id='source-a' AND event_id='shared-id'",
                []
            )
            .is_err()
    );
    assert!(
        connection
            .execute(
                "UPDATE parents SET parent_event_id='missing' WHERE event_id='task-a'",
                []
            )
            .is_err()
    );
    let count: i64 = connection
        .query_row("SELECT count(*) FROM events", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 3);
}

#[test]
fn committed_intent_survives_abrupt_child_exit_and_precommit_does_not() {
    let (_dir, path) = db();
    let mut store = open(&path).unwrap();
    opened(&mut store, "owner1");
    task(&mut store, "owner1", vec![]);
    for point in ["before", "after"] {
        let child = Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "crash_boundary_helper", "--nocapture"])
            .env("RELAYFORGE_CRASH_POINT", point)
            .env("RELAYFORGE_TEST_DB", &path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let output = wait_child(child, "crash-boundary helper");
        assert!(!output.status.success());
        let connection = Connection::open(&path).unwrap();
        let count: i64 = connection
            .query_row(
                "SELECT count(*) FROM events WHERE event_id=?1 AND kind='AttemptIntent'",
                [format!("crash-{point}")],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(count, i64::from(point == "after"));
    }
}

#[test]
fn crash_boundary_helper() {
    let Ok(point) = std::env::var("RELAYFORGE_CRASH_POINT") else {
        return;
    };
    let path = PathBuf::from(std::env::var("RELAYFORGE_TEST_DB").unwrap());
    let mut store = open(&path).unwrap();
    if point == "before" {
        let connection = Connection::open(&path).unwrap();
        let tx = connection.unchecked_transaction().unwrap();
        tx.execute("INSERT INTO events(source_id,event_id,schema_version,event_json,kind,session_id,task_id,attempt_id) VALUES('owner1','crash-before',1,'{}','AttemptIntent','session-1','task-1','crash-attempt')", []).unwrap();
        std::process::exit(77);
    } else {
        let event = Event {
            schema_version: 1,
            source_id: "owner1".into(),
            event_id: "crash-after".into(),
            parents: vec![EventRef {
                source_id: "owner1".into(),
                event_id: "task-create".into(),
            }],
            kind: EventKind::AttemptIntent {
                session_id: "session-1".into(),
                task_id: "task-1".into(),
                attempt_id: "crash-attempt".into(),
                predecessor_attempt_id: None,
                span_id: "span-crash".into(),
                span_role: SpanRole::Worker,
                selected: SelectedIdentity {
                    profile: "p1".into(),
                    model: "m1".into(),
                    model_version: "v1".into(),
                    effort: "low".into(),
                },
            },
        };
        store.append(event).unwrap();
        std::process::exit(77);
    }
}

#[test]
fn explicit_recovery_protects_active_owner_and_reconcile_is_repeatable() {
    let (_dir, path) = db();
    let mut store = open(&path).unwrap();
    opened(&mut store, "oldowner");
    task(&mut store, "oldowner", vec![]);
    intent(&mut store, "oldowner", "attempt-1", None);
    let mut premature_close = Event::new(
        "oldowner",
        "close-unresolved",
        EventKind::SessionClosed {
            session_id: "session-1".into(),
            owner_id: "oldowner".into(),
        },
    );
    premature_close.parents.push(EventRef {
        source_id: "oldowner".into(),
        event_id: "session-open".into(),
    });
    assert!(matches!(
        store.append(premature_close),
        Err(relayforge_core::StoreError::Conflict(
            "session_has_unresolved_attempts"
        ))
    ));
    assert_eq!(store.show(Some("session-1"), None, 10).unwrap().len(), 3);
    assert_eq!(
        store
            .reconcile("session-1", "intruder", "restart observed")
            .unwrap(),
        0
    );
    let recovery = Event {
        schema_version: 1,
        source_id: "newowner".into(),
        event_id: "recover-1".into(),
        parents: vec![EventRef {
            source_id: "oldowner".into(),
            event_id: "session-open".into(),
        }],
        kind: EventKind::SessionRecovered {
            session_id: "session-1".into(),
            expected_owner_id: "oldowner".into(),
            actor_id: "newowner".into(),
            reason: "operator recovery".into(),
            evidence_ref: "notes/recovery".into(),
        },
    };
    store.append(recovery).unwrap();
    assert!(matches!(
        store.append(Event {
            schema_version: 1,
            source_id: "lateowner".into(),
            event_id: "stale-recovery".into(),
            parents: vec![EventRef {
                source_id: "oldowner".into(),
                event_id: "session-open".into()
            }],
            kind: EventKind::SessionRecovered {
                session_id: "session-1".into(),
                expected_owner_id: "oldowner".into(),
                actor_id: "lateowner".into(),
                reason: "stale attempt".into(),
                evidence_ref: "notes/stale".into()
            }
        }),
        Err(relayforge_core::StoreError::Conflict(_))
    ));
    assert_eq!(
        store
            .reconcile("session-1", "newowner", "restart observed")
            .unwrap(),
        1
    );
    assert_eq!(
        store
            .reconcile("session-1", "newowner", "restart observed")
            .unwrap(),
        0
    );
    let late = Event {
        schema_version: 1,
        source_id: "newowner".into(),
        event_id: "late-result".into(),
        parents: vec![EventRef {
            source_id: "oldowner".into(),
            event_id: "intent-attempt-1".into(),
        }],
        kind: EventKind::AttemptResult {
            session_id: "session-1".into(),
            task_id: "task-1".into(),
            attempt_id: "attempt-1".into(),
            outcome: AttemptOutcome::Success,
            observed: ObservedIdentity::Unknown {
                reason: "receipt unavailable".into(),
            },
            evidence_ref: None,
        },
    };
    store.append(late).unwrap();
    append(
        &mut store,
        "newowner",
        "close-resolved",
        &[("newowner", "recover-1")],
        EventKind::SessionClosed {
            session_id: "session-1".into(),
            owner_id: "newowner".into(),
        },
    );
    let showed = store.show(Some("session-1"), None, 20).unwrap();
    assert!(showed.iter().any(|e| e.event_id == "late-result"));
    assert!(showed.iter().any(|e| e.event_id == "interrupted-attempt-1"));
    assert!(showed.iter().any(|e| e.event_id == "close-resolved"));

    for (suffix, outcome, should_reject_close) in [
        ("unknown", AttemptOutcome::Unknown, true),
        ("not-run", AttemptOutcome::NotRun, false),
    ] {
        let mut state = Store::memory().unwrap();
        opened(&mut state, "owner1");
        task(&mut state, "owner1", vec![]);
        intent(&mut state, "owner1", "attempt-1", None);
        append(
            &mut state,
            "owner1",
            &format!("result-{suffix}"),
            &[("owner1", "intent-attempt-1")],
            EventKind::AttemptResult {
                session_id: "session-1".into(),
                task_id: "task-1".into(),
                attempt_id: "attempt-1".into(),
                outcome,
                observed: ObservedIdentity::Unknown {
                    reason: "attempt outcome classification".into(),
                },
                evidence_ref: None,
            },
        );
        let mut close = Event::new(
            "owner1",
            format!("close-{suffix}"),
            EventKind::SessionClosed {
                session_id: "session-1".into(),
                owner_id: "owner1".into(),
            },
        );
        close.parents.push(EventRef {
            source_id: "owner1".into(),
            event_id: format!("result-{suffix}"),
        });
        let result = state.append(close);
        if should_reject_close {
            assert!(matches!(
                result,
                Err(relayforge_core::StoreError::Conflict(
                    "session_has_unresolved_attempts"
                ))
            ));
            assert_eq!(state.show(Some("session-1"), None, 10).unwrap().len(), 4);
        } else {
            assert!(matches!(result, Ok(AppendOutcome::Appended { .. })));
            assert_eq!(state.show(Some("session-1"), None, 10).unwrap().len(), 5);
        }
    }
}

#[test]
fn two_process_claim_race_has_exactly_one_owner() {
    let (_dir, path) = db();
    open(&path).unwrap();
    let barrier = Arc::new(Barrier::new(3));
    let mut workers = Vec::new();
    for owner in ["race-a", "race-b"] {
        let path = path.clone();
        let barrier = barrier.clone();
        workers.push(thread::spawn(move || {
            barrier.wait();
            let mut child = Command::new(cli_executable())
                .args(["append", "--database", path.to_str().unwrap()])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap();
            let event = Event::new(
                owner,
                "race-open",
                EventKind::SessionOpened {
                    session_id: "race-session".into(),
                    owner_id: owner.into(),
                },
            );
            child
                .stdin
                .take()
                .unwrap()
                .write_all(serde_json::to_string(&event).unwrap().as_bytes())
                .unwrap();
            wait_child(child, "session-claim race append")
                .status
                .success()
        }));
    }
    barrier.wait();
    let successes = workers
        .into_iter()
        .map(|w| w.join().unwrap())
        .filter(|success| *success)
        .count();
    assert_eq!(successes, 1);
}

#[test]
fn acceptance_requires_latest_success_and_current_passing_checks() {
    let mut store = Store::memory().unwrap();
    opened(&mut store, "owner1");
    task(&mut store, "owner1", vec!["build".into()]);
    intent(&mut store, "owner1", "attempt-1", None);
    append(
        &mut store,
        "owner1",
        "failure-1",
        &[("owner1", "intent-attempt-1")],
        EventKind::ToolResult {
            session_id: "session-1".into(),
            task_id: "task-1".into(),
            attempt_id: "attempt-1".into(),
            tool_id: "compiler".into(),
            operation_id: "build".into(),
            result: ToolOutcome::Failure,
            failure_category: Some(FailureCategory::Infrastructure),
            evidence_ref: Some("logs/build-1".into()),
        },
    );
    let accept = Event {
        schema_version: 1,
        source_id: "owner1".into(),
        event_id: "accept-early".into(),
        parents: vec![EventRef {
            source_id: "owner1".into(),
            event_id: "task-create".into(),
        }],
        kind: EventKind::TaskAccepted {
            session_id: "session-1".into(),
            task_id: "task-1".into(),
            attempt_id: "attempt-1".into(),
            actor_id: "reviewer".into(),
            evidence_ref: "review/one".into(),
        },
    };
    assert!(store.append(accept).is_err());
    append(
        &mut store,
        "owner1",
        "success-1",
        &[("owner1", "failure-1")],
        EventKind::AttemptResult {
            session_id: "session-1".into(),
            task_id: "task-1".into(),
            attempt_id: "attempt-1".into(),
            outcome: AttemptOutcome::Success,
            observed: ObservedIdentity::Known {
                profile: "p2".into(),
                model: "m2".into(),
                model_version: "v2".into(),
                effort: "medium".into(),
            },
            evidence_ref: Some("receipts/result".into()),
        },
    );
    append(
        &mut store,
        "owner1",
        "verification-old",
        &[("owner1", "success-1")],
        EventKind::Verification {
            session_id: "session-1".into(),
            task_id: "task-1".into(),
            attempt_id: "attempt-1".into(),
            check_id: "build".into(),
            passed: true,
            evidence_ref: "checks/build".into(),
        },
    );
    append(
        &mut store,
        "owner1",
        "failure-late",
        &[("owner1", "verification-old")],
        EventKind::ToolResult {
            session_id: "session-1".into(),
            task_id: "task-1".into(),
            attempt_id: "attempt-1".into(),
            tool_id: "compiler".into(),
            operation_id: "recheck".into(),
            result: ToolOutcome::Failure,
            failure_category: Some(FailureCategory::Infrastructure),
            evidence_ref: Some("logs/recheck".into()),
        },
    );
    append(
        &mut store,
        "owner1",
        "success-late",
        &[("owner1", "failure-late")],
        EventKind::AttemptResult {
            session_id: "session-1".into(),
            task_id: "task-1".into(),
            attempt_id: "attempt-1".into(),
            outcome: AttemptOutcome::Success,
            observed: ObservedIdentity::Unknown {
                reason: "runtime identity not exposed".into(),
            },
            evidence_ref: None,
        },
    );
    let stale_accept = Event {
        schema_version: 1,
        source_id: "owner1".into(),
        event_id: "accept-stale".into(),
        parents: vec![EventRef {
            source_id: "owner1".into(),
            event_id: "success-late".into(),
        }],
        kind: EventKind::TaskAccepted {
            session_id: "session-1".into(),
            task_id: "task-1".into(),
            attempt_id: "attempt-1".into(),
            actor_id: "reviewer".into(),
            evidence_ref: "review/stale".into(),
        },
    };
    assert!(matches!(
        store.append(stale_accept),
        Err(relayforge_core::StoreError::Conflict(
            "required_verification_missing_or_failed"
        ))
    ));
    append(
        &mut store,
        "owner1",
        "verification-current",
        &[("owner1", "success-late")],
        EventKind::Verification {
            session_id: "session-1".into(),
            task_id: "task-1".into(),
            attempt_id: "attempt-1".into(),
            check_id: "build".into(),
            passed: true,
            evidence_ref: "checks/build-current".into(),
        },
    );
    append(
        &mut store,
        "owner1",
        "usage-1",
        &[("owner1", "success-late")],
        EventKind::UsageCost {
            session_id: "session-1".into(),
            task_id: "task-1".into(),
            attempt_id: "attempt-1".into(),
            usage: ComponentValue::Known {
                value: 2,
                unit: "calls".into(),
            },
            cost: ComponentValue::Unknown {
                reason: "receipt omitted cost".into(),
            },
            elapsed_ms: ComponentValue::Known {
                value: 42,
                unit: "milliseconds".into(),
            },
        },
    );
    let accepted = Event {
        schema_version: 1,
        source_id: "owner1".into(),
        event_id: "accept-1".into(),
        parents: vec![EventRef {
            source_id: "owner1".into(),
            event_id: "verification-current".into(),
        }],
        kind: EventKind::TaskAccepted {
            session_id: "session-1".into(),
            task_id: "task-1".into(),
            attempt_id: "attempt-1".into(),
            actor_id: "reviewer".into(),
            evidence_ref: "review/one".into(),
        },
    };
    assert!(matches!(
        store.append(accepted).unwrap(),
        AppendOutcome::Appended { .. }
    ));
    let after_acceptance = Event {
        schema_version: 1,
        source_id: "owner1".into(),
        event_id: "intent-after-acceptance".into(),
        parents: vec![EventRef {
            source_id: "owner1".into(),
            event_id: "task-create".into(),
        }],
        kind: EventKind::AttemptIntent {
            session_id: "session-1".into(),
            task_id: "task-1".into(),
            attempt_id: "attempt-after-acceptance".into(),
            predecessor_attempt_id: Some("attempt-1".into()),
            span_id: "span-after-acceptance".into(),
            span_role: SpanRole::Worker,
            selected: SelectedIdentity {
                profile: "p1".into(),
                model: "m1".into(),
                model_version: "v1".into(),
                effort: "low".into(),
            },
        },
    };
    assert!(matches!(
        store.append(after_acceptance),
        Err(relayforge_core::StoreError::Conflict(
            "task_already_accepted"
        ))
    ));
}

#[test]
fn acceptance_rejects_unknown_and_not_run_after_success() {
    for (suffix, outcome) in [
        ("unknown", AttemptOutcome::Unknown),
        ("not-run", AttemptOutcome::NotRun),
    ] {
        let mut store = Store::memory().unwrap();
        opened(&mut store, "owner1");
        task(&mut store, "owner1", vec!["build".into()]);
        intent(&mut store, "owner1", "attempt-1", None);
        append(
            &mut store,
            "owner1",
            "success-before-unknown",
            &[("owner1", "intent-attempt-1")],
            EventKind::AttemptResult {
                session_id: "session-1".into(),
                task_id: "task-1".into(),
                attempt_id: "attempt-1".into(),
                outcome: AttemptOutcome::Success,
                observed: ObservedIdentity::Unknown {
                    reason: "runtime identity unavailable".into(),
                },
                evidence_ref: None,
            },
        );
        append(
            &mut store,
            "owner1",
            "verification-before-unknown",
            &[("owner1", "success-before-unknown")],
            EventKind::Verification {
                session_id: "session-1".into(),
                task_id: "task-1".into(),
                attempt_id: "attempt-1".into(),
                check_id: "build".into(),
                passed: true,
                evidence_ref: "checks/build".into(),
            },
        );
        let late_result_id = format!("result-{suffix}");
        append(
            &mut store,
            "owner1",
            &late_result_id,
            &[("owner1", "verification-before-unknown")],
            EventKind::AttemptResult {
                session_id: "session-1".into(),
                task_id: "task-1".into(),
                attempt_id: "attempt-1".into(),
                outcome,
                observed: ObservedIdentity::Unknown {
                    reason: "final attempt outcome is uncertain".into(),
                },
                evidence_ref: None,
            },
        );
        let mut accepted = Event::new(
            "owner1",
            format!("accept-{suffix}"),
            EventKind::TaskAccepted {
                session_id: "session-1".into(),
                task_id: "task-1".into(),
                attempt_id: "attempt-1".into(),
                actor_id: "reviewer".into(),
                evidence_ref: "review/acceptance".into(),
            },
        );
        accepted.parents.push(EventRef {
            source_id: "owner1".into(),
            event_id: late_result_id,
        });
        assert!(matches!(
            store.append(accepted),
            Err(relayforge_core::StoreError::Conflict(
                "successful_attempt_result_missing"
            ))
        ));
        assert_eq!(store.show(Some("session-1"), None, 20).unwrap().len(), 6);
    }
}

#[test]
fn repair_attempts_preserve_predecessors_and_observed_identity() {
    let (_dir, path) = db();
    let mut store = open(&path).unwrap();
    opened(&mut store, "owner1");
    task(&mut store, "owner1", vec![]);
    intent(&mut store, "owner1", "attempt-1", None);
    append(
        &mut store,
        "owner1",
        "attempt-1-failed",
        &[("owner1", "intent-attempt-1")],
        EventKind::AttemptResult {
            session_id: "session-1".into(),
            task_id: "task-1".into(),
            attempt_id: "attempt-1".into(),
            outcome: AttemptOutcome::Failure,
            observed: ObservedIdentity::Unknown {
                reason: "execution receipt missing".into(),
            },
            evidence_ref: None,
        },
    );
    intent(&mut store, "owner1", "attempt-2", Some("attempt-1"));
    append(
        &mut store,
        "owner1",
        "attempt-2-success",
        &[("owner1", "intent-attempt-2")],
        EventKind::AttemptResult {
            session_id: "session-1".into(),
            task_id: "task-1".into(),
            attempt_id: "attempt-2".into(),
            outcome: AttemptOutcome::Success,
            observed: ObservedIdentity::Known {
                profile: "observed-profile".into(),
                model: "observed-model".into(),
                model_version: "observed-version".into(),
                effort: "low".into(),
            },
            evidence_ref: Some("receipts/attempt-2".into()),
        },
    );
    let connection = Connection::open(&path).unwrap();
    let predecessor: Option<String> = connection
        .query_row(
            "SELECT predecessor_attempt_id FROM attempts WHERE attempt_id='attempt-2'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(predecessor.as_deref(), Some("attempt-1"));
    assert_eq!(store.show(Some("session-1"), None, 20).unwrap().len(), 6);
}

fn cli(args: &[&str], input: Option<&Event>) -> (bool, serde_json::Value) {
    let mut child = Command::new(cli_executable())
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    if let Some(event) = input {
        child
            .stdin
            .take()
            .unwrap()
            .write_all(serde_json::to_string(event).unwrap().as_bytes())
            .unwrap();
    }
    let output = wait_child(child, "CLI invocation");
    (
        output.status.success(),
        serde_json::from_slice(&output.stdout).unwrap(),
    )
}
#[test]
fn actual_cli_fresh_processes_init_append_show_reconcile_accept() {
    let (_dir, path) = db();
    let db_path = path.to_str().unwrap();
    assert!(cli(&["init", "--database", db_path], None).0);
    let opened = Event::new(
        "cli-owner",
        "session-open",
        EventKind::SessionOpened {
            session_id: "cli-session".into(),
            owner_id: "cli-owner".into(),
        },
    );
    assert!(cli(&["append", "--database", db_path], Some(&opened)).0);
    let (replay_ok, replay) = cli(&["append", "--database", db_path], Some(&opened));
    assert!(replay_ok);
    assert_eq!(replay["status"], "replayed");
    let task = Event {
        schema_version: 1,
        source_id: "cli-owner".into(),
        event_id: "task-create".into(),
        parents: vec![EventRef {
            source_id: "cli-owner".into(),
            event_id: "session-open".into(),
        }],
        kind: EventKind::TaskCreated {
            session_id: "cli-session".into(),
            task_id: "cli-task".into(),
            required_check_ids: vec!["build".into()],
        },
    };
    assert!(cli(&["append", "--database", db_path], Some(&task)).0);
    let intent = Event {
        schema_version: 1,
        source_id: "cli-owner".into(),
        event_id: "intent-1".into(),
        parents: vec![EventRef {
            source_id: "cli-owner".into(),
            event_id: "task-create".into(),
        }],
        kind: EventKind::AttemptIntent {
            session_id: "cli-session".into(),
            task_id: "cli-task".into(),
            attempt_id: "attempt-1".into(),
            predecessor_attempt_id: None,
            span_id: "span-1".into(),
            span_role: SpanRole::Worker,
            selected: SelectedIdentity {
                profile: "selected".into(),
                model: "model-a".into(),
                model_version: "v1".into(),
                effort: "high".into(),
            },
        },
    };
    assert!(cli(&["append", "--database", db_path], Some(&intent)).0);
    let failure = Event {
        schema_version: 1,
        source_id: "cli-owner".into(),
        event_id: "tool-failed".into(),
        parents: vec![EventRef {
            source_id: "cli-owner".into(),
            event_id: "intent-1".into(),
        }],
        kind: EventKind::ToolResult {
            session_id: "cli-session".into(),
            task_id: "cli-task".into(),
            attempt_id: "attempt-1".into(),
            tool_id: "builder".into(),
            operation_id: "compile".into(),
            result: ToolOutcome::Failure,
            failure_category: Some(FailureCategory::Infrastructure),
            evidence_ref: Some("logs/compile".into()),
        },
    };
    assert!(cli(&["append", "--database", db_path], Some(&failure)).0);
    let result = Event {
        schema_version: 1,
        source_id: "cli-owner".into(),
        event_id: "attempt-result".into(),
        parents: vec![EventRef {
            source_id: "cli-owner".into(),
            event_id: "tool-failed".into(),
        }],
        kind: EventKind::AttemptResult {
            session_id: "cli-session".into(),
            task_id: "cli-task".into(),
            attempt_id: "attempt-1".into(),
            outcome: AttemptOutcome::Success,
            observed: ObservedIdentity::Unknown {
                reason: "runtime receipt unavailable".into(),
            },
            evidence_ref: None,
        },
    };
    assert!(cli(&["append", "--database", db_path], Some(&result)).0);
    let verification = Event {
        schema_version: 1,
        source_id: "cli-owner".into(),
        event_id: "verify-build".into(),
        parents: vec![EventRef {
            source_id: "cli-owner".into(),
            event_id: "attempt-result".into(),
        }],
        kind: EventKind::Verification {
            session_id: "cli-session".into(),
            task_id: "cli-task".into(),
            attempt_id: "attempt-1".into(),
            check_id: "build".into(),
            passed: true,
            evidence_ref: "checks/build".into(),
        },
    };
    assert!(cli(&["append", "--database", db_path], Some(&verification)).0);
    let accept = Event {
        schema_version: 1,
        source_id: "cli-owner".into(),
        event_id: "accepted".into(),
        parents: vec![EventRef {
            source_id: "cli-owner".into(),
            event_id: "verify-build".into(),
        }],
        kind: EventKind::TaskAccepted {
            session_id: "cli-session".into(),
            task_id: "cli-task".into(),
            attempt_id: "attempt-1".into(),
            actor_id: "reviewer".into(),
            evidence_ref: "reviews/acceptance".into(),
        },
    };
    assert!(cli(&["append", "--database", db_path], Some(&accept)).0);
    let (ok, shown) = cli(&["show", "--database", db_path, "--task", "cli-task"], None);
    assert!(ok);
    assert_eq!(shown["count"], 6);
    assert!(shown.to_string().contains("tool-failed"));
    let recovered = Event {
        schema_version: 1,
        source_id: "cli-recovery".into(),
        event_id: "recovery-1".into(),
        parents: vec![EventRef {
            source_id: "cli-owner".into(),
            event_id: "session-open".into(),
        }],
        kind: EventKind::SessionRecovered {
            session_id: "cli-session".into(),
            expected_owner_id: "cli-owner".into(),
            actor_id: "cli-recovery".into(),
            reason: "explicit restart recovery".into(),
            evidence_ref: "recovery/receipt".into(),
        },
    };
    assert!(cli(&["append", "--database", db_path], Some(&recovered)).0);
    assert!(
        cli(
            &[
                "reconcile",
                "--database",
                db_path,
                "--session",
                "cli-session",
                "--actor",
                "cli-recovery",
                "--reason",
                "manual check"
            ],
            None
        )
        .0
    );
}

#[test]
fn actual_cli_fresh_processes_inventory_import_show_and_binding_diagnostics() {
    let (dir, path) = db();
    let snapshot = InventorySnapshot {
        schema_version: 1,
        snapshot_id: "cli-snapshot".into(),
        revision: "revision-1".into(),
        policy_version: "policy-1".into(),
        created_at: "2026-10-25T10:00:00Z".into(),
        harness: "codex".into(),
        harness_version: "1".into(),
        adapter: "native".into(),
        adapter_version: "1".into(),
        runtime_scope: "runtime-1".into(),
        session_scope: "session-1".into(),
        configuration_fingerprint: "config-1".into(),
        workspace_fingerprint: "workspace-1".into(),
        profiles: vec![InventoryProfile {
            profile_id: "profile-1".into(),
            revision: "profile-rev-1".into(),
            native_role: "worker".into(),
            model_provider: "provider-1".into(),
            configured_model_id: "alias-1".into(),
            resolved_model_id: None,
            resolved_model_version: None,
            effort: "high".into(),
            supported_efforts: vec!["high".into()],
            capabilities: vec![CapabilityFact {
                capability_id: "cap-1".into(),
                value: "enabled".into(),
                status: CapabilityStatus::Configured,
                source_ref: "inventory/cap-1".into(),
                observed_at: "2026-10-25T10:00:00Z".into(),
                expires_at: "2026-10-26T10:00:00Z".into(),
                profile_revision: "profile-rev-1".into(),
                harness_version: "1".into(),
                configuration_fingerprint: "config-1".into(),
                runtime_scope: "runtime-1".into(),
            }],
        }],
    };
    let request = EligibilityRequest {
        schema_version: 1,
        request_id: "cli-request".into(),
        task_id: "task-1".into(),
        session_id: "session-1".into(),
        snapshot_id: "cli-snapshot".into(),
        policy_version: "policy-1".into(),
        candidate_profile_ids: vec!["profile-1".into()],
        execution_path: "native".into(),
        selected_profile_id: None,
        required_capabilities: vec![RequiredCapability {
            capability_id: "cap-1".into(),
            value: "enabled".into(),
        }],
        workload_labels: vec![WorkloadLabel {
            name: "unknown".into(),
            value: "preserved".into(),
            provenance: "user".into(),
        }],
    };
    let snapshot_path = dir.path().join("snapshot.json");
    let request_path = dir.path().join("request.json");
    std::fs::write(&snapshot_path, serde_json::to_vec(&snapshot).unwrap()).unwrap();
    std::fs::write(&request_path, serde_json::to_vec(&request).unwrap()).unwrap();
    let db_path = path.to_str().unwrap();
    let snapshot_input = snapshot_path.to_str().unwrap();
    let request_input = request_path.to_str().unwrap();
    let (ok, imported) = cli(
        &[
            "inventory-import",
            "--db",
            db_path,
            "--input",
            snapshot_input,
        ],
        None,
    );
    assert!(ok);
    assert_eq!(imported["status"], "imported");
    let (ok, replayed) = cli(
        &[
            "inventory-import",
            "--db",
            db_path,
            "--input",
            snapshot_input,
        ],
        None,
    );
    assert!(ok);
    assert_eq!(replayed["status"], "replayed");
    let (ok, shown) = cli(
        &[
            "inventory-show",
            "--db",
            db_path,
            "--snapshot",
            "cli-snapshot",
        ],
        None,
    );
    assert!(ok);
    assert_eq!(shown["snapshot"]["snapshot_id"], "cli-snapshot");
    let (ok, diagnostic) = cli(
        &[
            "eligibility",
            "--db",
            db_path,
            "--snapshot",
            "cli-snapshot",
            "--input",
            request_input,
        ],
        None,
    );
    assert!(ok);
    assert_eq!(diagnostic["binding"], "binding_unavailable");
    assert_eq!(diagnostic["eligible_profile_ids"], serde_json::json!([]));
    assert_eq!(diagnostic["outcome"], "no_eligible_candidate");
    assert_eq!(
        diagnostic["candidates"][0]["reasons"][0],
        "missing_current_state"
    );
}

#[test]
fn actual_cli_fresh_processes_context_accounting_and_comparison() {
    let (dir, path) = db();
    let db_path = path.to_str().unwrap();
    assert!(cli(&["init", "--database", db_path], None).0);
    let session = Event::new(
        "owner-a",
        "session-open",
        EventKind::SessionOpened {
            session_id: "account-session".into(),
            owner_id: "owner-a".into(),
        },
    );
    assert!(cli(&["append", "--database", db_path], Some(&session)).0);
    let profile = InventoryProfile {
        profile_id: "account-profile".into(),
        revision: "account-profile-revision".into(),
        native_role: "worker".into(),
        model_provider: "provider".into(),
        configured_model_id: "account-model-alias".into(),
        resolved_model_id: Some("account-model".into()),
        resolved_model_version: Some("v1".into()),
        effort: "high".into(),
        supported_efforts: vec!["high".into()],
        capabilities: Vec::new(),
    };
    let snapshot = InventorySnapshot {
        schema_version: 1,
        snapshot_id: "account-snapshot".into(),
        revision: "revision-1".into(),
        policy_version: "policy-1".into(),
        created_at: "2026-10-25T09:00:00Z".into(),
        harness: "codex".into(),
        harness_version: "1".into(),
        adapter: "native".into(),
        adapter_version: "1".into(),
        runtime_scope: "runtime-1".into(),
        session_scope: "account-session".into(),
        configuration_fingerprint: "config-1".into(),
        workspace_fingerprint: "workspace-1".into(),
        profiles: vec![profile],
    };
    let snapshot_path = dir.path().join("account-snapshot.json");
    std::fs::write(&snapshot_path, serde_json::to_vec(&snapshot).unwrap()).unwrap();
    let snapshot_input = snapshot_path.to_str().unwrap();
    let (ok, imported) = cli(
        &[
            "inventory-import",
            "--db",
            db_path,
            "--input",
            snapshot_input,
        ],
        None,
    );
    assert!(ok);
    assert_eq!(imported["status"], "imported");
    let task = Event {
        schema_version: 1,
        source_id: "owner-a".into(),
        event_id: "account-task-create".into(),
        parents: vec![EventRef {
            source_id: "owner-a".into(),
            event_id: "session-open".into(),
        }],
        kind: EventKind::TaskCreated {
            session_id: "account-session".into(),
            task_id: "account-task".into(),
            required_check_ids: Vec::new(),
        },
    };
    assert!(cli(&["append", "--database", db_path], Some(&task)).0);
    let context = TaskContext {
        schema_version: 1,
        context_id: "account-context".into(),
        task_id: "account-task".into(),
        session_id: "account-session".into(),
        inventory_snapshot_id: "account-snapshot".into(),
        initial_profile_id: "account-profile".into(),
        initial_profile_revision: "account-profile-revision".into(),
        policy_version: "policy-1".into(),
        observed_at: "2026-10-25T10:00:00Z".into(),
        labels: TaskLabels {
            task_type: ContextLabel::Known {
                value: "coding".into(),
                provenance: "test".into(),
            },
            difficulty: ContextLabel::Known {
                value: "medium".into(),
                provenance: "test".into(),
            },
            readiness: ContextLabel::Known {
                value: "ready".into(),
                provenance: "test".into(),
            },
        },
    };
    let context_path = dir.path().join("account-context.json");
    std::fs::write(&context_path, serde_json::to_vec(&context).unwrap()).unwrap();
    let context_input = context_path.to_str().unwrap();
    let (ok, context_result) = cli(
        &["context-import", "--db", db_path, "--input", context_input],
        None,
    );
    assert!(ok);
    assert_eq!(context_result["status"], "imported");
    let intent = Event {
        schema_version: 1,
        source_id: "owner-a".into(),
        event_id: "account-intent".into(),
        parents: vec![EventRef {
            source_id: "owner-a".into(),
            event_id: "account-task-create".into(),
        }],
        kind: EventKind::AttemptIntent {
            session_id: "account-session".into(),
            task_id: "account-task".into(),
            attempt_id: "account-attempt".into(),
            predecessor_attempt_id: None,
            span_id: "account-span".into(),
            span_role: SpanRole::Worker,
            selected: SelectedIdentity {
                profile: "account-profile".into(),
                model: "account-model-alias".into(),
                model_version: "v1".into(),
                effort: "high".into(),
            },
        },
    };
    assert!(cli(&["append", "--database", db_path], Some(&intent)).0);
    let result = Event {
        schema_version: 1,
        source_id: "owner-a".into(),
        event_id: "account-result".into(),
        parents: vec![EventRef {
            source_id: "owner-a".into(),
            event_id: "account-intent".into(),
        }],
        kind: EventKind::AttemptResult {
            session_id: "account-session".into(),
            task_id: "account-task".into(),
            attempt_id: "account-attempt".into(),
            outcome: AttemptOutcome::Success,
            observed: ObservedIdentity::Known {
                profile: "account-profile".into(),
                model: "account-model".into(),
                model_version: "v1".into(),
                effort: "high".into(),
            },
            evidence_ref: Some("receipts/account".into()),
        },
    };
    assert!(cli(&["append", "--database", db_path], Some(&result)).0);
    let cost = Event {
        schema_version: 1,
        source_id: "owner-a".into(),
        event_id: "account-cost".into(),
        parents: vec![EventRef {
            source_id: "owner-a".into(),
            event_id: "account-result".into(),
        }],
        kind: EventKind::UsageCost {
            session_id: "account-session".into(),
            task_id: "account-task".into(),
            attempt_id: "account-attempt".into(),
            usage: ComponentValue::Known {
                value: 100,
                unit: "tokens".into(),
            },
            cost: ComponentValue::Known {
                value: 10,
                unit: "micro".into(),
            },
            elapsed_ms: ComponentValue::Known {
                value: 25,
                unit: "ms".into(),
            },
        },
    };
    assert!(cli(&["append", "--database", db_path], Some(&cost)).0);
    let accepted = Event {
        schema_version: 1,
        source_id: "owner-a".into(),
        event_id: "account-accepted".into(),
        parents: vec![EventRef {
            source_id: "owner-a".into(),
            event_id: "account-result".into(),
        }],
        kind: EventKind::TaskAccepted {
            session_id: "account-session".into(),
            task_id: "account-task".into(),
            attempt_id: "account-attempt".into(),
            actor_id: "reviewer".into(),
            evidence_ref: "reviews/account".into(),
        },
    };
    assert!(cli(&["append", "--database", db_path], Some(&accepted)).0);
    let (ok, accounting) = cli(
        &["accounting", "--db", db_path, "--task", "account-task"],
        None,
    );
    assert!(ok);
    assert_eq!(accounting["report"]["tasks"][0]["accepted"], true);
    assert_eq!(
        accounting["report"]["tasks"][0]["cost"]["full_chain_total"]["value"],
        10
    );

    let comparison = ComparisonRequest {
        schema_version: 1,
        baseline_profile_id: "account-profile".into(),
        evidence_window_start: "2026-10-24T00:00:00Z".into(),
        evidence_window_end: "2026-10-26T00:00:00Z".into(),
        eligibility: EligibilityRequest {
            schema_version: 1,
            request_id: "account-compare".into(),
            task_id: "account-task".into(),
            session_id: "account-session".into(),
            snapshot_id: "account-snapshot".into(),
            policy_version: "policy-1".into(),
            candidate_profile_ids: vec!["account-profile".into()],
            execution_path: "native".into(),
            selected_profile_id: None,
            required_capabilities: Vec::new(),
            workload_labels: Vec::new(),
        },
    };
    let comparison_path = dir.path().join("account-comparison.json");
    std::fs::write(&comparison_path, serde_json::to_vec(&comparison).unwrap()).unwrap();
    let (ok, compared) = cli(
        &[
            "compare",
            "--db",
            db_path,
            "--input",
            comparison_path.to_str().unwrap(),
        ],
        None,
    );
    assert!(ok);
    assert_eq!(compared["report"]["method"], "descriptive-v1");
    assert_eq!(compared["report"]["binding"], "binding_unavailable");
    assert_eq!(compared["report"]["outcome"], "no_eligible_candidate");
}

#[test]
fn synthetic_sensitive_input_is_rejected_without_echo_or_storage() {
    let (_dir, path) = db();
    let mut store = open(&path).unwrap();
    opened(&mut store, "safeowner");
    let event = Event {
        schema_version: 1,
        source_id: "safeowner".into(),
        event_id: "sensitive".into(),
        parents: vec![EventRef {
            source_id: "safeowner".into(),
            event_id: "session-open".into(),
        }],
        kind: EventKind::SessionClosed {
            session_id: "session-1".into(),
            owner_id: "safeowner-password-fixture".into(),
        },
    };
    let mut child = Command::new(cli_executable())
        .args(["append", "--database", path.to_str().unwrap()])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(serde_json::to_string(&event).unwrap().as_bytes())
        .unwrap();
    let output = wait_child(child, "sensitive-input CLI invocation");
    assert!(!output.status.success());
    for bytes in [&output.stdout, &output.stderr] {
        assert!(
            !bytes
                .windows(b"safeowner-password-fixture".len())
                .any(|window| window == b"safeowner-password-fixture")
        );
    }
    let connection = Connection::open(&path).unwrap();
    let stored: i64 = connection
        .query_row(
            "SELECT count(*) FROM events WHERE event_json LIKE '%fixture%'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(stored, 0);
    drop(connection);
    for candidate in [
        path.clone(),
        PathBuf::from(format!("{}-wal", path.display())),
    ] {
        if let Ok(bytes) = std::fs::read(candidate) {
            assert!(
                !bytes
                    .windows(b"safeowner-password-fixture".len())
                    .any(|window| window == b"safeowner-password-fixture")
            );
        }
    }
}
