# Milestone 1 — JSON lines over stdin/stdout

**Goal:** implement `run_lines` in `src/jsonl.rs`: read a stream line by line, hand each line to a
function, and write that function's replies back — the transport every MCP stdio server is built on.

**Time:** 1–2 hours. **Done when:** `cargo test --test m01_json_lines` passes.

**Start:** a branch for this milestone: `git switch main && git pull && git switch -c m01`.
Offline, with `m00b` not merged yet? Branch from it instead: `git switch -c m01 m00b`.

## Why this matters

Claude Code starts an MCP server as a child process and talks to it over the process's **stdin**
and **stdout**: one JSON message per line, in both directions. Logging goes to **stderr**, because
anything else on stdout would corrupt the conversation. (Read the official rules:
[`reference/mcp/2026-07-28/basic-transports-stdio.mdx`](reference/mcp/2026-07-28/basic-transports-stdio.mdx).)

This milestone ignores JSON entirely: a line is a line. Keeping the layers apart is what makes
each one easy to test.

## Concepts

**Readers and writers.** `std::io::Read` and `Write` are traits for byte streams. `BufRead` adds
buffering and line-oriented reading: `lines()` gives an iterator of `io::Result<String>`, each
without its trailing `\n` (or `\r\n`). Stdin's lock (`io::stdin().lock()`) is a `BufRead`.
- `rustup doc std::io::BufRead` — look at `lines()` and its note about line endings
- `rustup doc std::io::Write` — `write!`/`writeln!` and `flush()`
- Rust Book ch. 12 (an I/O project): `rustup doc --book`, then chapter 12

**Generic over the stream.** `run_lines<R: BufRead, W: Write>` works with *any* reader and writer.
In production that's stdin and stdout; in tests it's a byte slice (`&[u8]` implements `BufRead`) and
a `Vec<u8>` (implements `Write`). No real terminal, no subprocess: fast, deterministic tests.
- Rust Book ch. 10.2, traits as bounds

**Closures as behaviour.** `handle: impl FnMut(&str) -> Option<String>` is the "what to do with a
line" part, passed in by the caller. `FnMut` because the caller's closure may change state it
captured (the MCP server will).
- Rust Book ch. 13.1, closures; read the part on `Fn`, `FnMut` and `FnOnce`

**Flushing.** Writers often buffer output. A server that doesn't flush after each reply can leave the
client waiting forever for an answer that's sitting in a buffer.

**Errors with `?`.** Reading can fail (`io::Result`). Use `?` to pass errors to the caller.
- Rust Book ch. 9.2

## Example (a different program, same techniques)

```rust
use std::io::{self, BufRead, Write};

/// Writes the number of words on each line of `input` to `output`.
fn word_counts(
    input: impl BufRead,
    mut output: impl Write,
) -> io::Result<()> {
    for line in input.lines() {
        let line = line?;
        writeln!(output, "{}", line.split_whitespace().count())?;
    }
    Ok(())
}

#[test]
fn counts_words() {
    let mut out = Vec::new();
    word_counts("a b\nc\n".as_bytes(), &mut out).unwrap();
    assert_eq!(out, b"2\n1\n");
}
```

## Tasks

- [ ] Read the tests in `tests/m01_json_lines.rs`. Each test is one requirement; list them for yourself
- [ ] Implement `run_lines` so `replies_once_per_line` passes
- [ ] Make `handles_a_last_line_without_newline` and `strips_crlf_line_endings` pass (check what
      `lines()` already does for you before writing code)
- [ ] Skip blank and whitespace-only lines
- [ ] Write nothing when the handler returns `None`
- [ ] Reject a reply containing `\n` with `io::ErrorKind::InvalidData`, *before* writing anything
- [ ] Flush after every reply
- [ ] `cargo clippy --all-targets` is clean

## Done when

```
cargo test --test m01_json_lines
```

## Stretch goals

- Make the server binary echo its input using `run_lines` with stdin/stdout (temporarily — milestone
  3 replaces this), and try it: `printf 'a\nb\n' | cargo run`
- Why does `run_lines` take `input: R` by value but you can still pass `&mut output`? (Hint: look
  for `impl<W: Write + ?Sized> Write for &mut W` in the `Write` docs.)

## Reflect

- What would go wrong if the server printed a debug message with `println!`?
- Why is the "reply contains a newline" check an *error* rather than something to fix silently (for
  example by replacing the newline)?

## Finish

- [ ] Commit your work (ticked boxes, code, notes) on `m01`
- [ ] `gh pr create --fill`, wait for the `build (stubs)` check, then merge it on GitHub.
      Offline? Keep committing; open the PR when you're back online
- [ ] `git switch main && git pull` — ready for the next milestone
