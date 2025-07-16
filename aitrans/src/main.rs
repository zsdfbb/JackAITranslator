use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::env;

// API请求数据结构
#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
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

// 定义配置文件路径
const CONFIG_TOML_PATH: &str = "~/.config/aitrans/config.toml";

/* 从配置文件 ~/.config/aitrans/config.toml 读取API参数 */
fn read_api_config() -> Result<(String, String), Box<dyn std::error::Error>> {
    let config = config::Config::builder()
        .add_source(config::File::with_name(CONFIG_TOML_PATH))
        .build()?;

    let model = config.get_string("model")?;
    let api_key = config.get_string("api_key")?;

    Ok((model, api_key))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    read_api_config()

    // 从命令行参数获取待翻译文本
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <text_to_translate>", args[0]);
        std::process::exit(1);
    }
    let text = &args[1..].join(" ");
    println!("待翻译文本: {}", text);

    // 设置API参数
    let (model, api_key) = read_api_config()?;
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
                content: format!("请将以下文本翻译成英文:\n{}", text),
            },
        ],
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
            eprintln!("未收到有效翻译结果");
        }
    } else {
        eprintln!("API请求失败: {}", response.status());
    }

    Ok(())
}