#![allow(unused)]
use clap::Parser;
use std::{fs::File, path::PathBuf};

const DEFAULT_CONFIG_FILE_NAME: &str = "proc_alarm.json";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long = "config", help = "Use provided config file")]
    config_path: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !sysinfo::IS_SUPPORTED_SYSTEM {
        // Give unsupported system error
        todo!()
    }

    let args = Args::parse();
    let config_path = args.config_path.unwrap_or({
        let mut path = dirs::config_local_dir().expect("Failed to get default config directory");
        path.push(DEFAULT_CONFIG_FILE_NAME);

        path
    });

    let mut config_file = File::options()
        .write(true)
        .create(true)
        .read(true)
        .open(config_path)?;

    // GUI app

    Ok(())
}
