use relayforge_core::{Event, open, parse_event_bytes};
use rusqlite::Connection;
use serde_json::json;
use std::env;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process;

const MAX_STDIN: usize = 65_536;

fn option(args: &[String], name: &str) -> Result<Option<String>, String> {
    let mut found = None;
    let mut index = 0;
    while index < args.len() {
        if args[index] == name {
            if found.is_some() || index + 1 == args.len() {
                return Err("invalid_arguments".into());
            }
            found = Some(args[index + 1].clone());
            index += 2;
        } else {
            index += 1;
        }
    }
    Ok(found)
}
fn required(args: &[String], name: &str) -> Result<String, String> {
    option(args, name)?.ok_or_else(|| "missing_required_option".into())
}
fn check_options(args: &[String], allowed: &[&str]) -> Result<(), String> {
    let mut index = 0;
    while index < args.len() {
        if !allowed.contains(&args[index].as_str())
            || index + 1 == args.len()
            || args[index + 1].starts_with("--")
        {
            return Err("invalid_arguments".into());
        }
        index += 2;
    }
    Ok(())
}
fn database(args: &[String]) -> Result<PathBuf, String> {
    Ok(PathBuf::from(required(args, "--database")?))
}
fn read_event() -> Result<Event, String> {
    let mut input = Vec::new();
    io::stdin()
        .take((MAX_STDIN + 1) as u64)
        .read_to_end(&mut input)
        .map_err(|_| "input_read_failed")?;
    if input.is_empty() || input.len() > MAX_STDIN {
        return Err("event_size_out_of_range".into());
    }
    parse_event_bytes(&input).map_err(|e| e.to_string())
}
fn run() -> Result<serde_json::Value, String> {
    let args: Vec<String> = env::args().skip(1).collect();
    let command = args.first().map(String::as_str).unwrap_or("");
    match command {
        "sqlite-version" => {
            let connection = Connection::open_in_memory().map_err(|_| "storage_error")?;
            let version: String = connection
                .query_row("SELECT sqlite_version()", [], |r| r.get(0))
                .map_err(|_| "storage_error")?;
            Ok(json!({"ok":true,"sqlite_version":version}))
        }
        "init" => {
            check_options(&args[1..], &["--database"])?;
            let path = database(&args[1..])?;
            let _store = open(&path).map_err(|e| e.to_string())?;
            Ok(json!({"ok":true,"command":"init","schema_version":1}))
        }
        "append" => {
            check_options(&args[1..], &["--database"])?;
            let path = database(&args[1..])?;
            let event = read_event()?;
            let mut store = open(&path).map_err(|e| e.to_string())?;
            let result = store.append(event).map_err(|e| e.to_string())?;
            match result {
                relayforge_core::AppendOutcome::Appended { sequence } => {
                    Ok(json!({"ok":true,"status":"appended","sequence":sequence}))
                }
                relayforge_core::AppendOutcome::Replayed { sequence } => {
                    Ok(json!({"ok":true,"status":"replayed","sequence":sequence}))
                }
            }
        }
        "show" => {
            check_options(
                &args[1..],
                &["--database", "--session", "--task", "--limit"],
            )?;
            let path = database(&args[1..])?;
            let session = option(&args[1..], "--session")?;
            let task = option(&args[1..], "--task")?;
            let limit = option(&args[1..], "--limit")?
                .map(|v| v.parse::<usize>().map_err(|_| "invalid_limit"))
                .transpose()?
                .unwrap_or(100);
            let store = open(&path).map_err(|e| e.to_string())?;
            let events = store
                .show(session.as_deref(), task.as_deref(), limit)
                .map_err(|e| e.to_string())?;
            Ok(json!({"ok":true,"count":events.len(),"events":events}))
        }
        "reconcile" => {
            check_options(
                &args[1..],
                &["--database", "--session", "--actor", "--reason"],
            )?;
            let path = database(&args[1..])?;
            let session = required(&args[1..], "--session")?;
            let actor = required(&args[1..], "--actor")?;
            let reason = required(&args[1..], "--reason")?;
            let mut store = open(&path).map_err(|e| e.to_string())?;
            let appended = store
                .reconcile(&session, &actor, &reason)
                .map_err(|e| e.to_string())?;
            Ok(json!({"ok":true,"appended":appended,"session_id":session}))
        }
        _ => Err(
            "usage: relayforge <init|append|show|reconcile|sqlite-version> --database PATH".into(),
        ),
    }
}
fn main() {
    match run() {
        Ok(value) => println!("{value}"),
        Err(code) => {
            println!("{}", json!({"ok":false,"error":code}));
            process::exit(2);
        }
    }
}
