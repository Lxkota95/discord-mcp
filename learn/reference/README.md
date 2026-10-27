# Offline reference material

| Path | What | Source and license |
|---|---|---|
| `json-rpc.md` | JSON-RPC 2.0 summarised | Written for this course (MIT, like the repo) |
| `mcp/2026-07-28/` | MCP specification pages (version 2026-07-28): overview, architecture, base protocol, stdio transport, versioning, cancellation, server features index, `server/discover`, tools, changelog, and the JSON schema | [modelcontextprotocol/modelcontextprotocol](https://github.com/modelcontextprotocol/modelcontextprotocol), copied unmodified; licensed as described in `mcp/LICENSE` (Apache-2.0, with some MIT-licensed contributions) |
| `mcp/2025-11-25/` | MCP 2025-11-25 base protocol and lifecycle (the legacy `initialize` handshake) | Same as above |

The `.mdx` files are Markdown with a few JSX components (`<Note>`, `<Tabs>` …); read them as
Markdown and skip the tags. `cat` renders them in the terminal; Neovim shows them as plain text.

Rust's own documentation is installed with the toolchain: `rustup doc --book`, `rustup doc --std`,
`rustup doc std::io::BufRead`. Dependency docs: `cargo doc --open`. Linux system calls:
`man 2 <name>` (e.g. `man 2 read`).

More reference material arrives with later milestones (Discord API pages, under CC-BY-SA-4.0).
