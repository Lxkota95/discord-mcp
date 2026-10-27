//! The discord-mcp server binary: MCP over stdio.

use discord_mcp::jsonl::run_lines;
use discord_mcp::mcp::{Server, ServerInfo};
use std::io;

fn main() -> io::Result<()> {
    let mut server = Server::new(ServerInfo {
        name: env!("CARGO_PKG_NAME").to_owned(),
        version: env!("CARGO_PKG_VERSION").to_owned(),
    });
    // stdout is the protocol channel: nothing else may be printed there (logs go to stderr)
    run_lines(io::stdin().lock(), io::stdout().lock(), |line| {
        server.handle_line(line)
    })
}
