//! Milestone 2: JSON-RPC 2.0 messages, the envelope every MCP message travels in.
//!
//! The types are given (they follow the spec); your work is [`parse`] and [`Response::to_value`]
//! (add private helper functions as you like).
//! See `learn/02-json-rpc.md`.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Standard JSON-RPC 2.0 error codes.
pub const PARSE_ERROR: i64 = -32700;
pub const INVALID_REQUEST: i64 = -32600;
pub const METHOD_NOT_FOUND: i64 = -32601;
pub const INVALID_PARAMS: i64 = -32602;
pub const INTERNAL_ERROR: i64 = -32603;

/// A request id. JSON-RPC allows a number or a string; the reply must echo it exactly.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Id {
    Number(i64),
    String(String),
}

/// A message that expects a reply.
#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    pub id: Id,
    pub method: String,
    pub params: Option<Value>,
}

/// A message that expects no reply (it has no `id`).
#[derive(Debug, Clone, PartialEq)]
pub struct Notification {
    pub method: String,
    pub params: Option<Value>,
}

/// What a client can send a server. (On stdio, clients never send responses.)
#[derive(Debug, Clone, PartialEq)]
pub enum Incoming {
    Request(Request),
    Notification(Notification),
}

/// The `error` member of a failed response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorObject {
    pub code: i64,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl ErrorObject {
    pub fn new(code: i64, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }

    #[must_use]
    pub fn with_data(mut self, data: Value) -> Self {
        self.data = Some(data);
        self
    }
}

/// A reply to a request: a result or an error. `id` is `None` only when the request was so broken
/// that its id couldn't be read (it's then sent as `null`).
#[derive(Debug, Clone, PartialEq)]
pub struct Response {
    pub id: Option<Id>,
    pub outcome: Result<Value, ErrorObject>,
}

impl Response {
    pub fn result(id: Id, result: Value) -> Self {
        Self {
            id: Some(id),
            outcome: Ok(result),
        }
    }

    pub fn error(id: Option<Id>, error: ErrorObject) -> Self {
        Self {
            id,
            outcome: Err(error),
        }
    }

    /// The response as JSON: `{"jsonrpc": "2.0", "id": …, "result": …}` or `… "error": {…}`.
    pub fn to_value(&self) -> Value {
        todo!("milestone 2: see learn/02-json-rpc.md")
    }

    /// The response as one line of JSON, ready to write to stdout.
    pub fn to_line(&self) -> String {
        self.to_value().to_string()
    }
}

/// Parses one line from the client. On failure, returns the error response to send back:
/// `-32700` for text that isn't JSON, `-32600` for JSON that isn't a valid request or
/// notification (carrying the request's `id` when it can be read).
///
/// # Errors
///
/// The error is itself the [`Response`] to send.
#[allow(unused_variables)] // remove this line when you implement it
pub fn parse(line: &str) -> Result<Incoming, Response> {
    todo!("milestone 2: see learn/02-json-rpc.md")
}
