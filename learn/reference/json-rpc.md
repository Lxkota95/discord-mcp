# JSON-RPC 2.0 in one page

A summary in our own words of the [JSON-RPC 2.0 specification](https://www.jsonrpc.org/specification)
(read it online for the full text), as MCP uses it.

## Messages

Every message is a JSON **object** with `"jsonrpc": "2.0"`.

**Request** — expects a reply:
```json
{"jsonrpc": "2.0", "id": 1, "method": "tools/list", "params": {"cursor": "abc"}}
```
- `id`: a number or a string, chosen by the sender. MCP forbids `null` and requires ids to be unique
  per sender for the session.
- `method`: a string.
- `params`: optional; if present, an object or an array (MCP always uses objects).

**Notification** — a request without `id`. The receiver **must not** reply, not even with an error:
```json
{"jsonrpc": "2.0", "method": "notifications/cancelled", "params": {"requestId": 1}}
```

**Response** — the reply to a request, with the request's `id` and exactly one of `result` or `error`:
```json
{"jsonrpc": "2.0", "id": 1, "result": {"tools": []}}
{"jsonrpc": "2.0", "id": 1, "error": {"code": -32601, "message": "Method not found"}}
```
- `error.code`: an integer; `error.message`: a short description; `error.data`: optional, anything.
- If the request's `id` couldn't be read (e.g. the text wasn't JSON), the error's `id` is `null`.

## Standard error codes

| Code | Name | When |
|---|---|---|
| -32700 | Parse error | The text isn't valid JSON |
| -32600 | Invalid Request | JSON, but not a valid request (missing `jsonrpc`/`method`, wrong types…) |
| -32601 | Method not found | The method doesn't exist (or isn't available) |
| -32602 | Invalid params | The method exists but the params are wrong |
| -32603 | Internal error | Something broke inside the server |
| -32000 to -32099 | Server error | Reserved for implementations; MCP uses -32020 to -32099 (e.g. -32022, unsupported protocol version) |

## Batches

JSON-RPC allows sending an array of requests in one message. MCP removed batching (in its
2025-06-18 version), so a JSON array is just an invalid request here.

## What MCP adds

The method names and params (`server/discover`, `tools/list`, `tools/call`, …), a protocol version
in each request's `params._meta`, and the transport (one message per line on stdio). See `mcp/`.
