# discord-mcp
An [MCP](https://modelcontextprotocol.io) server, written in Rust, that lets AI assistants such as
Claude Code help run a Discord server, with guardrails.

It's also a **course**: built milestone by milestone from bytes on stdin upward, as part of learning
systems programming in Rust. Start at [`learn/README.md`](learn/README.md). Everything works offline.

| Branch | What |
|---|---|
| `main` | The course and the code as I build it. Unfinished milestones are stubs (`todo!()`) whose tests fail until implemented |
| `solutions` | Reference solutions, one tagged commit per milestone (`m01-solution`, …) |

**Status:** milestones 0–3 (transport, JSON-RPC, MCP versioning) are written; Discord tools come next.
The server speaks MCP **2026-07-28** (stateless, `server/discover`) and, for older clients,
**2025-11-25** (`initialize`).

## License
MIT. `learn/reference/mcp/` contains MCP specification pages under their own license (see
`learn/reference/README.md`).
