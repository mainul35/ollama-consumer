use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Serialize, Deserialize)]
struct RequestData {
    model: String,
    prompt: String,
}

// Define the ResponseChunk struct
#[derive(Deserialize)]
struct ResponseChunk {
    response: String,
    #[serde(default)]
    done: bool,
}

fn concatenate_responses(json_stream: &str) -> String {
    let mut full_response = String::new();
    
    for line in json_stream.lines() {
        if line.trim().is_empty() {
            continue;
        }
        
        match serde_json::from_str::<ResponseChunk>(line) {
            Ok(chunk) => {
                full_response.push_str(&chunk.response);
                if chunk.done {
                    break;  // Stop if this was the last chunk
                }
            },
            Err(e) => {
                eprintln!("Failed to parse chunk: {}", e);
                continue;
            }
        }
    }
    
    full_response
}

fn get_input_prompt() -> String {
    print!("Enter your prompt: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = "http://localhost:11434/api/generate";

    let mut prompt = get_input_prompt();
    while prompt.is_empty() {
        println!("Prompt cannot be empty. Please enter a valid prompt.");
        prompt = get_input_prompt();
        
    }

    let request_data = RequestData {
        model: "qwen2.5-coder:14b".to_string(),
        prompt: prompt,
    };

    // Make a POST request
    let client = reqwest::Client::new();
    let response = client.post(url)
        .json(&request_data)
        .send()
        .await?;

    // Deserialize the JSON response
    let response_text = response.text().await?;

    let result = concatenate_responses(&response_text);
    println!("{}", result);

    Ok(())
}
