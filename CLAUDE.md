# CLAUDE.md

This file provides guidance to Claude Code when working with code in this repository.

## Project Overview

Sage is a local AI shell assistant written in Rust. Users run `sage <question>` or pipe
output to it (`command | sage`) and get instant, context-aware answers powered by a
local Ollama LLM. No cloud. No API keys. Zero config to get started.

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

Flat crates/ workspace (seal-style):

| Crate | Responsibility |
|---|---|
| `crates/sage` | Binary entry point, wires everything together |
| `crates/sage_cli` | clap CLI definitions and argument parsing |
| `crates/sage_llm` | Ollama async REST client, streaming responses |
| `crates/sage_core` | Shared types, errors, config |
| `crates/sage_context` | Shell context detection (cwd, git, OS, exit code) |

## Code Conventions

- Edition 2024, MSRV 1.80
- No unwrap() in library code — use anyhow::Result and ?
- Use tracing not println! for debug output
- Streaming LLM output: print tokens as they arrive, don't buffer
- Keep crates small and single-responsibility
- Follow seal's style: minimal comments, clean code speaks for itself

## Style

- Do not add excessive comments
- Prefer flat module structure
- All errors propagate via anyhow
