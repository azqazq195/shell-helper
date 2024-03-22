use clap::Args;
use std::io::{self};
use crate::command::copy_command::copy_local::copy as copy_local;
use crate::command::copy_command::copy_remote::copy as copy_remote;

#[derive(Args)]
pub struct CopyArgs {
    pub from: String,
    pub to: String,
    // #[clap(short = 'r', long = "remote")]
    // pub remote: bool
}

pub fn execute(args: &CopyArgs) -> io::Result<()> {
    copy_local(&args)
    // match args.remote {
    //     true => copy_remote(&args),
    //     false => copy_local(&args)
    // }
}

