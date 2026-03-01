use std::fmt;

#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub model: String,
    pub api_host: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let api_key = std::env::var("ANTHROPIC_API_KEY").map_err(|_| {
            anyhow::anyhow!(
                "ANTHROPIC_API_KEY is not set. Get a key at https://console.anthropic.com"
            )
        })?;
        Ok(Self {
            api_key,
            model: std::env::var("SAGE_MODEL")
                .unwrap_or_else(|_| "claude-3-5-haiku-20241022".into()),
            api_host: std::env::var("SAGE_API_HOST")
                .unwrap_or_else(|_| "https://api.anthropic.com".into()),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExitStatus {
    Success,
    Failure,
}

impl From<ExitStatus> for i32 {
    fn from(s: ExitStatus) -> i32 {
        match s {
            ExitStatus::Success => 0,
            ExitStatus::Failure => 1,
        }
    }
}

impl fmt::Display for ExitStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", i32::from(*self))
    }
}
