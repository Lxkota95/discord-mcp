//! Milestone 2: JSON-RPC 2.0 messages, the envelope every MCP message travels in.
//!
//! The types are given (they follow the spec); the work is in [`parse`] and [`Response::to_value`].
//! See `learn/02-json-rpc.md`.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

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
        let id = match &self.id {
            Some(id) => json!(id),
            None => Value::Null,
        };
        match &self.outcome {
            Ok(result) => json!({ "jsonrpc": "2.0", "id": id, "result": result }),
            Err(error) => json!({ "jsonrpc": "2.0", "id": id, "error": error }),
        }
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
pub fn parse(line: &str) -> Result<Incoming, Response> {
    let value: Value = serde_json::from_str(line)
        .map_err(|_| Response::error(None, ErrorObject::new(PARSE_ERROR, "Parse error")))?;

    let Value::Object(object) = value else {
        return Err(invalid(None, "a message must be a JSON object"));
    };
    // Read the id first, so later errors can be addressed to the request
    let id = match object.get("id") {
        None => None,
        Some(raw) => Some(
            serde_json::from_value::<Id>(raw.clone())
                .map_err(|_| invalid(None, "id must be a number or a string"))?,
        ),
    };
    if object.get("jsonrpc") != Some(&json!("2.0")) {
        return Err(invalid(id, r#"jsonrpc must be "2.0""#));
    }
    let Some(Value::String(method)) = object.get("method") else {
        return Err(invalid(id, "method must be a string"));
    };
    let params = params(&object).map_err(|message| invalid(id.clone(), message))?;

    Ok(match id {
        Some(id) => Incoming::Request(Request {
            id,
            method: method.clone(),
            params,
        }),
        None => Incoming::Notification(Notification {
            method: method.clone(),
            params,
        }),
    })
}

/// `params` is optional, but when present must be an object or an array.
fn params(object: &Map<String, Value>) -> Result<Option<Value>, &'static str> {
    match object.get("params") {
        None => Ok(None),
        Some(p @ (Value::Object(_) | Value::Array(_))) => Ok(Some(p.clone())),
        Some(_) => Err("params must be an object or an array"),
    }
}

fn invalid(id: Option<Id>, message: &str) -> Response {
    Response::error(
        id,
        ErrorObject::new(INVALID_REQUEST, format!("Invalid request: {message}")),
    )
}
