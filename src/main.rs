use clap::Parser;
use std::process;

mod searcher;

#[derive(Parser)]
struct Args {
    /// Vanity pattern to search for (e.g., "EQabc")
    #[arg(short, long)]
    pattern: String,

    /// Start of key range
    #[arg(short, long)]
    start: u64,

    /// End of key range
    #[arg(short, long)]
    end: u64,

    /// Number of threads
    #[arg(short, long, default_value = "4")]
    threads: usize,
}

fn main() {
    let args = Args::parse();
    match searcher::search_vanity(&args.pattern, args.start, args.end, args.threads) {
        Some((key, addr)) => {
            println!("Found: {} {}", key, addr);
            process::exit(0);
        }
        None => {
            println!("No match found in the range.");
        }
    }
}
