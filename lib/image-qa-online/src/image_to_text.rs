use openai_dive::v1::api::Client;
use openai_dive::v1::resources::chat::{
    ChatCompletionParametersBuilder, ChatCompletionResponseFormat, ChatMessage, ChatMessageContent,
    ChatMessageContentPart, ChatMessageImageContentPart, ChatMessageTextContentPart, ImageUrlType,
};
use std::fs;
use std::io::Read;
use std::time::Duration;

/// 提取图片中的文字
pub async fn extra(
    prompt: String,
    image_urls: Vec<String>,
    //base_url: &str,
    //model_name: &str,
    //api_key: &str,
) -> anyhow::Result<Option<String>> {
    let base_url = "https://open.bigmodel.cn/api/paas/v4";
    let model_name = "glm-4v-flash";
    let api_key = "c50114c7ae6143a1bf4d6b624a2ca80e.dSZnckn86UIdNHbT";
    let mut client = Client::new(api_key.to_string());
    client.set_base_url(base_url);
    client.http_client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(10))
        .build()?;
    let parameters = ChatCompletionParametersBuilder::default()
        // 模型名称
        .model(model_name)
        // 消息
        .messages(build_messages(&prompt, image_urls))
        // 返回格式
        .response_format(ChatCompletionResponseFormat::Text)
        // 不使用流式调用
        .stream(false)
        .build()?;

    let response = client.chat().create(parameters).await?;
    log::debug!("[image-to-text]model response: {:?}", response);

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

fn build_messages(prompt: &str, image_urls: Vec<String>) -> Vec<ChatMessage> {
    let mut contents = vec![];
    for image_url in image_urls {
        contents.push(ChatMessageContentPart::Image(ChatMessageImageContentPart {
            image_url: ImageUrlType {
                url: image_url.to_string(),
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

    vec![ChatMessage::User {
        content,
        name: None,
    }]
}
