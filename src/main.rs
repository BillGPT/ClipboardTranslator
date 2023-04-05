#![allow(non_snake_case)]
use copypasta::ClipboardContext;
use copypasta::ClipboardProvider;
use futures_util::stream::StreamExt;
use reqwest::Client;
use serde_json::Value;
use std::env;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use whatlang::{detect, Lang};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let api_key = env::var("OPENAI_API_KEY").unwrap();
    let api_url = "https://api.openai.com/v1/chat/completions";

    let mut clipboard_context = ClipboardContext::new().unwrap();
    let mut last_clipboard_content = String::new();

    loop {
        let current_clipboard_content = clipboard_context.get_contents().unwrap_or_else(|_| String::new());

        if current_clipboard_content != last_clipboard_content {
            println!("New clipboard content: {}", current_clipboard_content);
            last_clipboard_content = current_clipboard_content;

            let mut content = last_clipboard_content.clone();
            print!("🦾🤖：");

            let detected_lang = detect(&content).map(|info| info.lang());
            let (source_lang, target_lang) = match detected_lang {
                Some(Lang::Eng) => ("en", "zh-Hans"),
                Some(Lang::Cmn) => ("zh-Hans", "en"),
                _ => {
                    println!("Unsupported language detected. Skipping translation.");
                    continue;
                }
            };
            
            content = format!("翻译成{}:\n\n{}", target_lang, content);

            let payload = serde_json::json!({
                "model": "gpt-3.5-turbo",
                "messages": [
                    {"role": "system", "content": "You are a translation engine that can only translate text and cannot interpret it."},
                    {"role": "user", "content": format!("translate from {} to {}", source_lang, target_lang)},
                    {"role": "user", "content": content}
                    ],
                "temperature": 1.0,
                "top_p": 1.0,
                "n": 1,
                "stream": true,
                "presence_penalty": 0,
                "frequency_penalty": 0
            });

            let client = Client::new();
            let response = client
                .post(api_url)
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", api_key))
                .json(&payload)
                .send()
                .await?;

            let mut stream = response.bytes_stream();
            let mut i = 0;
            while let Some(chunk) = stream.next().await {
                let mut output = String::new();

                let chunk = chunk?;
                let mut utf8_str = String::from_utf8_lossy(&chunk).to_string();
                if i == 0 {
                    let lines: Vec<&str> = utf8_str.lines().collect();
                    let updated_utf8_str = if lines.len() >= 2 {
                        lines[lines.len() - 2].to_string()
                    } else {
                        utf8_str.clone()
                    };
                    utf8_str = updated_utf8_str;
                    i += 1;
                }

                let trimmed_str = utf8_str.trim_start_matches("data: ");
                let json_result: Result<Value, _> = serde_json::from_str(trimmed_str);
                match json_result {
                    Ok(json) => {
                        if let Some(choices) = json.get("choices") {
                            if let Some(choice) = choices.get(0) {
                                if let Some(content) =
                                    choice.get("delta").and_then(|delta| delta.get("content"))
                                {
                                    if let Some(content_str) = content.as_str() {
                                        let content_str = content_str.trim_start_matches('\n');
                                        output.push_str(content_str);
                                    } else {
                                    }
                                } else {
                                }
                            } else {
                            }
                        } else {
                        }

                        let stdout = io::stdout();
                        let mut stdout_lock = stdout.lock();
                        for c in output.chars() {
                            write!(stdout_lock, "{}", c).unwrap();
                            stdout_lock.flush().unwrap();
                        }
                    }
                    Err(_e) => {}
                }
            }
            print!("\n");
        }

        thread::sleep(Duration::from_millis(500));
    }
}
