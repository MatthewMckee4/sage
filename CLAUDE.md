# CLAUDE.md

This file provides guidance to Claude Code when working with code in this repository.

## Project Overview

Sage is a local AI shell assistant written in Rust. Users run `sage <question>` or pipe
output to it (`command | sage`). The LLM backend is **Claude API** (Anthropic) — not Ollama.
The binary runs on the user's machine; it calls the Claude API with their own API key.

## Development Commands

```bash
cargo build
cargo test
cargo fmt
cargo clippy
cargo run -p sage -- how do I find large files
echo "error: file not found" | cargo run -p sage --
```

## Architecture

Flat crates/ workspace:

| Crate | Responsibility |
|---|---|
| `crates/sage` | Binary entry point |
| `crates/sage_cli` | clap CLI definitions |
| `crates/sage_llm` | Claude API async client (streaming) |
| `crates/sage_core` | Shared types, config, errors |
| `crates/sage_context` | Shell context detection (cwd, git, OS) |

## LLM Backend: Claude API

- Model: `claude-3-5-haiku-20241022` (fast, cheap, good for shell tasks)
- API key from `ANTHROPIC_API_KEY` env var
- Endpoint: `https://api.anthropic.com/v1/messages`
- Use streaming API (SSE) so tokens print as they arrive
- If no API key: print helpful error "Set ANTHROPIC_API_KEY to use sage"

## Code Conventions

- Edition 2024, MSRV 1.80
- No unwrap() in library code — use anyhow::Result and ?
- Use tracing not println! for debug output
- Stream tokens to stdout as they arrive
- Keep crates small and single-responsibility

## Style

- Minimal comments — clean code speaks for itself
- Flat module structure
- All errors via anyhow
