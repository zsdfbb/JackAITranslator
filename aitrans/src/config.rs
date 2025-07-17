// 定义配置文件路径
const CONFIG_TOML_PATH: &str = ".config/aitrans/config.toml";

/* 从配置文件 ~/.config/aitrans/config.toml 读取API参数 */
pub fn read_ai_trans_config() -> Result<(String, String, String), Box<dyn std::error::Error>>  {
    // 读取环境变量 HOME
    let home_dir = std::env::var("HOME")?;
    let config_path = format!("{}/{}", home_dir, CONFIG_TOML_PATH);

    let config = config::Config::builder()
        .add_source(config::File::with_name(config_path.as_str()))
        .build()?;

    let platform = config.get_string("platform")?;
    let model = config.get_string("model")?;
    let api_key = config.get_string("api_key")?;

    return Ok((platform, model, api_key));
}