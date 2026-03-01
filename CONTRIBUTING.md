# Contributing to sage

## Workflow

- All changes via pull requests — no direct pushes to `main`
- Squash merge only
- Branch naming: `feat/`, `fix/`, `docs/`, `ci/`, `refactor/`

## Setup

```bash
git clone https://github.com/MatthewMckee4/sage
cd sage
cargo build
```

## Before submitting a PR

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
```
