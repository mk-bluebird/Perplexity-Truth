mod debunk;
mod db;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: perplexity-truth \"your claim or question\"");
        std::process::exit(1);
    }

    let query = args[1..].join(" ");

    // Minimal: analyze and return JSON to stdout
    let result = debunk::analyze_claim(&query);
    println!("{}", result);
}
