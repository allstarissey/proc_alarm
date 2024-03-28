use std::path::PathBuf;
use clap::Parser;

const DEFAULT_CONFIG_FILE_NAME: &str = "proc_alarm.json";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short='C', long, help = "Open command-line interface")]
    cli: bool,

    #[arg(short, long="config", help = "Use provided config file")]
    config_path: Option<PathBuf>
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !sysinfo::IS_SUPPORTED_SYSTEM {
        // Give unsupported system error
        todo!()
    }

    let args = Args::parse();
    let config_path = match args.config_path {
        Some(c) => c,
        None => {
            let mut path = dirs::config_local_dir().expect("Failed to get default config directory");
            path.push(DEFAULT_CONFIG_FILE_NAME);

            path
        }
    };

    if args.cli {
        // Launch CLI app
        todo!()
    }

    // GUI app

    Ok(())
}
