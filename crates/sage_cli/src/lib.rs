use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "sage", about = "AI in your terminal", version)]
pub struct Cli {
    /// Question to ask (or leave empty when piping input)
    pub question: Vec<String>,

    /// Claude model to use
    #[arg(long, env = "SAGE_MODEL", default_value = "claude-3-5-haiku-20241022")]
    pub model: String,

    /// Anthropic API host
    #[arg(
        long,
        env = "SAGE_API_HOST",
        default_value = "https://api.anthropic.com"
    )]
    pub host: String,
}

impl Cli {
    pub fn question_string(&self) -> String {
        self.question.join(" ")
    }
}
