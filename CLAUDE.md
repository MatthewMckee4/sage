# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Sage is a local AI shell assistant written in Rust. Users run `sage <question>` or pipe
output to it (`command | sage`). Calls Claude API (Anthropic) with the user's own API key.

## Style

Do not add too many comments. Only comment where necessary — if the code is complicated and cannot be simplified.

## Development Commands

```bash
# Build and test
cargo build
cargo test

# Format
cargo fmt

# Lint
cargo clippy

# Run CLI
cargo run -p sage -- how do I find large files
echo "permission denied" | cargo run -p sage --
```

## Architecture

Flat crates/ workspace:

| Crate | Responsibility |
|---|---|
| `crates/sage` | Binary entry point |
| `crates/sage_cli` | clap CLI definitions and arg parsing |
| `crates/sage_llm` | Claude API async streaming client |
| `crates/sage_core` | Shared types, config, errors |
| `crates/sage_context` | Shell context detection (cwd, git, OS, shell) |

### CLI Flow

1. `main.rs` parses args with clap
2. Detects if stdin is piped (isatty check)
3. Builds context string from `sage_context`
4. Calls Claude API via `sage_llm`, streams tokens to stdout
5. Returns `ExitStatus` enum (Success=0, Failure=1)

### Command Functions

**Critical**: All functions MUST return `Result<ExitStatus>` from `anyhow`. Use `?` to propagate errors.

### LLM Backend

- **Claude API** (`https://api.anthropic.com/v1/messages`)
- Model: `claude-3-5-haiku-20241022` (fast, cheap)
- API key from `ANTHROPIC_API_KEY` env var
- SSE streaming — print tokens as they arrive
- If no API key: print `Error: set ANTHROPIC_API_KEY to use sage`

## Code Conventions

- **Edition 2024, MSRV 1.80**
- **No `unwrap()`** in library code — use `anyhow::Result` and `?`
- **No direct `print!`/`eprintln!`** — all output through a `Printer` abstraction
- **Strict clippy pedantic**
- All shared deps declared in `[workspace.dependencies]`

## PR Workflow

- All changes via pull requests — no direct commits to `main`
- Squash merge only
- Branch naming: `feat/`, `fix/`, `docs/`, `ci/`, `refactor/`
