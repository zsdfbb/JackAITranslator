use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}


// API请求数据结构
#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

// API响应数据结构
#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: MessageResponse,
}

#[derive(Debug, Deserialize)]
struct MessageResponse {
    content: String,
}

pub fn call(text: &str, model:String, api_key: String, _from: String, _to: String) -> Result<(), Box<dyn std::error::Error>> 
{
    let endpoint = "https://api.siliconflow.cn/v1/chat/completions";
    // 构建请求
    let client = Client::new();
    let request = ChatRequest {
        // model: "deepseek-ai/DeepSeek-R1-0528-Qwen3-8B".to_string(),  // 使用文档推荐的模型
        model,
        messages: vec![
            Message {
                role: "system".to_string(),
                content: "你是一个专业翻译，能准确进行中英互译。只提供翻译结果，其他内容不要返回".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: format!("请将下列内容从 {} 语言翻译成 {} 语言:\n{}", _from, _to, text),
            },
        ],
        stream: false,
    };

    // 发送API请求
    let response = client
        .post(endpoint)
        .bearer_auth(api_key)
        .json(&request)
        .send()?;

    // 解析响应
    if response.status().is_success() {
        let api_response: ChatResponse = response.json()?;
        if let Some(first_choice) = api_response.choices.get(0) {
            println!("翻译结果:{}", first_choice.message.content);
        } else {
            return Err("未收到有效翻译结果".into());
        }
    } else {
        return Err(format!("API请求失败: {}", response.status()).into());
    }
    return Ok(());
}