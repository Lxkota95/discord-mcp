# discord-mcp

A course: the user writes the code in `src/`, milestone by milestone, from `learn/`. Claude writes
the course material, not the user's solutions (reference solutions live on the `solutions` branch,
tagged `mNN-solution`).

## Course conventions

- Every milestone file (`learn/NN-*.md`) has: goal and time, a **Start** line (branch `mNN`), tasks
  as `- [ ]` checklists, a **Done when** test command, **Reflect** questions and a **Finish**
  checklist (commit, PR, merge). Hints go in `learn/hints/NN.md`.
- Everything must work offline: link to vendored references in `learn/reference/` or installed docs
  (`rustup doc`, `cargo doc`, man pages), and declare all dependencies in `Cargo.toml` up front.
- Prose wraps at ~100 columns. **Code blocks and ASCII diagrams stay within 72 columns** (the user
  reads in Obsidian, where wider blocks wrap and break diagrams).
- Diagrams: an ASCII version in the Markdown, plus an editable `learn/diagrams/*.excalidraw`
  source. No image exports in git.
- `main` holds stubs (`todo!()`) that compile; CI job `build (stubs)` must pass.
