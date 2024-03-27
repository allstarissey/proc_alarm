use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Open command-line interface")]
    cli: bool,
}

fn main() {
    if !sysinfo::IS_SUPPORTED_SYSTEM {
        // Give unsupported system error
        todo!()
    }

    let args = Args::parse();

    if args.cli {
        // Launch CLI app
        todo!()
    }

    // GUI app
}
