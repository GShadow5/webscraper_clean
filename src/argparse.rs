use crate::{json_code, named};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[derive(Debug)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE")]
    pub folder: Option<PathBuf>,

    #[arg(short, long, value_name = "URL")]
    pub url: Option<String>,

    #[arg(short, long)]
    pub scan: bool,

    // Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbosity: u8,

    #[arg(short, long)]
    pub download: bool,

    // Turn testing mode on
    #[arg(short, long)]
    pub test: bool,

    #[arg(short, long)]
    pub create_config_file: bool,
}

#[derive(Debug)]
pub struct Config {
    pub verbosity: u8,
    pub url: String,
    pub scan: bool,
    pub image_directory: PathBuf,
    pub download: bool,
    pub test: bool,
}

#[named]
pub fn parse_args() -> Config {
    // If config file is present, read it and use it to override the command line arguments
    if let Ok(json) = std::fs::read_to_string("config.json") {
        let json: json::JsonValue = match json::parse(&json) {
            Ok(json) => json,
            Err(e) => {
                println!("fn {} - Error parsing config.json: {}", function_name!(), e);
                std::process::exit(1);
            }
        };
        let config = json_code::parse_config_json(json);
        return config;
    }
    // Parse command line arguments
    let cli = Cli::parse();
    let mut config = Config {
        download: false,
        url: String::from(""),
        scan: false,
        image_directory: PathBuf::from("images"),
        test: false,
        verbosity: 0,
    };

    if cli.scan == false && cli.download == false {
        println!(
            "fn {} - You must specify either --scan, --download, or --help",
            function_name!()
        );
        std::process::exit(1);
    }
    if let Some(path) = cli.folder.as_deref() {
        config.image_directory = path.to_path_buf();
    }
    if cli.scan {
        config.scan = cli.scan;
    }
    if let Some(url) = cli.url.as_deref() {
        config.url = url.to_string();
    }
    if cli.test {
        config.test = cli.test;
    }

    if cli.download {
        config.download = cli.download;
    }

    // If the image directory is the default, create it if it doesn't exist
    if config.image_directory.eq(&PathBuf::from("images")) {
        if config.image_directory.exists() == false {
            std::fs::create_dir(&config.image_directory).unwrap();
        }
    }

    // If create_config_file is true, create the config file and exit
    if cli.create_config_file {
        let json = json_code::make_config_json(config);
        let json = json.pretty(2);
        std::fs::write("config.json", json).unwrap();
        println!("fn {} - Created config file", function_name!());
        std::process::exit(0);
    }
    config
}
