use anyhow::Result;
use clap::Parser;
use sage_cli::{Cli, Command};
use sage_context::{ShellContext, is_stdin_piped};
use sage_core::{Config, ExitStatus};
use std::io::{self, IsTerminal};

#[tokio::main]
async fn main() {
    let status = run().await.unwrap_or_else(|e| {
        eprintln!("sage: {e}");
        ExitStatus::Failure
    });
    std::process::exit(i32::from(status));
}

async fn run() -> Result<ExitStatus> {
    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "warn".into()))
        .init();

    let cli = Cli::parse();

    if let Some(Command::Init) = cli.command {
        return run_init();
    }

    let config = Config::from_env()?;
    let context = ShellContext::detect();

    let prompt = if is_stdin_piped() {
        let mut input = String::new();
        for line in io::stdin().lines() {
            let line = line?;
            input.push_str(&line);
            input.push('\n');
        }
        let question = cli.question_string();
        if question.is_empty() {
            format!(
                "Explain this output and suggest fixes if needed:\n\n```\n{input}```\n\nContext: {}",
                context.to_context_string()
            )
        } else {
            format!(
                "{question}\n\n```\n{input}```\n\nContext: {}",
                context.to_context_string()
            )
        }
    } else {
        let q = cli.question_string();
        if q.is_empty() {
            anyhow::bail!(
                "provide a question or pipe input\n  Usage: sage <question>\n         command 2>&1 | sage"
            );
        }
        format!("{q}\n\nContext: {}", context.to_context_string())
    };

    sage_llm::ask(&config, &prompt).await?;
    Ok(ExitStatus::Success)
}

fn run_init() -> Result<ExitStatus> {
    eprintln!("sage init");
    eprintln!("─────────");

    if !io::stdin().is_terminal() {
        anyhow::bail!("sage init requires an interactive terminal");
    }

    eprintln!("Get an API key at: https://console.anthropic.com");
    eprintln!();
    let key = rpassword::prompt_password("Paste your Anthropic API key: ")?;
    let key = key.trim().to_string();
    if key.is_empty() {
        anyhow::bail!("no key provided");
    }

    // Save to ~/.config/sage/config.toml
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("cannot determine config directory"))?
        .join("sage");
    std::fs::create_dir_all(&config_dir)?;
    let config_path = config_dir.join("config.toml");
    std::fs::write(&config_path, format!("api_key = \"{key}\"\n"))?;

    eprintln!("Saved to {}", config_path.display());
    eprintln!();
    eprintln!("All set! Try: sage how do I list files by size");
    Ok(ExitStatus::Success)
}
