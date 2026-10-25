use relayforge_canary::{open, seed, snapshot};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn bundled_runtime_reports_sqlite_3531() {
    let connection = open(PathBuf::from(":memory:").as_path()).unwrap();
    let version: String = connection
        .query_row("SELECT sqlite_version()", [], |row| row.get(0))
        .unwrap();
    assert_eq!(version, "3.53.1");
}

#[test]
fn committed_seed_survives_reopen_with_stable_event_ids() {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let scratch = std::env::var_os("VERIFICATION_SCRATCH")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    let database_dir = scratch.join(format!("relayforge-canary-test-{nonce}"));
    std::fs::create_dir(&database_dir).unwrap();
    let path = database_dir.join("events.db");
    {
        let mut connection = open(&path).unwrap();
        seed(&mut connection).unwrap();
    }
    {
        let connection = open(&path).unwrap();
        let current = snapshot(&connection).unwrap();
        assert_eq!(current.event_count, 2);
        assert_eq!(current.event_ids, ["session.created", "task.created"]);
        let (schema_version, created_at): (i64, String) = connection
            .query_row(
                "SELECT schema_version, created_at FROM events ORDER BY event_id LIMIT 1",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(schema_version, 1);
        assert!(!created_at.is_empty());
    }
    std::fs::remove_file(path).unwrap();
    std::fs::remove_dir(database_dir).unwrap();
}
