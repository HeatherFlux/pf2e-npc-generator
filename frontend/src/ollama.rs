use serde::{Deserialize, Serialize};
use crate::types::*;

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

pub async fn generate_description(npc: &Npc) -> Result<String, String> {
    let prompt = format!(
        "Create a brief but vivid description for a level {} {} named {}. \
        They have the following attributes: \
        Strength {}, Dexterity {}, Constitution {}, \
        Intelligence {}, Wisdom {}, Charisma {}. \
        Make the description feel like it belongs in a fantasy RPG setting. \
        Focus on their appearance, personality, and notable features. \
        Keep it to 2-3 sentences.",
        npc.level,
        format!("{:?}", npc.class),
        npc.name,
        npc.ability_scores.strength,
        npc.ability_scores.dexterity,
        npc.ability_scores.constitution,
        npc.ability_scores.intelligence,
        npc.ability_scores.wisdom,
        npc.ability_scores.charisma,
    );

    let client = reqwest::Client::new();
    let request = OllamaRequest {
        model: "llama2".to_string(),
        prompt,
        stream: false,
    };

    match client
        .post("http://localhost:3000/api/generate")
        .json(&request)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<OllamaResponse>().await {
                    Ok(ollama_response) => Ok(ollama_response.response),
                    Err(e) => Err(format!("Failed to parse response: {}", e))
                }
            } else {
                Err(format!("Ollama server error: {}. Make sure Ollama is running and the model is installed.", response.status()))
            }
        },
        Err(e) => Err(format!("Failed to connect to proxy: {}. Make sure the proxy server is running at http://localhost:3000", e))
    }
}

// ... rest of your existing ollama.rs content ... 