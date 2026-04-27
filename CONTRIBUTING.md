# Contributing to Bhasha

Thanks for helping improve Bhasha. This document explains how to work on the repo and what we look for in contributions.

## Ground rules

- Be kind and professional. Follow the [Code of Conduct](CODE_OF_CONDUCT.md).
- Prefer small, focused pull requests over large rewrites unless discussed first.
- Match existing Rust style (`cargo fmt`). Fix new warnings your change introduces when practical.

## Getting set up

1. Fork and clone the repository.
2. Install stable Rust (2021 edition or newer).
3. From the repo root:

   ```bash
   cargo build
   cargo test
   cargo fmt --all -- --check
   cargo clippy --all-targets --all-features
   ```

4. Run the interpreter on an example:

   ```bash
   cargo run -- examples/hello.bhasha
   ```

## Where to look in the code

| If you want to… | Start here |
|-------------------|------------|
| Add or change syntax / keywords | `src/tokens.rs`, then `src/parser.rs` |
| Change evaluation semantics | `src/interpreter.rs` |
| Change AST shape | `src/ast.rs` (update parser + interpreter together) |
| CLI behavior (args, file I/O) | `src/main.rs` |

Language keywords are defined as Logos tokens in `tokens.rs`. Keep README / examples in sync when you add user-visible syntax.

## Tests

The project currently has few automated tests. Contributions that add **unit tests** (parser snippets, interpreter outcomes) or **integration tests** (run `.bhasha` files and assert output) are especially welcome.

Suggested layout:

- `#[cfg(test)]` modules next to `parser`, `interpreter`, or `tokens` for small cases.
- Optional `tests/` integration tests using `std::process::Command` to run the binary.

## Pull request checklist

- [ ] `cargo fmt`, `cargo clippy`, and `cargo test` pass locally.
- [ ] New syntax or flags are documented in `README.md` and/or `examples/` if user-facing.
- [ ] Breaking language changes are called out in the PR description.

## README badges (optional)

If you publish the repo on GitHub, replace `<your-username>` in `README.md` clone URLs and add a CI badge pointing at your fork’s Actions tab if desired.

## Questions

Open a [discussion](https://github.com/<your-username>/bhasha/discussions) or an issue for design questions before investing in a large change.
