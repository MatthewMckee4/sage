use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "sage", about = "AI in your terminal", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Question to ask (when no subcommand given)
    pub question: Vec<String>,

    /// Claude model to use
    #[arg(
        long,
        env = "SAGE_MODEL",
        global = true,
        default_value = "claude-3-5-haiku-20241022"
    )]
    pub model: String,

    /// Anthropic API host
    #[arg(
        long,
        env = "SAGE_API_HOST",
        global = true,
        default_value = "https://api.anthropic.com"
    )]
    pub host: String,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Set up sage interactively (API key, model, etc.)
    Init,
}

impl Cli {
    pub fn question_string(&self) -> String {
        self.question.join(" ")
    }
}
