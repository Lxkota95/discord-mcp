//! Transcript tests: a `.transcript` file is a recorded conversation with the server.
//!
//!     # comments and blank lines are ignored
//!     > {"jsonrpc":"2.0","id":1,"method":"server/discover",...}   a line the client sends
//!     < {"jsonrpc":"2.0","id":1,"result":{...}}                    the reply the server must send
//!     < none                                                       ...or: no reply at all
//!
//! Replies are compared as JSON values, so key order and spacing don't matter. In an expected reply,
//! the string `"*"` matches any value: used where the spec doesn't fix the exact text (e.g. most
//! error messages), so your wording is free.

use discord_mcp::mcp::{Server, ServerInfo};
use serde_json::Value;
use std::path::Path;

pub fn test_server() -> Server {
    Server::new(ServerInfo {
        name: "discord-mcp".into(),
        version: "0.1.0".into(),
    })
}

/// Plays `tests/transcripts/<name>.transcript` against a fresh server and checks every reply.
pub fn check_transcript(name: &str) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/transcripts")
        .join(format!("{name}.transcript"));
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("reading {}: {e}", path.display()));
    let mut server = test_server();
    let mut reply: Option<String> = None;
    let mut sent = String::new();

    for (n, line) in text.lines().enumerate() {
        let n = n + 1;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(request) = line.strip_prefix("> ") {
            sent = request.to_owned();
            reply = server.handle_line(request);
        } else if let Some(expected) = line.strip_prefix("< ") {
            let actual = reply.take();
            if expected == "none" {
                assert!(
                    actual.is_none(),
                    "{name}:{n}: expected no reply to\n  {sent}\ngot\n  {actual:?}"
                );
                continue;
            }
            let actual = actual.unwrap_or_else(|| panic!("{name}:{n}: no reply to\n  {sent}"));
            let expected: Value = serde_json::from_str(expected)
                .unwrap_or_else(|e| panic!("{name}:{n}: expected reply isn't JSON: {e}"));
            let actual_value: Value = serde_json::from_str(&actual).unwrap_or_else(|e| {
                panic!("{name}:{n}: the server's reply isn't JSON ({e}):\n  {actual}")
            });
            assert!(
                matches(&expected, &actual_value),
                "{name}:{n}: wrong reply to\n  {sent}\nexpected (\"*\" matches anything)\n  {expected}\ngot\n  {actual}"
            );
        } else {
            panic!("{name}:{n}: lines must start with '> ', '< ' or '#'");
        }
    }
}

/// JSON equality, except that an expected `"*"` matches any value.
fn matches(expected: &Value, actual: &Value) -> bool {
    match (expected, actual) {
        (Value::String(s), _) if s == "*" => true,
        (Value::Object(e), Value::Object(a)) => {
            e.len() == a.len()
                && e.iter()
                    .all(|(k, v)| a.get(k).is_some_and(|av| matches(v, av)))
        }
        (Value::Array(e), Value::Array(a)) => {
            e.len() == a.len() && e.iter().zip(a).all(|(ev, av)| matches(ev, av))
        }
        _ => expected == actual,
    }
}
