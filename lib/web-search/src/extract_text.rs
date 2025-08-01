use common::log;
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::chat::{
    ChatCompletionParametersBuilder, ChatCompletionResponseFormat, ChatMessage, ChatMessageContent,
};
use std::env;
use std::time::Duration;

const BASE_URL: &str = "https://open.bigmodel.cn/api/paas/v4";
const MODEL_NAME: &str = "glm-4-flash-250414";

/// 提取图片中的文字
pub async fn extra(prompt: String, raw_content: String) -> anyhow::Result<Option<String>> {
    let base_url = env::var("BASE_URL").unwrap_or(BASE_URL.to_string());
    let model_name = env::var("MODEL_NAME").unwrap_or(MODEL_NAME.to_string());
    let api_key = env::var("API_KEY").unwrap_or(env!("API_KEY").to_string());
    let mut client = Client::new(api_key.to_string());
    client.set_base_url(&base_url);
    client.http_client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(10))
        .build()?;
    let parameters = ChatCompletionParametersBuilder::default()
        // 模型名称
        .model(&model_name)
        // 消息
        .messages(build_messages(&prompt, &raw_content).await?)
        // 返回格式
        .response_format(ChatCompletionResponseFormat::Text)
        // 不使用流式调用
        .stream(false)
        .build()?;

    let response = client.chat().create(parameters).await?;
    log::debug!("[image-qa-online]model response: {:?}", response);

    let choice = response.choices.get(0).unwrap().clone();
    let text = match choice.message {
        ChatMessage::Assistant { content, .. } => match content {
            None => None,
            Some(content) => match content {
                ChatMessageContent::Text(text) => Some(text),
                _ => return Err(anyhow::anyhow!("返回格式错误")),
            },
        },
        _ => return Err(anyhow::anyhow!("返回格式错误")),
    };

    Ok(text)
}

async fn build_messages(prompt: &str, raw_content: &String) -> anyhow::Result<Vec<ChatMessage>> {
    Ok(vec![ChatMessage::User {
        content: ChatMessageContent::Text(format!(
            "{}\n\n【以下是网页内容】\n{}",
            prompt, raw_content
        )),
        name: None,
    }])
}
