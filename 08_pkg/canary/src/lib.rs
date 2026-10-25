//! Minimal durable-event canary for RelayForge.
//!
//! ```
//! assert!(relayforge_canary::completion_requires_both(true, true));
//! assert!(!relayforge_canary::completion_requires_both(true, false));
//! ```

use rusqlite::{Connection, OptionalExtension, Transaction, params};
use std::path::Path;

const SCHEMA: &str = "
PRAGMA foreign_keys = ON;
CREATE TABLE IF NOT EXISTS events (
    event_id TEXT PRIMARY KEY,
    schema_version INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    session_id TEXT NOT NULL,
    task_id TEXT NOT NULL,
    attempt_id TEXT,
    kind TEXT NOT NULL,
    result TEXT NOT NULL,
    identity TEXT NOT NULL,
    cost TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS capability_facts (
    capability_id TEXT PRIMARY KEY,
    status TEXT NOT NULL,
    evidence TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS observations (
    observation_id TEXT PRIMARY KEY,
    metric TEXT NOT NULL,
    value TEXT NOT NULL
);
";

#[derive(Debug)]
pub struct Snapshot {
    pub state: &'static str,
    pub result: &'static str,
    pub identity: &'static str,
    pub cost: &'static str,
    pub complete: bool,
    pub event_ids: Vec<String>,
    pub event_count: usize,
}

pub fn completion_requires_both(verified: bool, accepted: bool) -> bool {
    verified && accepted
}

pub fn open(path: &Path) -> rusqlite::Result<Connection> {
    let connection = Connection::open(path)?;
    connection.execute_batch(SCHEMA)?;
    Ok(connection)
}

fn add_event(
    tx: &Transaction<'_>,
    id: &str,
    attempt: Option<&str>,
    kind: &str,
    result: &str,
    identity: &str,
    cost: &str,
) -> rusqlite::Result<()> {
    tx.execute(
        "INSERT INTO events(
             event_id, schema_version, created_at, session_id, task_id, attempt_id,
             kind, result, identity, cost
         ) VALUES (
             ?1, 1, strftime('%Y-%m-%dT%H:%M:%fZ', 'now'), 'session-001', 'task-001',
             ?2, ?3, ?4, ?5, ?6
         )
         ON CONFLICT(event_id) DO NOTHING",
        params![id, attempt, kind, result, identity, cost],
    )?;
    Ok(())
}

pub fn seed(connection: &mut Connection) -> rusqlite::Result<()> {
    let tx = connection.transaction()?;
    add_event(
        &tx,
        "session.created",
        None,
        "session",
        "recorded",
        "unknown",
        "unknown",
    )?;
    add_event(
        &tx,
        "task.created",
        None,
        "task",
        "recorded",
        "unknown",
        "unknown",
    )?;
    tx.execute(
        "INSERT INTO capability_facts VALUES ('tool-observation', 'configured', 'synthetic fixture')
         ON CONFLICT(capability_id) DO NOTHING",
        [],
    )?;
    tx.execute(
        "INSERT INTO observations VALUES ('observation-001', 'tool-call-result', 'failure')
         ON CONFLICT(observation_id) DO NOTHING",
        [],
    )?;
    tx.commit()
}

pub fn commit_intent(connection: &mut Connection) -> rusqlite::Result<()> {
    let tx = connection.transaction()?;
    add_event(
        &tx,
        "attempt.intent",
        Some("attempt-001"),
        "dispatch_intent",
        "pending",
        "unknown",
        "unknown",
    )?;
    tx.commit()
}

pub fn append_failure(connection: &mut Connection) -> rusqlite::Result<()> {
    let tx = connection.transaction()?;
    add_event(
        &tx,
        "tool.failure",
        Some("attempt-001"),
        "tool_call",
        "failure",
        "unknown",
        "unknown",
    )?;
    tx.commit()
}

pub fn append_verification(connection: &mut Connection) -> rusqlite::Result<()> {
    let tx = connection.transaction()?;
    add_event(
        &tx,
        "task.verified",
        Some("attempt-001"),
        "verification",
        "passed",
        "unknown",
        "unknown",
    )?;
    tx.commit()
}

pub fn append_acceptance(connection: &mut Connection) -> rusqlite::Result<()> {
    let tx = connection.transaction()?;
    add_event(
        &tx,
        "task.accepted",
        Some("attempt-001"),
        "acceptance",
        "accepted",
        "unknown",
        "unknown",
    )?;
    tx.commit()
}

pub fn reconcile(connection: &mut Connection) -> rusqlite::Result<Snapshot> {
    let tx = connection.transaction()?;
    let has_intent = tx
        .query_row(
            "SELECT 1 FROM events WHERE event_id = 'attempt.intent'",
            [],
            |_| Ok(()),
        )
        .optional()?
        .is_some();
    let has_result = tx
        .query_row(
            "SELECT 1 FROM events WHERE event_id = 'tool.failure'",
            [],
            |_| Ok(()),
        )
        .optional()?
        .is_some();
    if has_intent && !has_result {
        add_event(
            &tx,
            "attempt.interrupted",
            Some("attempt-001"),
            "reconciliation",
            "interrupted",
            "unknown",
            "unknown",
        )?;
    }
    tx.commit()?;
    snapshot(connection)
}

pub fn snapshot(connection: &Connection) -> rusqlite::Result<Snapshot> {
    let has_intent = event_exists(connection, "attempt.intent")?;
    let has_interrupted = event_exists(connection, "attempt.interrupted")?;
    let has_failure = event_exists(connection, "tool.failure")?;
    let verified = event_exists(connection, "task.verified")?;
    let accepted = event_exists(connection, "task.accepted")?;
    let complete = completion_requires_both(verified, accepted);
    let state = if complete {
        "completed"
    } else if has_intent && !has_failure && has_interrupted {
        "interrupted"
    } else {
        "active"
    };
    let result = if has_failure {
        "failure"
    } else if has_intent && has_interrupted {
        "unknown"
    } else {
        "pending"
    };
    let mut statement = connection.prepare("SELECT event_id FROM events ORDER BY event_id")?;
    let event_ids = statement
        .query_map([], |row| row.get(0))?
        .collect::<rusqlite::Result<Vec<String>>>()?;
    Ok(Snapshot {
        state,
        result,
        identity: "unknown",
        cost: "unknown",
        event_count: event_ids.len(),
        event_ids,
        complete,
    })
}

fn event_exists(connection: &Connection, id: &str) -> rusqlite::Result<bool> {
    connection
        .query_row("SELECT 1 FROM events WHERE event_id = ?1", [id], |_| Ok(()))
        .optional()
        .map(|value| value.is_some())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::params;

    #[test]
    fn completion_requires_verification_and_acceptance() {
        assert!(!completion_requires_both(false, false));
        assert!(!completion_requires_both(true, false));
        assert!(!completion_requires_both(false, true));
        assert!(completion_requires_both(true, true));
    }

    #[test]
    fn capability_facts_are_distinct_from_observations() {
        let mut connection = open(Path::new(":memory:")).unwrap();
        seed(&mut connection).unwrap();
        let facts: i64 = connection
            .query_row("SELECT count(*) FROM capability_facts", [], |row| {
                row.get(0)
            })
            .unwrap();
        let observations: i64 = connection
            .query_row("SELECT count(*) FROM observations", [], |row| row.get(0))
            .unwrap();
        assert_eq!((facts, observations), (1, 1));
        let fact_status: String = connection
            .query_row("SELECT status FROM capability_facts", [], |row| row.get(0))
            .unwrap();
        let observation: String = connection
            .query_row("SELECT value FROM observations", [], |row| row.get(0))
            .unwrap();
        assert_eq!(fact_status, "configured");
        assert_eq!(observation, "failure");
    }

    #[test]
    fn duplicate_events_do_not_inflate_counts() {
        let mut connection = open(Path::new(":memory:")).unwrap();
        seed(&mut connection).unwrap();
        commit_intent(&mut connection).unwrap();
        commit_intent(&mut connection).unwrap();
        assert_eq!(snapshot(&connection).unwrap().event_count, 3);
    }

    #[test]
    fn completion_state_is_false_until_both_events_exist() {
        let mut connection = open(Path::new(":memory:")).unwrap();
        seed(&mut connection).unwrap();
        append_verification(&mut connection).unwrap();
        assert!(!snapshot(&connection).unwrap().complete);
        append_acceptance(&mut connection).unwrap();
        assert!(snapshot(&connection).unwrap().complete);
    }

    #[test]
    fn event_identity_is_unique_even_when_replayed() {
        let mut connection = open(Path::new(":memory:")).unwrap();
        seed(&mut connection).unwrap();
        let tx = connection.transaction().unwrap();
        add_event(
            &tx,
            "probe.event",
            None,
            "probe",
            "unknown",
            "unknown",
            "unknown",
        )
        .unwrap();
        add_event(
            &tx,
            "probe.event",
            None,
            "probe",
            "unknown",
            "unknown",
            "unknown",
        )
        .unwrap();
        tx.commit().unwrap();
        let count: i64 = connection
            .query_row(
                "SELECT count(*) FROM events WHERE event_id = ?1",
                params!["probe.event"],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 1);
    }
}
