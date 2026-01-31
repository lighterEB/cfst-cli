use clap::Parser;
use cfst::{Cli, run};
fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(&cli) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
