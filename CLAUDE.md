- ALWAYS read CONTRIBUTING.md for guidelines on how to run tools
- ALWAYS attempt to add a test case for changed behavior. Get your tests to pass — if you didn't run the tests, your code does not work.
- PREFER integration tests over unit tests where possible
- ALWAYS run `cargo test` to run all tests
- ALWAYS run `uvx prek run -a` at the end of a task
- FOLLOW existing code style. Check neighboring files for patterns.
- AVOID writing significant amounts of new code. Look for existing methods and utilities first.
- AVOID using `panic!`, `unreachable!`, `.unwrap()`, unsafe code, and clippy rule ignores. Encode constraints in the type system instead.
- PREFER patterns like `if let` to handle fallibility
- PREFER `#[expect()]` over `#[allow()]` if clippy must be disabled
- PREFER let chains (`if let` combined with `&&`) over nested `if let` statements
- PREFER short imports over fully-qualified paths for readability
- AVOID redundant comments and section separators in test files. Use comments to explain invariants and why something unusual was done, not to narrate code.
- AVOID useless inline comments in tests. The code should speak for itself.
- PREFER function comments over inline comments

## Project Overview

Sage is a local AI shell assistant written in Rust. Users run `sage <question>` or pipe
output to it (`command | sage`). Uses Claude API (Anthropic) as the LLM backend.

## Development Commands

```bash
cargo build
cargo test
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p sage -- how do I find large files
echo "permission denied" | cargo run -p sage --
```

## Architecture

Flat `crates/` workspace:

| Crate | Responsibility |
|---|---|
| `crates/sage` | Binary entry point |
| `crates/sage_cli` | clap CLI definitions |
| `crates/sage_llm` | Claude API streaming client |
| `crates/sage_core` | Shared types, config, errors |
| `crates/sage_context` | Shell context detection |

## LLM Backend

- Model: `claude-3-5-haiku-20241022` by default
- API key from `ANTHROPIC_API_KEY` env var or `~/.config/sage/config.toml`
- Streaming SSE responses
