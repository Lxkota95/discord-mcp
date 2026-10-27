//! Milestone 3: speaking MCP — protocol versions, `server/discover`, and the legacy handshake.
//!
//! This server is *dual-era*: it serves modern clients (MCP 2026-07-28, stateless, every request
//! carries its protocol version in `_meta`) and legacy clients (MCP 2025-11-25, which open with an
//! `initialize` handshake). See `learn/03-speaking-mcp.md`.

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
        Self {
            info,
            legacy: false,
        }
    }

    /// Handles one line from the client and returns the line to send back, if any. Lines that
    /// don't parse get the parse error response from [`jsonrpc::parse`].
    pub fn handle_line(&mut self, line: &str) -> Option<String> {
        match jsonrpc::parse(line) {
            Ok(message) => self.handle(message).map(|response| response.to_line()),
            Err(response) => Some(response.to_line()),
        }
    }

    /// Handles one parsed message. Notifications never get a reply.
    pub fn handle(&mut self, message: Incoming) -> Option<Response> {
        match message {
            Incoming::Notification(notification) => {
                self.notify(&notification);
                None
            }
            Incoming::Request(request) => Some(self.request(request)),
        }
    }

    fn notify(&mut self, _notification: &Notification) {
        // `notifications/initialized` (legacy) and `notifications/cancelled` need no action yet
    }

    fn request(&mut self, request: Request) -> Response {
        let Request { id, method, params } = request;

        // Legacy era: the client opens with `initialize`, then talks without `_meta` versions
        if method == "initialize" {
            return self.initialize(id, params.as_ref());
        }
        if self.legacy {
            return match method.as_str() {
                "ping" => Response::result(id, json!({})),
                _ => method_not_found(id, &method),
            };
        }

        // Modern era: every request names its protocol version
        let Some(requested) = params
            .as_ref()
            .and_then(|p| p.pointer(&format!("/_meta/{}", escape(META_PROTOCOL_VERSION))))
            .and_then(Value::as_str)
        else {
            return Response::error(
                Some(id),
                ErrorObject::new(
                    jsonrpc::INVALID_PARAMS,
                    format!(
                        "missing params._meta[\"{META_PROTOCOL_VERSION}\"]; this server supports \
                         {MODERN_VERSION}, or {LEGACY_VERSION} via initialize"
                    ),
                ),
            );
        };
        if requested != MODERN_VERSION {
            return Response::error(
                Some(id),
                ErrorObject::new(UNSUPPORTED_PROTOCOL_VERSION, "Unsupported protocol version")
                    .with_data(json!({
                        "supported": [MODERN_VERSION, LEGACY_VERSION],
                        "requested": requested,
                    })),
            );
        }

        match method.as_str() {
            "server/discover" => Response::result(id, self.discover()),
            _ => method_not_found(id, &method),
        }
    }

    /// `initialize` (legacy): answer with the legacy version we speak; the client decides whether
    /// it can use it. From now on this process serves the legacy protocol.
    fn initialize(&mut self, id: Id, params: Option<&Value>) -> Response {
        if params
            .and_then(|p| p.get("protocolVersion"))
            .and_then(Value::as_str)
            .is_none()
        {
            return Response::error(
                Some(id),
                ErrorObject::new(
                    jsonrpc::INVALID_PARAMS,
                    "initialize needs a protocolVersion",
                ),
            );
        }
        self.legacy = true;
        Response::result(
            id,
            json!({
                "protocolVersion": LEGACY_VERSION,
                "capabilities": { "tools": {} },
                "serverInfo": { "name": self.info.name, "version": self.info.version },
            }),
        )
    }

    /// `server/discover` (modern): supported versions, capabilities and identity.
    fn discover(&self) -> Value {
        json!({
            "resultType": "complete",
            "supportedVersions": [MODERN_VERSION, LEGACY_VERSION],
            "capabilities": { "tools": {} },
            "_meta": {
                META_SERVER_INFO: { "name": self.info.name, "version": self.info.version },
            },
            "ttlMs": 3_600_000,
            "cacheScope": "public",
        })
    }
}

fn method_not_found(id: Id, method: &str) -> Response {
    Response::error(
        Some(id),
        ErrorObject::new(METHOD_NOT_FOUND, format!("Method not found: {method}")),
    )
}

/// Escapes a key for a JSON Pointer (RFC 6901): `~` becomes `~0` and `/` becomes `~1`, so the
/// `/` inside `io.modelcontextprotocol/protocolVersion` isn't read as a path separator.
fn escape(key: &str) -> String {
    key.replace('~', "~0").replace('/', "~1")
}
