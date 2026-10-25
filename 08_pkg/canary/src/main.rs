use relayforge_canary::{
    append_acceptance, append_failure, append_verification, commit_intent, open, reconcile, seed,
    snapshot,
};
use rusqlite::Connection;
use std::env;
use std::path::PathBuf;
use std::process;

fn database(args: &[String]) -> Result<PathBuf, String> {
    args.windows(2)
        .find(|pair| pair[0] == "--database")
        .map(|pair| PathBuf::from(&pair[1]))
        .ok_or_else(|| "missing --database path".to_owned())
}

fn print_snapshot(connection: &Connection) -> rusqlite::Result<()> {
    let state = snapshot(connection)?;
    println!(
        "{{\"state\":\"{}\",\"result\":\"{}\",\"identity\":\"{}\",\"cost\":\"{}\",\"complete\":{},\"event_count\":{},\"event_ids\":[{}]}}",
        state.state,
        state.result,
        state.identity,
        state.cost,
        state.complete,
        state.event_count,
        state
            .event_ids
            .iter()
            .map(|id| format!("\"{id}\""))
            .collect::<Vec<_>>()
            .join(",")
    );
    Ok(())
}

fn run() -> Result<(), String> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.first().map(String::as_str) == Some("sqlite-version") {
        let connection = Connection::open_in_memory().map_err(|error| error.to_string())?;
        let version: String = connection
            .query_row("SELECT sqlite_version()", [], |row| row.get(0))
            .map_err(|error| error.to_string())?;
        println!("{version}");
        return Ok(());
    }
    if args.first().map(String::as_str) != Some("canary") {
        return Err("usage: relayforge canary <command> --database PATH".to_owned());
    }
    let command = args.get(1).map(String::as_str).unwrap_or_default();
    let path = database(&args)?;
    let mut connection = open(&path).map_err(|error| error.to_string())?;
    match command {
        "seed" => seed(&mut connection),
        "crash-after-intent" => {
            commit_intent(&mut connection).map_err(|error| error.to_string())?;
            process::abort();
        }
        "reconcile" => {
            reconcile(&mut connection).map_err(|error| error.to_string())?;
            return print_snapshot(&connection).map_err(|error| error.to_string());
        }
        "record-failure" => append_failure(&mut connection),
        "verify" => append_verification(&mut connection),
        "accept" => append_acceptance(&mut connection),
        _ => return Err(format!("unknown canary command: {command}")),
    }
    .map_err(|error| error.to_string())?;
    print_snapshot(&connection).map_err(|error| error.to_string())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("relayforge: {error}");
        process::exit(2);
    }
}
