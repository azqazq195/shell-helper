use clap::Args;
use std::fs;
use std::io;
use std::path::Path;
use log::{info, warn};
use rayon::prelude::*;

const SIZE_UNITS: [&str; 5] = ["Bytes", "KB", "MB", "GB", "TB"];

#[derive(Args)]
pub struct SizeArgs {
    pub path: String,
    #[clap(short = 'h', long = "human-readable")]
    pub human_readable: bool,
}

pub fn execute(args: &SizeArgs) -> io::Result<()> {
    let path = Path::new(&args.path);
    let size = calculate_file_size(path)?;
    let output = if args.human_readable {
        convert_to_human_readable(size)
    } else {
        size.to_string()
    };
    println!("{}", output);
    Ok(())
}

fn calculate_file_size(path: &Path) -> io::Result<u64> {
    let metadata = match fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(e) => {
            warn!("Unable to access metadata for {:?}: {}", path, e);
            return Err(e);
        }
    };
    if metadata.is_dir() {
        calculate_dir_size(path)
    } else {
        info!("File {:?} size: {} bytes", path, metadata.len());
        Ok(metadata.len())
    }
}

fn calculate_dir_size(path: &Path) -> io::Result<u64> {
    let entries = fs::read_dir(path)?
        .filter_map(Result::ok)
        .collect::<Vec<_>>();
    let total_size: u64 = entries.par_iter()
        .map(|entry| {
            let path = entry.path();
            calculate_file_size(&path).unwrap_or_else(|error| {
                warn!("Failed to calculate the size of {:?}: {}", path, error);
                0u64
            })
        })
        .sum();
    Ok(total_size)
}

fn convert_to_human_readable(size: u64) -> String {
    let (size, unit) = calculate_size_and_unit(size);
    format!("{:.1} {}", size, SIZE_UNITS[unit])
}

fn calculate_size_and_unit(mut size: u64) -> (f64, usize) {
    let mut size = size as f64;
    let mut unit = 0;
    while size >= 1024.0 && unit < SIZE_UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }
    (size, unit)
}
