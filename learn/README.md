# Course: a Discord MCP server in Rust

You'll build an [MCP](https://modelcontextprotocol.io) server — the kind of program Claude Code
starts to give the model new tools — that helps manage a Discord server. You build it milestone by
milestone, **by hand**, starting from bytes on stdin: by the end you'll know the protocol, async
Rust with tokio, and how to expose real-world actions to an AI safely.

**Everything here works offline.** Each milestone links to documentation that's already on your
machine, ships tests that tell you when you're done, and has hints and a reference solution for when
you're stuck. Claude and the internet are welcome extras, never requirements.

## How to use this course

1. Start a branch for the milestone: `git switch main && git pull && git switch -c m01` (`main`
   is protected: changes reach it through pull requests).
2. Read the milestone file (`learn/NN-*.md`) top to bottom once, then work through its **Tasks**
   checklist, ticking boxes (`- [x]`) as you go. Commit as often as you like.
3. Run its **Done when** command. Red tests tell you what's missing; green means the milestone is done.
4. Stuck for more than ~20 minutes? Open `learn/hints/NN.md` and read **one** hint, then try again.
5. Still stuck, or curious how else it could be done? Compare with the reference solution:
   `git diff main mNN-solution -- src/` (or `git switch solutions` to browse; `git switch main` to
   get back). Reading a solution after a real attempt is learning, not cheating.
6. Answer the **Reflect** questions in your own notes.
7. Open a pull request for the milestone (`gh pr create`, or push the branch and use the link it
   prints). CI checks that it builds; merge it when you're happy. This is also the natural point to
   ask for a review. Offline, keep committing on the branch and open the PR when you're back online.

The code you write lives in `src/`. Each module starts as **stubs**: the types and function
signatures are given, and every function body is `todo!()`, so everything compiles and the tests
fail until you implement it. Don't change the signatures (the tests depend on them); do add any
private helper functions you like.

## Before going offline (do this while you have internet)

- [ ] `git pull` in this repo (new milestones and fixes)
- [ ] `cargo fetch` — downloads every dependency the **whole course** needs (they're all declared in
      `Cargo.toml` from the start)
- [ ] `cargo build --all-targets` — so later builds can run with `--offline`
- [ ] `cargo doc` — builds documentation for every dependency; open it with `cargo doc --open`
      (search box included), or a single crate with `cargo doc --open -p serde_json`
- [ ] `rustup doc` works — the Rust Book, standard library docs and Rust by Example are installed
      with the toolchain: `rustup doc --book`, `rustup doc --std`, `rustup doc --rust-by-example`,
      or jump to an item directly: `rustup doc std::io::BufRead`
- [ ] `man 2 read` shows a page (Linux man pages, needed later in the course)

Offline, build and test with `cargo build --offline` / `cargo test --offline` (plain `cargo build`
also works once everything is fetched).

## Milestones

| # | Milestone | You learn | Needs Discord? |
|---|---|---|---|
| 0 | [Setup](00-setup.md) | The course workflow, offline tooling | no |
| 1 | [JSON lines over stdin/stdout](01-json-lines.md) | `std::io`, generic readers/writers, closures, testing I/O | no |
| 2 | [JSON-RPC 2.0](02-json-rpc.md) | Modelling a protocol with serde and enums, validation, errors as values | no |
| 3 | [Speaking MCP](03-speaking-mcp.md) | Protocol versions, discovery, backward compatibility, state | no |
| 4 | Tools *(coming next)* | `tools/list`, `tools/call`, JSON Schema | no |
| 5 | Going async with tokio *(coming)* | async I/O, tasks, concurrency, cancellation | no |
| 6 | A Discord REST client *(coming)* | reqwest, typed API models, rate limits, fakes for testing | recorded responses |
| 7 | Read-only tools *(coming)* | Guild, roles, channels, permissions | recorded responses |
| 8 | Safety *(coming)* | Allowlists, preview mode, audit reasons, untrusted data | no |
| 9 | Write tools *(coming)* | Preview-then-confirm role management | recorded responses |
| 10 | Ship it *(coming)* | Logging, config, running inside Claude Code | yes, at the end |
| 11 | *(stretch)* Port to `rmcp` *(coming)* | Reading a real SDK, comparing designs | no |

Milestones 1–5 need no Discord account at all.

## Reference material in this repo

`learn/reference/` holds what you need offline that isn't in Rust's docs: the relevant pages of the
MCP specification (the official text, under its license) and short summaries, e.g. of JSON-RPC.
See [`reference/README.md`](reference/README.md).

## Getting a review

When Claude (or anyone) is around: open the milestone's pull request and ask for a review. Useful
questions: "Is this idiomatic?", "What would break under load?", "What did the reference solution
do differently, and why?"
