# Milestone 2 — JSON-RPC 2.0

**Goal:** implement `parse` and `Response::to_value` in `src/jsonrpc.rs`: turn a line of text into a
typed message (or the exact error reply the protocol requires), and turn a typed reply back into
JSON.

**Time:** 2–3 hours. **Done when:** `cargo test --test m02_json_rpc` passes.

## Why this matters

Every MCP message is a [JSON-RPC 2.0](reference/json-rpc.md) message: a small, strict envelope. Get
the envelope right once, and every later milestone only deals with the contents. Read
[`reference/json-rpc.md`](reference/json-rpc.md) first (10 minutes): it's the whole spec, summarised.

## Concepts

**Three kinds of message.** A *request* has an `id` and expects a *response* with the same `id`. A
*notification* has no `id` and gets no reply. A *response* carries `result` **or** `error`, never
both. The types for all of these are already in `src/jsonrpc.rs`: read them before writing code.

**Errors are replies too.** If a line isn't JSON, you can't read its id, so the error reply has
`"id": null`. If it *is* JSON and has a readable id but is otherwise invalid, the error reply must
carry that id. That's why `parse` returns `Result<Incoming, Response>`: the `Err` is the reply.

**`serde_json::Value`, the untyped JSON tree.** `serde_json::from_str::<Value>(line)` parses any
JSON. Then you inspect it: `value.get("method")`, `match` on `Value::String(s)`, and so on. It's the
right tool when validation rules decide *which* type something is. The `json!` macro builds `Value`s.
- `cargo doc --open -p serde_json` — `Value`, `json!`, `from_str`, `from_value`

**Typed where it helps.** `Id` is an `#[serde(untagged)]` enum: serde tries `Number`, then `String`.
`serde_json::from_value::<Id>(raw)` turns a `Value` into an `Id` or an error, so you don't
hand-write that check. `ErrorObject` derives `Serialize`, so `json!({"error": error})` works.
- `cargo doc --open -p serde` — search for "untagged" (Enum representations)

**Enums and pattern matching.** `let … else` reads well for "it must be this shape, otherwise
return this error":
```rust
let Some(Value::String(method)) = object.get("method") else {
    return Err(/* … */);
};
```
- Rust Book ch. 6 (enums, `match`) and ch. 19.3 (pattern syntax, incl. `@` bindings)

## Example (a different protocol, same techniques)

```rust
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
enum Size { Pixels(u32), Named(String) }

#[derive(Debug, PartialEq)]
enum Shape { Circle { size: Size }, Square { size: Size } }

/// Reads `{"kind": "circle"|"square", "size": 3 | "big"}`.
fn shape(text: &str) -> Result<Shape, String> {
    let value: Value = serde_json::from_str(text).map_err(|e| format!("not JSON: {e}"))?;
    let Value::Object(map) = value else { return Err("not an object".into()) };
    let size: Size = serde_json::from_value(map.get("size").cloned().unwrap_or(Value::Null))
        .map_err(|_| "size must be a number or a string".to_string())?;
    match map.get("kind") {
        Some(k) if k == &json!("circle") => Ok(Shape::Circle { size }),
        Some(k) if k == &json!("square") => Ok(Shape::Square { size }),
        _ => Err("kind must be circle or square".into()),
    }
}
```

## Tasks

- [ ] Read `reference/json-rpc.md` and the types at the top of `src/jsonrpc.rs`
- [ ] Implement `Response::to_value` first (it's smaller): tests `a_result_serializes_with_its_id`,
      `an_error_serializes_with_data_when_present`
- [ ] `parse`: text that isn't JSON → `PARSE_ERROR` reply with no id
- [ ] `parse`: valid requests and notifications (three `parses_…`/`…_notification` tests)
- [ ] `parse`: each invalid case in `invalid_requests_are_rejected` → `INVALID_REQUEST`, with the
      request's id when it's readable. Read the id *first*, so later errors can use it
- [ ] `an_unaddressable_error_has_a_null_id` and `to_line_is_a_single_line` pass
- [ ] `cargo clippy --all-targets` is clean

## Done when

```
cargo test --test m02_json_rpc
```

## Stretch goals

- Implement `Serialize` for `Response` by hand (instead of building a `Value`), so it can be written
  straight to a writer. Compare the two approaches.
- Add a test of your own: what should happen with `{"jsonrpc":"2.0","id":null,"method":"x"}`?
  (The spec allows `null` ids but discourages them; MCP forbids them. Decide, write the test.)

## Reflect

- Why does the error reply for a bad request carry the request's `id` when it can? What would a
  client do with an error it can't match to a request?
- `parse` returns `Result<Incoming, Response>`. What are the advantages of the error being a ready-to-
  send reply rather than an error type like `ParseError`?
