use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use std::env;

// Data structure to represent the request payload
#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct RequestData {
    model: String,
    messages: Vec<ChatMessage>,
}

// Data structure to represent the response from the OpenAI API
#[derive(Deserialize)]
struct ResponseData {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    text: String,
}

#[tokio::main]
pub async fn try_it() -> Result<(), Error> {
    // Replace with your OpenAI API key
    let api_key = env::var("OPENAPIKEY")
        .expect(&format!("'OPENAPIKEY' not set"));

    // Set the API endpoint URL
    let api_url = "https://api.openai.com/v1/chat/completions";

    // Create a request payload
    let request_data = RequestData {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: r#"
You are a poetic assistant, skilled in explaining complex programming concepts 
with creative flair."#.to_string()
            },
            ChatMessage {
                role: "user".to_string(),
                content: r#"
Compose a poem that explains the concept of recursion in programming.
"#.to_string(),
            }
        ],
    };

    // Create a Reqwest client and set the API key in the headers
    let client = Client::new();
    let response = client
        .post(api_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_data)
        .send()
        .await?;

    // Check if the request was successful (status code 200)
    if response.status().is_success() {
        let completed_text = response.text().await?;

//        let response_body = response.json::<ResponseData>().await?;
//        let completed_text = response_body.choices[0].text.to_string();
        println!("Completed Text:\n{}", completed_text);
    } else {
        println!("Error: {:?}", response.status().to_string());
        let text = response.text().await?;
        println!("Text: {}", text);
    }

    Ok(())
}

