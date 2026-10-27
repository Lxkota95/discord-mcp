//! Milestone 1 — done when these pass: `cargo test --test m01_json_lines`

use discord_mcp::jsonl::run_lines;
use std::io::{self, Write};

/// Runs `run_lines` over `input` and returns everything written.
fn run(input: &str, handle: impl FnMut(&str) -> Option<String>) -> String {
    let mut output = Vec::new();
    run_lines(input.as_bytes(), &mut output, handle).expect("run_lines failed");
    String::from_utf8(output).expect("output is UTF-8")
}

#[test]
fn replies_once_per_line() {
    let out = run("hello\nworld\n", |line| Some(line.to_uppercase()));
    assert_eq!(out, "HELLO\nWORLD\n");
}

#[test]
fn handles_a_last_line_without_newline() {
    let out = run("one\ntwo", |line| Some(line.to_owned()));
    assert_eq!(out, "one\ntwo\n");
}

#[test]
fn strips_crlf_line_endings() {
    // The handler must see the line without its ending, even for Windows-style "\r\n"
    let mut seen = Vec::new();
    run("a\r\nb\r\n", |line| {
        seen.push(line.to_owned());
        None
    });
    assert_eq!(seen, ["a", "b"]);
}

#[test]
fn skips_blank_lines() {
    let mut calls = 0;
    run("\n   \nx\n\n", |_| {
        calls += 1;
        None
    });
    assert_eq!(calls, 1);
}

#[test]
fn writes_nothing_when_the_handler_returns_none() {
    assert_eq!(run("a\nb\n", |_| None), "");
}

#[test]
fn rejects_a_reply_containing_a_newline() {
    let mut output = Vec::new();
    let err = run_lines("x\n".as_bytes(), &mut output, |_| {
        Some("two\nlines".to_owned())
    })
    .expect_err("a multi-line reply must be an error");
    assert_eq!(err.kind(), io::ErrorKind::InvalidData);
    assert!(
        output.is_empty(),
        "nothing should be written for a bad reply"
    );
}

/// A writer that counts flushes, to prove each reply is flushed straight away.
#[derive(Default)]
struct CountingWriter {
    data: Vec<u8>,
    flushes: usize,
}

impl Write for CountingWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.data.extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        self.flushes += 1;
        Ok(())
    }
}

#[test]
fn flushes_after_every_reply() {
    let mut writer = CountingWriter::default();
    run_lines("a\nb\nc\n".as_bytes(), &mut writer, |l| Some(l.to_owned())).unwrap();
    assert!(
        writer.flushes >= 3,
        "flushed {} times for 3 replies",
        writer.flushes
    );
}
