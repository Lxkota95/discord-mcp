# Milestone 3 — Speaking MCP

**Goal:** implement the `Server` in `src/mcp.rs` so it answers MCP's opening moves for **both** eras
of the protocol, then wire it to stdin/stdout in `src/main.rs`. At the end, `cargo run` is a real
(if tool-less) MCP server.

**Time:** 2–3 hours. **Done when:** `cargo test --test m03_speaking_mcp` passes and the server
answers `server/discover` from your terminal.

**Start:** a branch for this milestone: `git switch main && git pull && git switch -c m03`.
Offline, with `m02` not merged yet? Branch from it instead: `git switch -c m03 m02`.

## Why this matters

MCP changed shape in its **2026-07-28** version, and real clients are mid-migration:

| | Legacy (2025-11-25) | Modern (2026-07-28) |
|---|---|---|
| Opening | `initialize` request, then `notifications/initialized` | none: the client may call anything first |
| Protocol version | negotiated once, in `initialize` | in **every** request: `params._meta["io.modelcontextprotocol/protocolVersion"]` |
| State | a session per connection | stateless |
| Discovery | the `initialize` result | `server/discover` (servers **must** implement it) |
| `ping` | exists | removed |

A server that only speaks one era fails with half the clients. Ours is **dual-era**: a request with
the modern `_meta` version is served statelessly; an `initialize` switches the process into legacy
mode. Supporting old and new at once — and failing *helpfully* when you can't — is everyday systems
work: databases, kernels and network protocols all do it.

Read (offline, in this repo):
- [`reference/mcp/2026-07-28/basic-versioning.mdx`](reference/mcp/2026-07-28/basic-versioning.mdx) —
  "Protocol Version Negotiation" and "Backward Compatibility with Initialization-Based Versions"
  (including the compatibility matrix)
- [`reference/mcp/2026-07-28/server-discover.mdx`](reference/mcp/2026-07-28/server-discover.mdx)
- [`reference/mcp/2025-11-25/lifecycle.mdx`](reference/mcp/2025-11-25/lifecycle.mdx) — the legacy
  "Initialization" section only
- [`reference/mcp/2026-07-28/changelog.mdx`](reference/mcp/2026-07-28/changelog.mdx) — skim "Major
  changes" 2, 3, 5 and 8

## Concepts

**Routing on the method name.** `match method.as_str() { "server/discover" => …, _ => … }` is the
heart of every protocol server. Unknown methods get `-32601` (`METHOD_NOT_FOUND`).

**Reading nested JSON safely.** The version lives at `params._meta["io.modelcontextprotocol/protocolVersion"]`.
Every step can be missing, so chain `Option`s: `params.as_ref().and_then(|p| p.get("_meta")).and_then(…)`.
Alternatively `Value::pointer("/_meta/…")` takes a [JSON Pointer](https://www.rfc-editor.org/rfc/rfc6901)
path — but the key contains a `/`, which a pointer must write as `~1`. (Either approach is fine.)
- `cargo doc --open -p serde_json` — `Value::get`, `Value::pointer`, `Value::as_str`

**A little state.** Modern requests need none. But after a legacy `initialize`, this process speaks
the legacy protocol for the rest of its life (one stdio process serves one client), so `Server` keeps
one piece of state. Methods that change it take `&mut self`.
- Rust Book ch. 5 (structs and methods)

**Errors with data.** An unsupported version gets MCP's own error code `-32022`, with
`data: {"supported": [...], "requested": "…"}` so the client can retry with a version you support.

**Transcript tests.** `tests/transcripts/m03-*.transcript` are recorded conversations: `>` lines are
sent to your server, `<` lines are the replies it must give (`< none` means no reply). They're plain
text: read them first — they're the precise spec of this milestone. In an expected reply, `"*"`
matches anything, used where the wording is up to you (most error messages).

## Example (a different protocol, same techniques)

A tiny dual-mode server: "v2" clients put a version in every request; "v1" clients send `hello` first.

```rust
use serde_json::{Value, json};

struct Greeter { legacy: bool }

impl Greeter {
    fn handle(&mut self, method: &str, params: Option<&Value>) -> Result<Value, String> {
        if method == "hello" {
            self.legacy = true;
            return Ok(json!({ "version": 1 }));
        }
        if self.legacy {
            return match method { "greet" => Ok(json!("hi (v1)")), _ => Err(format!("unknown: {method}")) };
        }
        let version = params.and_then(|p| p.get("v")).and_then(Value::as_u64);
        match (version, method) {
            (None, _) => Err("send v, or hello first".into()),
            (Some(2), "greet") => Ok(json!("hi (v2)")),
            (Some(v), _) if v != 2 => Err(format!("unsupported version {v}")),
            (_, m) => Err(format!("unknown: {m}")),
        }
    }
}
```

## Tasks

- [ ] Read the five transcripts in `tests/transcripts/` and the reading list above
- [ ] `Server::new` and `handle_line` (parse with milestone 2's `parse`; a parse failure's `Err` is
      already the reply)
- [ ] `handle`: notifications return `None`, requests return `Some(response)`
- [ ] Modern requests: missing version → `-32602`; unsupported version → `-32022` with `data`
- [ ] `server/discover` → the result in `m03-modern-discover.transcript` (use the constants in
      `src/mcp.rs`; the server name and version come from `ServerInfo`)
- [ ] Unknown methods and modern `ping` → `-32601`
- [ ] Legacy: `initialize` (needs `params.protocolVersion`, else `-32602`) → legacy result and legacy
      mode; then `ping` → `{}`
- [ ] Wire it up in `src/main.rs`: a `Server` and `run_lines` over `io::stdin().lock()` and
      `io::stdout().lock()`. Then try it:
      ```
      printf '%s\n' '{"jsonrpc":"2.0","id":1,"method":"server/discover","params":{"_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28"}}}' | cargo run -q
      ```
- [ ] `cargo clippy --all-targets` is clean

## Done when

```
cargo test --test m03_speaking_mcp
```
and the `printf … | cargo run` above prints a `server/discover` result.

## Stretch goals

- Write a transcript of your own (`tests/transcripts/m03-mine.transcript` plus a test in
  `tests/m03_speaking_mcp.rs`): what should a modern request *after* a legacy `initialize` get? Is
  that a client bug or a valid case? (Read the "dual-era server" lines in the versioning page.)
- Log each method name to **stderr** (`eprintln!`) and run the `printf` test again: logs and protocol
  stay separate.

## Reflect

- The spec says a modern-only server should name its supported versions in any error it returns to
  `initialize`. Why would that message matter so much to a legacy client's user?
- Where would you put a third protocol version, if MCP ships one next year?

## Finish

- [ ] Commit your work (ticked boxes, code, notes) on `m03`
- [ ] `gh pr create --fill`, wait for the `build (stubs)` check, then merge it on GitHub.
      Offline? Keep committing; open the PR when you're back online
- [ ] `git switch main && git pull` — ready for the next milestone
