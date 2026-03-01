use anyhow::Result;
use reqwest::Client;
use sage_core::Config;
use serde::Serialize;
use std::io::Write;

#[derive(Serialize)]
struct Request<'a> {
    model: &'a str,
    max_tokens: u32,
    system: &'a str,
    messages: Vec<Message<'a>>,
    stream: bool,
}

#[derive(Serialize)]
struct Message<'a> {
    role: &'a str,
    content: &'a str,
}

const SYSTEM_PROMPT: &str = "You are sage, an AI shell assistant. \
Give concise, direct answers. Format shell commands in code blocks. \
When explaining errors, identify the root cause first then the fix. Be brief.";

pub async fn ask(config: &Config, prompt: &str) -> Result<()> {
    let client = Client::new();
    let body = Request {
        model: &config.model,
        max_tokens: 1024,
        system: SYSTEM_PROMPT,
        messages: vec![Message {
            role: "user",
            content: prompt,
        }],
        stream: true,
    };

    let mut response = client
        .post(format!("{}/v1/messages", config.api_host))
        .header("x-api-key", &config.api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await?;

    if !response.status().is_success() {
        let text = response.text().await?;
        anyhow::bail!("API error: {}", text);
    }

    let stdout = std::io::stdout();
    let mut out = stdout.lock();

    while let Some(chunk) = response.chunk().await? {
        let text = std::str::from_utf8(&chunk)?;
        for line in text.lines() {
            if let Some(data) = line.strip_prefix("data: ") {
                if data == "[DONE]" {
                    break;
                }
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(data)
                    && let Some(delta) = json.pointer("/delta/text").and_then(|v| v.as_str())
                {
                    out.write_all(delta.as_bytes())?;
                    out.flush()?;
                }
            }
        }
    }
    writeln!(out)?;
    Ok(())
}
