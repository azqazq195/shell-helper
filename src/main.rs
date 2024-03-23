// TODO 지우기
#![allow(warnings)]
mod command;
mod console_utils;

use clap::{Parser, Subcommand};
use crate::command::copy_command::copy::CopyArgs;
use crate::command::read_command::read::ReadArgs;
use crate::command::size_command::size::SizeArgs;

use crate::command::copy_command::copy;
use crate::command::read_command::read;
use crate::command::size_command::size;
use crate::console_utils::console_utils::message_level;
use crate::console_utils::MessageLevel;


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
    // message("안녕");
    // message_color("안녕", ConsoleColor::Magenta);
    message_level("안녕", MessageLevel::Success);
    message_level("안녕", MessageLevel::Error);
    message_level("안녕", MessageLevel::Warning);
    message_level("hello", MessageLevel::Warning);

    // let cli = Cli::parse();
    //
    // let result = match &cli.command {
    //     Commands::Read(args) => read::execute(args),
    //     Commands::Copy(args) => copy::execute(args),
    //     Commands::Size(args) => size::execute(args),
    // };
    //
    // if let Err(e) = result {
    //     eprintln!("Error: {}", e);
    //     std::process::exit(1);
    // }
}