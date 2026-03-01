use std::process::Command;

#[derive(Debug, Default)]
pub struct ShellContext {
    pub cwd: String,
    pub shell: String,
    pub os: String,
    pub git_branch: Option<String>,
}

impl ShellContext {
    pub fn detect() -> Self {
        let cwd = std::env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| "unknown".into());

        let shell = std::env::var("SHELL")
            .map(|s| s.split('/').last().unwrap_or("unknown").to_string())
            .unwrap_or_else(|_| "unknown".into());

        let os = std::env::consts::OS.to_string();

        let git_branch = Command::new("git")
            .args(["branch", "--show-current"])
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .filter(|s| !s.is_empty());

        Self {
            cwd,
            shell,
            os,
            git_branch,
        }
    }

    pub fn to_context_string(&self) -> String {
        let mut parts = vec![
            format!("shell={}", self.shell),
            format!("cwd={}", self.cwd),
            format!("os={}", self.os),
        ];
        if let Some(branch) = &self.git_branch {
            parts.push(format!("git={}", branch));
        }
        parts.join(", ")
    }
}

pub fn is_stdin_piped() -> bool {
    use std::io::IsTerminal;
    !std::io::stdin().is_terminal()
}
