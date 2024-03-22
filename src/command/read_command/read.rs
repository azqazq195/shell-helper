use clap::Args;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

// Extracted constant for sleep duration
const SLEEP_DURATION: Duration = Duration::from_millis(500);

#[derive(Args)]
pub struct ReadArgs {
    pub path: String,
    #[clap(short = 'f', long = "follow")]
    pub follow: bool,
}

pub fn execute(args: &ReadArgs) -> io::Result<()> {
    if args.follow {
        follow_tail(&args.path)
    } else {
        print_file_content(&args.path)
    }.expect("TODO: panic message");
    Ok(())
}

fn print_file_content(path: &str) -> io::Result<()> {
    let reader = get_buffered_reader(path)?;

    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}

// Renamed tail_f to follow_tail
fn follow_tail(path: &str) -> io::Result<()> {
    let mut reader = get_buffered_reader(path)?;
    loop {
        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            sleep(SLEEP_DURATION);
        } else {
            print!("{}", line);
        }
        if bytes_read > 0 && !line.ends_with('\n') {
            println!(); // print newline after the last line
        }
    }
}

fn get_buffered_reader(path: &str) -> io::Result<BufReader<File>> {
    let filepath = PathBuf::from(path);
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    Ok(reader)
}
