mod command;

use clap::{Parser, Subcommand};
use crate::command::copy_command::copy::CopyArgs;
use crate::command::read_command::read::ReadArgs;
use crate::command::size_command::size::SizeArgs;

use crate::command::copy_command::copy;
use crate::command::read_command::read;
use crate::command::size_command::size;


#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    #[command(aliases = &["r"], about = "Read files")]
    Read(ReadArgs),

    #[command(aliases = &["cp"], about = "Copy files, directories")]
    Copy(CopyArgs),

    #[command(aliases = &["s"], about = "Get size of files, directories")]
    Size(SizeArgs),
}


fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Read(args) => read::execute(args),
        Commands::Copy(args) => copy::execute(args),
        Commands::Size(args) => size::execute(args),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}