# Design: how discord-mcp fits together

Read this before writing code (it's [milestone 0b](00b-design.md)). It explains what the pieces
are, how one request travels through them, and why the code is split the way it is. Every milestone
builds one layer described here.

## 1. The actors

```
┌───────────────────────── your laptop ──────────────────────────┐
│                                                                │
│  you ──▶ Claude Code (the "host")                              │
│            │  runs the conversation, shows you every           │
│            │  tool call and asks before running it             │
│            │                                                   │
│            ├── MCP client ──stdio──▶ discord-mcp (our server)  │
│            ├── MCP client ────────▶ another MCP server         │
│            └── the model (Claude), over the internet           │
│                                           │                    │
└───────────────────────────────────────────┼────────────────────┘
                                            │ HTTPS (bot token)
                                            ▼
                                   Discord's REST API
                                   (your Discord server)
```

- **Host — Claude Code.** Owns the conversation and the model. It starts MCP servers as child
  processes and decides what the model may do. It keeps the **human in the loop**: you see and
  approve tool calls.
- **Client.** The part of Claude Code that talks to **one** server. One client per server, so servers
  can't see each other.
- **Server — discord-mcp.** Offers **tools** (named actions with a JSON input schema) and nothing
  else. It never sees the conversation; it only gets the tool calls the host forwards.
- **Discord.** Our server talks to Discord's REST API with a **bot token**. Discord is where real
  effects happen — and where mistakes would hurt.

Full diagram: [`diagrams/actors.excalidraw`](diagrams/actors.excalidraw) ([how to open](diagrams/README.md)).

Official version: [`reference/mcp/2026-07-28/architecture.mdx`](reference/mcp/2026-07-28/architecture.mdx)
(read "Core Components" and "Design Principles").

## 2. The journey of one tool call

You ask Claude Code: *"Create a Gold role on my Discord server."*

```
you    Claude Code                  discord-mcp                  Discord
         + model
 │          │                            │                          │
 │─ ask ───▶│                            │                          │
 │          │─ server/discover ─────────▶│ (once)                   │
 │          │◀── versions, capabilities ─│                          │
 │          │─ tools/list ──────────────▶│                          │
 │          │◀─── tools + input schemas ─│                          │
 │          │ model picks create_role    │                          │
 │◀ approve?│                            │                          │
 │─ yes ───▶│                            │                          │
 │          │─ tools/call create_role ──▶│                          │
 │          │ {"name": "Gold"}           │ validate input, check    │
 │          │                            │ allowlist, preview (§5)  │
 │          │                            │ POST /guilds/{id}/roles ▶│
 │          │                            │◀── 200 {id, name: Gold} ─│
 │          │◀── result: "Created Gold" ─│                          │
 │◀ answer ─│                            │                          │
 │          │                            │                          │
```

Full diagram: [`diagrams/tool-call.excalidraw`](diagrams/tool-call.excalidraw).

Every arrow between Claude Code and discord-mcp is **one JSON-RPC message on one line** of the
server's stdin or stdout. Each request carries its protocol version in `params._meta`, because modern
MCP is **stateless**: the server needs nothing from earlier requests to answer this one.

## 3. The layers we build

Each layer only knows about the one below it. That's what makes each one small and testable on its
own — and it's the order of the milestones.

```
┌────────────────────────────────────────────────────────┐
│ main.rs       config, logging, stdin/stdout            │ m10
├────────────────────────────────────────────────────────┤
│ safety        allowlisted guild, preview, audit log    │ m8, m9
├────────────────────────────────────────────────────────┤
│ tools         list_roles, create_role, …: schemas,     │ m4, m7, m9
│               calls into Discord, results for model    │
├───────────────────────────┬────────────────────────────┤
│ mcp (protocol)            │ discord (REST client)      │ m3 | m6
│ versions, discover,       │ typed models, HTTP,        │
│ initialize, routing       │ rate limits, errors        │
├───────────────────────────┴────────────────────────────┤
│ jsonrpc       envelope: parse, validate, reply         │ m2
├────────────────────────────────────────────────────────┤
│ jsonl         one message per line, any reader/writer  │ m1
├────────────────────────────────────────────────────────┤
│ async runtime tokio: concurrency, cancellation         │ m5 (rebuilds
│                                                        │ m1's loop)
└────────────────────────────────────────────────────────┘
```

Full diagram: [`diagrams/layers.excalidraw`](diagrams/layers.excalidraw).

| Layer | Knows about | Doesn't know about |
|---|---|---|
| `jsonl` | lines of text | JSON, MCP, Discord |
| `jsonrpc` | JSON-RPC envelopes and error codes | what methods mean |
| `mcp` | protocol versions, method routing | Discord |
| `discord` | Discord's REST API | MCP |
| `tools` | both: turns tool calls into Discord calls | how bytes are framed |

Why this split pays off: the transport, JSON-RPC and MCP layers are tested with plain strings
(milestones 1–3, no Discord needed); the Discord client is tested against **recorded** responses
(milestone 6, no network needed); only the final milestone talks to the real Discord.

## 4. Errors: two kinds, on purpose

| Kind | Example | How it's reported | Who acts on it |
|---|---|---|---|
| **Protocol error** | not JSON, unknown method, unsupported version, unknown tool | JSON-RPC `error` (codes like -32700, -32601, -32602, -32022) | Claude Code (a bug or version mismatch) |
| **Tool execution error** | Discord says 403, role name too long, guild not allowed | a normal `result` with `"isError": true` and a readable message | **the model**, which can adjust and retry |

Getting this split right is what lets the model fix its own mistakes ("that role already exists —
I'll pick another name") instead of the whole call just failing. Official text:
[`reference/mcp/2026-07-28/server-tools.mdx`](reference/mcp/2026-07-28/server-tools.mdx),
"Error Handling".

## 5. Safety: where the guardrails are

The model is capable but can be wrong, and text it reads (role names, channel topics) can contain
instructions written by *anyone* on the Discord server. So safety is layered:

1. **Claude Code's permission prompt** — you approve each tool call (the host's job, not ours).
2. **Least privilege** — the server uses its own Discord bot, with only the permissions its tools need.
3. **Guild allowlist** (m8) — the server refuses to act on any Discord server not explicitly listed.
4. **Preview, then confirm** (m8–m9) — write tools first return what they *would* do; only a second,
   confirmed call does it. (MCP's "input required" results are one way to ask; see
   [`reference/mcp/2026-07-28/server-tools.mdx`](reference/mcp/2026-07-28/server-tools.mdx).)
5. **Audit trail** — every change carries a reason in Discord's audit log (`via discord-mcp`).
6. **No message content** — there are no tools that read chat messages, the biggest source of
   untrusted text. Data from Discord is always treated as data, never as instructions.

## 6. Two protocol eras

MCP changed in 2026-07-28 (stateless, `server/discover`) and older clients still speak 2025-11-25
(an `initialize` handshake, then a session). Our server answers both — see milestone 3 for the
details. Everything above the `mcp` layer doesn't care which era a request came from.

## 7. Deliberately out of scope

- **Other MCP features**: resources, prompts, subscriptions, sampling — tools are enough here.
- **The Streamable HTTP transport**: Claude Code runs us locally over stdio; no web server needed.
- **Discord's gateway (websocket)**: we only make REST calls when a tool runs; no live connection.
- **Reading messages**: see §5.

These are good stretch topics once the core works.
