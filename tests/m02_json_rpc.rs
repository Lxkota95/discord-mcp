//! Milestone 2 — done when these pass: `cargo test --test m02_json_rpc`

use discord_mcp::jsonrpc::{
    ErrorObject, INVALID_REQUEST, Id, Incoming, Notification, PARSE_ERROR, Request, Response, parse,
};
use serde_json::json;

#[test]
fn parses_a_request_with_a_number_id() {
    let msg = parse(r#"{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{"a":1}}"#).unwrap();
    assert_eq!(
        msg,
        Incoming::Request(Request {
            id: Id::Number(1),
            method: "tools/list".into(),
            params: Some(json!({"a": 1})),
        })
    );
}

#[test]
fn parses_a_request_with_a_string_id_and_no_params() {
    let msg = parse(r#"{"jsonrpc":"2.0","id":"abc","method":"server/discover"}"#).unwrap();
    assert_eq!(
        msg,
        Incoming::Request(Request {
            id: Id::String("abc".into()),
            method: "server/discover".into(),
            params: None,
        })
    );
}

#[test]
fn a_message_without_id_is_a_notification() {
    let msg =
        parse(r#"{"jsonrpc":"2.0","method":"notifications/cancelled","params":{"requestId":3}}"#)
            .unwrap();
    assert_eq!(
        msg,
        Incoming::Notification(Notification {
            method: "notifications/cancelled".into(),
            params: Some(json!({"requestId": 3})),
        })
    );
}

/// Asserts `line` fails to parse with `code`, addressed to `id`.
fn assert_error(line: &str, code: i64, id: Option<Id>) {
    let response = parse(line).expect_err("this line must not parse");
    assert_eq!(response.id, id, "error response id for {line}");
    let error = response.outcome.expect_err("the response must be an error");
    assert_eq!(error.code, code, "error code for {line}");
}

#[test]
fn text_that_is_not_json_is_a_parse_error() {
    assert_error("{not json", PARSE_ERROR, None);
}

#[test]
fn invalid_requests_are_rejected() {
    // Wrong or missing protocol version: the error is still addressed to the request's id
    assert_error(
        r#"{"jsonrpc":"1.0","id":7,"method":"x"}"#,
        INVALID_REQUEST,
        Some(Id::Number(7)),
    );
    assert_error(
        r#"{"id":7,"method":"x"}"#,
        INVALID_REQUEST,
        Some(Id::Number(7)),
    );
    // No method, or a method that isn't a string
    assert_error(
        r#"{"jsonrpc":"2.0","id":7}"#,
        INVALID_REQUEST,
        Some(Id::Number(7)),
    );
    assert_error(
        r#"{"jsonrpc":"2.0","id":7,"method":5}"#,
        INVALID_REQUEST,
        Some(Id::Number(7)),
    );
    // params must be an object or an array
    assert_error(
        r#"{"jsonrpc":"2.0","id":7,"method":"x","params":"no"}"#,
        INVALID_REQUEST,
        Some(Id::Number(7)),
    );
    // An id that isn't a number or string can't be echoed back
    assert_error(
        r#"{"jsonrpc":"2.0","id":true,"method":"x"}"#,
        INVALID_REQUEST,
        None,
    );
    // Valid JSON that isn't an object (batches were removed from MCP)
    assert_error(
        r#"[{"jsonrpc":"2.0","id":1,"method":"x"}]"#,
        INVALID_REQUEST,
        None,
    );
}

#[test]
fn a_result_serializes_with_its_id() {
    let response = Response::result(Id::String("r1".into()), json!({"ok": true}));
    assert_eq!(
        response.to_value(),
        json!({"jsonrpc": "2.0", "id": "r1", "result": {"ok": true}})
    );
}

#[test]
fn an_error_serializes_with_data_when_present() {
    let response = Response::error(
        Some(Id::Number(2)),
        ErrorObject::new(-32022, "Unsupported protocol version")
            .with_data(json!({"supported": ["x"]})),
    );
    assert_eq!(
        response.to_value(),
        json!({
            "jsonrpc": "2.0",
            "id": 2,
            "error": {"code": -32022, "message": "Unsupported protocol version", "data": {"supported": ["x"]}}
        })
    );
}

#[test]
fn an_unaddressable_error_has_a_null_id() {
    let response = parse("nope").unwrap_err();
    let value = response.to_value();
    assert_eq!(value["id"], json!(null));
    assert_eq!(value["error"]["code"], json!(PARSE_ERROR));
    assert!(
        value["error"].get("data").is_none(),
        "data is omitted when there is none"
    );
}

#[test]
fn to_line_is_a_single_line() {
    let line = Response::result(Id::Number(1), json!({"text": "a\nb"})).to_line();
    assert!(
        !line.contains('\n'),
        "JSON escapes newlines inside strings: {line}"
    );
}
