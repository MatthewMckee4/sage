# sage 🌿

> AI in your terminal. Ask anything, explain anything, fix anything.

```bash
sage how do I find all files larger than 1GB
sage explain "permission denied on /etc/hosts"
ls -la | sage what is taking up the most space
make build 2>&1 | sage
```

Runs entirely on your machine. No cloud. No API keys required.

## Install

```bash
cargo install sage
```

## Usage

```bash
# Ask a question
sage how do I recursively change file permissions

# Explain an error
sage explain "segmentation fault core dumped"

# Pipe output to explain it
cat build.log | sage
dmesg | tail -20 | sage
```

## Requirements

- [Ollama](https://ollama.ai) running locally (`ollama pull llama3`)
- Rust 1.75+

## License

MIT
