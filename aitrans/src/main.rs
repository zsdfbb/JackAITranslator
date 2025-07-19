use log::{error, debug};

mod config;
mod ollama;
mod siliconflow;

const VERSION: &str = "1.0.0";


const HELP: &str = "\
AITrans - A command-line tool for translating text between languages.

USAGE:
  aitrans --from [input language] --to [output language] [INPUT]
  aitrans [INPUT]           // chinese to english
  aitrans -r [INPUT]        // english to chinese
  aitrans -l, --list        // list all supported languages
  aitrans -v, --version     // Prints version information
  aitrans --debug [INPUT]   // Prints debug information
";

#[derive(Debug)]
struct AppArgs {
    from: String,
    to: String,
    input: String,
    log_level: String,
}

const LANGUAGES: &[&str] = &[
    "English",
    "Chinese",
];

fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut args = AppArgs {
        from: "Chinese".to_string(),
        to: "English".to_string(),
        input: "".to_string(),
        log_level: "info".to_string(),
    };

    let mut pargs = pico_args::Arguments::from_env();

    // Help has a higher priority and should be handled separately.
    if pargs.contains(["-h", "--help"]) {
        println!("{}", HELP);
        std::process::exit(0);
    }

    // Help has a higher priority and should be handled separately.
    if pargs.contains(["-l", "--list"]) {
        for lang in LANGUAGES {
            println!("{}", lang);
        }
        std::process::exit(0);
    }

    if pargs.contains(["-v", "--version"]) {
        println!("{}", VERSION);
        std::process::exit(0);
    }

    if pargs.contains(["-f", "--from"]) {
        let value: Option<String> = pargs.opt_value_from_str("--to").unwrap();
        if let Some(value) = value {
            if !LANGUAGES.contains(&value.as_str()) {
                error!("Invalid language: {}", value);
                std::process::exit(1);
            }
            args.from = value;
        }
    }

    if pargs.contains(["-t", "--to"]) {
        let value: Option<String> = pargs.opt_value_from_str("--to").unwrap();
        if let Some(value) = value {
            if !LANGUAGES.contains(&value.as_str()) {
                error!("Invalid language: {}", value);
                std::process::exit(1);
            }
            args.to = value;
        }
    }

    if pargs.contains(["-r", "--reverse"]) {
        use std::mem::swap;
        swap(&mut args.from, &mut args.to);
    }

    if pargs.contains(["-d", "--debug"]) {
        args.log_level = "debug".to_string();
    }

    args.input = pargs.free_from_str()?;

    Ok(args)
}

fn logger_init(log_level: String) {
    let mut elog_builder = env_logger::Builder::new();

    if log_level == "debug" {
        elog_builder.filter(None, log::LevelFilter::Debug);
        debug!("Debug mode enabled.")
    } else {
        elog_builder.filter(None, log::LevelFilter::Info);
    }

    elog_builder.init();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /* 解析命令行参数 */
    let args = parse_args()?;

    logger_init(args.log_level);

    // 设置API参数
    let (platform, model, api_key) = config::read_ai_trans_config()?;

    // 多平台支持
    match platform.as_str() {
        "siliconflow" => {
            debug!("use siliconflow to translate");
            siliconflow::call(&args.input.as_str(), model, api_key, args.from, args.to)?;
        }
        "ollama" => {
            debug!("Use ollama to translate");
            ollama::call(&args.input.as_str(), model, api_key, args.from, args.to)?;
        }
        _ => {
            error!("Unknown platform: {}", platform);
            std::process::exit(1);
        }
    }

    Ok(())
}