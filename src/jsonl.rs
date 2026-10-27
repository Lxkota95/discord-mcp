//! Milestone 1: newline-delimited messages over any reader and writer.
//!
//! MCP's stdio transport sends one JSON message per line. This module only knows about lines; it
//! doesn't care what's in them, which keeps it easy to test with in-memory buffers.

use std::io::{self, BufRead, Write};

/// Reads `input` line by line and passes each non-blank line (without its line ending) to
/// `handle`. Whatever `handle` returns is written to `output` as one line and flushed straight
/// away, so the other side sees it immediately. Returns when `input` ends.
///
/// # Errors
///
/// Any I/O error from reading or writing, and [`io::ErrorKind::InvalidData`] if `handle` returns
/// text containing a newline: the stdio transport forbids embedded newlines, because the newline is
/// the message separator.
#[allow(unused_variables, unused_mut)] // remove this line when you implement it
pub fn run_lines<R: BufRead, W: Write>(
    input: R,
    mut output: W,
    mut handle: impl FnMut(&str) -> Option<String>,
) -> io::Result<()> {
    todo!("milestone 1: see learn/01-json-lines.md")
}
