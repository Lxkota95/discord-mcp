# Milestone 0 — Setup

**Goal:** a working course setup that you can use with no internet: dependencies fetched, docs
built, tests running (and failing, because nothing is implemented yet).

**Time:** ~30 minutes.

**Start:** a branch for this milestone: `git switch main && git pull && git switch -c m00`.

## Tasks

- [x] Clone this repo into `~/repos/discord-mcp` (already done if you're reading this there)
- [x] Do the **Before going offline** checklist in [`README.md`](README.md)
- [x] Run `cargo test --no-fail-fast` (without the flag, cargo stops at the first failing test file).
      Expect lots of failures that say `not yet implemented` — that's the stubs
      telling you where your work goes. Note which test files exist: `tests/m01_*.rs`,
      `tests/m02_*.rs`, `tests/m03_*.rs`, one per milestone
- [x] Run just one milestone's tests: `cargo test --test m01_json_lines`
- [x] Open the docs you'll use most, so you know where they are:
  - `rustup doc --book` — the Rust Programming Language
  - `rustup doc std::io::BufRead` — a standard library item, directly
  - `cargo doc --open -p serde_json` — a dependency's docs
- [x] Look at how the reference solutions are stored: `git tag -l 'm*-solution'`, and
      `git log --oneline solutions`. Don't read them yet!
- [x] Turn off your Wi-Fi and run `cargo test --offline`. It should build and run exactly as before.
      Turn Wi-Fi back on.

## Done when

`cargo test --offline` builds and runs (with failures) while you're offline.

## Reflect

- Which three docs do you expect to use most? Is there anything you'd want offline that isn't here?
  (If so, note it — it can be added to `learn/reference/`.)

## Finish

- [ ] Commit your work (ticked boxes, code, notes) on `m00`
- [ ] `gh pr create --fill`, wait for the `build (stubs)` check, then merge it on GitHub.
      Offline? Keep committing; open the PR when you're back online
- [ ] `git switch main && git pull` — ready for the next milestone
