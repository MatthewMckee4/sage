use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::{self, IsTerminal, Write};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FileConfig {
    pub api_key: Option<String>,
    pub model: Option<String>,
    pub api_host: Option<String>,
}

impl FileConfig {
    pub fn path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("sage").join("config.toml"))
    }

    pub fn load() -> Self {
        let Some(path) = Self::path() else {
            return Self::default();
        };
        let Ok(content) = std::fs::read_to_string(&path) else {
            return Self::default();
        };
        toml::from_str(&content).unwrap_or_default()
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path().ok_or_else(|| anyhow::anyhow!("cannot determine config dir"))?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&path, content)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub model: String,
    pub api_host: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let file = FileConfig::load();

        // Priority: env > config file > interactive prompt
        let api_key = if let Ok(key) = std::env::var("ANTHROPIC_API_KEY") {
            key
        } else if let Some(key) = file.api_key.clone() {
            key
        } else {
            Self::interactive_setup()?
        };

        Ok(Self {
            api_key,
            model: std::env::var("SAGE_MODEL")
                .ok()
                .or(file.model)
                .unwrap_or_else(|| "claude-3-5-haiku-20241022".into()),
            api_host: std::env::var("SAGE_API_HOST")
                .ok()
                .or(file.api_host)
                .unwrap_or_else(|| "https://api.anthropic.com".into()),
        })
    }

    fn interactive_setup() -> Result<String> {
        let stdin_is_tty = io::stdin().is_terminal();
        if !stdin_is_tty {
            anyhow::bail!(
                "API key not set.\n  Set ANTHROPIC_API_KEY or run: sage init\n  Get a key at: https://console.anthropic.com"
            );
        }

        eprintln!();
        eprintln!("Welcome to sage! Let's get you set up.");
        eprintln!();
        eprintln!("You need an Anthropic API key.");
        eprintln!("Get one at: https://console.anthropic.com");
        eprintln!();

        let key = rpassword::prompt_password("Paste your API key: ")?;
        let key = key.trim().to_string();

        if key.is_empty() {
            anyhow::bail!("no API key provided");
        }

        eprint!(
            "Save to {}? [Y/n]: ",
            FileConfig::path()
                .map(|p| p.display().to_string())
                .unwrap_or_default()
        );
        io::stderr().flush()?;
        let mut answer = String::new();
        io::stdin().read_line(&mut answer)?;
        let save = answer.trim().is_empty() || answer.trim().eq_ignore_ascii_case("y");

        if save {
            let config = FileConfig {
                api_key: Some(key.clone()),
                ..Default::default()
            };
            config.save()?;
            eprintln!("Saved. You won't be asked again.");
        }

        eprintln!();
        Ok(key)
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
