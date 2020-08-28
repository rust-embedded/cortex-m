use std::{env, process};
use xtask::{assemble_blobs, check_blobs};

fn main() {
    let subcommand = env::args().skip(1).next();
    match subcommand.as_ref().map(|s| &**s) {
        Some("assemble") => assemble_blobs(),
        Some("check-blobs") => check_blobs(),
        _ => {
            eprintln!("usage: cargo xtask <subcommand>");
            eprintln!();
            eprintln!("subcommands:");
            eprintln!("    assemble     Reassemble the pre-built artifacts");
            eprintln!("    check-blobs  Check that the pre-built artifacts are up-to-date and reproducible");
            process::exit(1);
        }
    }
}
