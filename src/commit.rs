use std::fs::File;
use std::io::Read;

use openai_api_rs::v1::{
    api::OpenAIClient,
    chat_completion::{self, ChatCompletionRequest},
};

const TEMPLATE_FILE_NAME: &str = ".aicommit-template";

pub fn read_template() -> std::io::Result<String> {
    if let Some(path) = dirs::home_dir() {
        let file = File::open(path.join(TEMPLATE_FILE_NAME))?;
        let mut reader = std::io::BufReader::new(file);
        let mut contents = String::new();

        reader.read_to_string(&mut contents)?;

        Ok(contents)
    } else {
        Ok(String::new())
    }
}

pub async fn generate_commit(
    content: String,
    api_key: String,
    api_url: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let model = "gemini-2.0-flash";
    let system_message = "You are a commit message generator. I will provide you with a git diff, and I would like you to generate an appropriate commit message using the conventional commit format. Do not write any explanations or other words, just reply with the commit message.";

    let mut client = OpenAIClient::builder()
        .with_endpoint(api_url)
        .with_api_key(api_key)
        .build()?;

    let req = ChatCompletionRequest::new(
        model.to_string(),
        vec![
            chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::system,
                content: chat_completion::Content::Text(system_message.to_string()),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
            chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::user,
                content: chat_completion::Content::Text(content),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
        ],
    );

    let result = client.chat_completion(req).await?;
    let contents = &result.choices[0].message.content;

    match contents {
        Some(content) => Ok(content.clone()),
        None => Ok(String::from("")),
    }
}
