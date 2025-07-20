use anyhow::bail;
use base64::Engine;
use base64::alphabet::STANDARD;
use base64::prelude::BASE64_STANDARD;
use common::log;
use openai_dive::v1::api::Client;
use openai_dive::v1::error::APIError;
use openai_dive::v1::resources::chat::{
    ChatCompletionParametersBuilder, ChatCompletionResponse, ChatCompletionResponseFormat,
    ChatMessage, ChatMessageContent, ChatMessageContentPart, ChatMessageImageContentPart,
    ChatMessageTextContentPart, ImageUrlType,
};
use reqwest::Url;
use std::io::Read;
use std::time::Duration;
use std::{env, fs};

const BASE_URL: &str = "https://open.bigmodel.cn/api/paas/v4";
const MODEL_NAME: &str = "glm-4v-flash";

/// 提取图片中的文字
pub async fn extra(prompt: String, image_urls: Vec<String>) -> anyhow::Result<Option<String>> {
    let api_key = env::var("API_KEY").unwrap_or(env!("API_KEY").to_string());
    let mut client = Client::new(api_key.to_string());
    client.set_base_url(BASE_URL);
    client.http_client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(10))
        .build()?;
    let parameters = ChatCompletionParametersBuilder::default()
        // 模型名称
        .model(MODEL_NAME)
        // 消息
        .messages(build_messages(&prompt, image_urls).await?)
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

async fn build_messages(prompt: &str, image_urls: Vec<String>) -> anyhow::Result<Vec<ChatMessage>> {
    let mut contents = vec![];
    for image_url in image_urls {
        contents.push(ChatMessageContentPart::Image(ChatMessageImageContentPart {
            image_url: ImageUrlType {
                url: image_url_to_base64(&image_url).await?,
                detail: None,
            },
            r#type: "image_url".to_string(),
        }));
    }
    let text = ChatMessageContentPart::Text(ChatMessageTextContentPart {
        text: prompt.to_string(),
        r#type: "text".to_string(),
    });
    contents.push(text);

    let content = ChatMessageContent::ContentPart(contents);

    Ok(vec![ChatMessage::User {
        content,
        name: None,
    }])
}

async fn image_url_to_base64(image_url: &str) -> anyhow::Result<String> {
    let data = if let Ok(_) = Url::parse(image_url) {
        let client = reqwest::Client::new();
        let response = client.get(image_url).send().await?.bytes().await?;
        response.to_vec()
    } else {
        let mut file = fs::File::open(image_url)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        buffer
    };

    // 转换为 Base64
    let base64 = BASE64_STANDARD.encode(&data);
    Ok(base64)
}
