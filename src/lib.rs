//! A Discord MCP server in Rust, built milestone by milestone: see `learn/README.md`.

pub mod jsonl; // milestone 1: newline-delimited messages
pub mod jsonrpc; // milestone 2: JSON-RPC 2.0 messages
pub mod mcp; // milestone 3: speaking MCP (versions, discovery, the legacy handshake)
