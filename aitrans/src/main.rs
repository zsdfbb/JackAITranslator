use std::env;

mod config;
mod ollama;
mod siliconflow;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从命令行参数获取待翻译文本
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <text_to_translate>", args[0]);
        std::process::exit(1);
    }
    let text = &args[1..].join(" ");

    // 设置API参数
    let (platform, model, api_key) = config::read_ai_trans_config()?;

    // 多平台支持
    match platform.as_str() {
        "siliconflow" => {
            siliconflow::call(text, model, api_key)?;
        }
        "ollama" => {
            ollama::call(text, model, api_key)?;
        }
        _ => {
            eprintln!("Unknown platform: {}", platform);
            std::process::exit(1);
        }
    }

    Ok(())
}