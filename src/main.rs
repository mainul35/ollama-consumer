use futures::StreamExt;  // Enables `.next()` on streams
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

// This method works to return all the response at once, currently not in use
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

    loop {
        let mut prompt = get_input_prompt();

        if prompt.to_lowercase() == "quit" || prompt.to_lowercase() == "exit" {
            println!("Exiting the program. Goodbye!");
            break;
        }

        while prompt.is_empty() {
            println!("Prompt cannot be empty. Please enter a valid prompt.");
            prompt = get_input_prompt();
            
        }
    
        let request_data = RequestData {
            model: "deepseek-r1:14b".to_string(),
            prompt: prompt,
        };
    
        // Make a POST request
        let client = reqwest::Client::new();
        let response = client.post(url)
            .json(&request_data)
            .send()
            .await?;
    
        // Deserialize the JSON response
        let mut stream = response.bytes_stream();
        let mut full_response = String::new();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            let chunk_str = String::from_utf8_lossy(&chunk);

            // Parse each JSON chunk
            for line in chunk_str.lines() {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                match serde_json::from_str::<ResponseChunk>(trimmed) {
                    Ok(chunk) => {
                        print!("{}", chunk.response);
                        io::stdout().flush()?;
                        full_response.push_str(&chunk.response);

                        if chunk.done {
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("Skipping mulformed chunk: {}", e);
                    }
                }
            }
        }
        // println!("{}", result);
        println!("\n\nFull response collected: {} chars", full_response.len());
    }
    Ok(())
}
