//! Milestone 3 — done when these pass: `cargo test --test m03_speaking_mcp`
//! Each test plays a recorded conversation from tests/transcripts/ (see tests/common/mod.rs).

mod common;
use common::check_transcript;

#[test]
fn modern_discover() {
    check_transcript("m03-modern-discover");
}

#[test]
fn modern_version_errors() {
    check_transcript("m03-modern-version-errors");
}

#[test]
fn modern_unknown_and_removed_methods() {
    check_transcript("m03-modern-methods");
}

#[test]
fn legacy_handshake() {
    check_transcript("m03-legacy-handshake");
}

#[test]
fn broken_input() {
    check_transcript("m03-broken-input");
}
