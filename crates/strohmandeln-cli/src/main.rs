use std::io;
use strohmandeln_cli::{parse_args, run};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut options = match parse_args(&args) {
        Ok(o) => o,
        Err(why) => {
            eprintln!("{why}");
            std::process::exit(2);
        }
    };
    if !args.iter().any(|a| a == "--seed") {
        options.seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(1, |d| d.as_secs() % 1_000_000);
    }
    let stdin = io::stdin();
    if let Err(e) = run(&options, &mut stdin.lock(), &mut io::stdout()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
