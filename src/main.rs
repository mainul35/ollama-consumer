use serde::{Deserialize, Serialize};

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = "http://localhost:11434/api/generate";
    let request_data = RequestData {
        model: "qwen2.5-coder:14b".to_string(),
        prompt: "What is the japanese of good morning?".to_string(),
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
    println!("Concatenated Response: {}", result);

    Ok(())
}
