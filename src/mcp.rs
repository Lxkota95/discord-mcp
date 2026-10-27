//! Milestone 3: speaking MCP — protocol versions, `server/discover`, and the legacy handshake.
//!
//! This server is *dual-era*: it serves modern clients (MCP 2026-07-28, stateless, every request
//! carries its protocol version in `_meta`) and legacy clients (MCP 2025-11-25, which open with an
//! `initialize` handshake). See `learn/03-speaking-mcp.md`.

#![allow(unused_variables, unused_imports, dead_code)] // remove when you implement milestone 3

use crate::jsonrpc::{
    self, ErrorObject, Id, Incoming, METHOD_NOT_FOUND, Notification, Request, Response,
};
use serde_json::{Value, json};

/// The modern protocol version: stateless, version in every request's `_meta`.
pub const MODERN_VERSION: &str = "2026-07-28";
/// The legacy protocol version, negotiated once with `initialize`.
pub const LEGACY_VERSION: &str = "2025-11-25";
/// MCP's error code for a protocol version the server doesn't support.
pub const UNSUPPORTED_PROTOCOL_VERSION: i64 = -32022;
/// Where modern requests carry their protocol version: `params._meta[...]`.
pub const META_PROTOCOL_VERSION: &str = "io.modelcontextprotocol/protocolVersion";
/// Where modern results carry the server's identity: `result._meta[...]`.
pub const META_SERVER_INFO: &str = "io.modelcontextprotocol/serverInfo";

/// The server's name and version, as reported to clients.
#[derive(Debug, Clone)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
}

/// An MCP server's protocol state. Modern requests need no state; a legacy client's `initialize`
/// switches this process into legacy mode for the rest of its life (one stdio process serves one
/// client).
#[derive(Debug)]
pub struct Server {
    info: ServerInfo,
    legacy: bool,
}

impl Server {
    pub fn new(info: ServerInfo) -> Self {
        todo!("milestone 3: see learn/03-speaking-mcp.md")
    }

    /// Handles one line from the client and returns the line to send back, if any. Lines that
    /// don't parse get the parse error response from [`jsonrpc::parse`].
    pub fn handle_line(&mut self, line: &str) -> Option<String> {
        todo!("milestone 3: see learn/03-speaking-mcp.md")
    }

    /// Handles one parsed message. Notifications never get a reply.
    pub fn handle(&mut self, message: Incoming) -> Option<Response> {
        todo!("milestone 3: see learn/03-speaking-mcp.md")
    }

    // Add private methods here: e.g. one per MCP method, and one for the version check.
}
