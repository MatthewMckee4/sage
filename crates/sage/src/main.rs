use anyhow::Result;
use clap::Parser;
use sage_cli::Cli;
use sage_context::{ShellContext, is_stdin_piped};
use sage_core::{Config, ExitStatus};

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
    let config = Config::from_env()?;
    let context = ShellContext::detect();

    let prompt = if is_stdin_piped() {
        let mut input = String::new();
        std::io::stdin().lines().try_for_each(|l| {
            l.map(|line| {
                input.push_str(&line);
                input.push('\n');
            })
        })?;
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
            anyhow::bail!("provide a question or pipe input to sage");
        }
        format!("{q}\n\nContext: {}", context.to_context_string())
    };

    sage_llm::ask(&config, &prompt).await?;
    Ok(ExitStatus::Success)
}
