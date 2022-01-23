use std::{env, process};
use xtask::{assemble_blobs, check_blobs, check_host_side};

fn main() {
    let subcommand = env::args().nth(1);
    match subcommand.as_deref() {
        Some("assemble") => assemble_blobs(),
        Some("check-blobs") => check_blobs(),
        Some("check-host-side") => check_host_side(),
        _ => {
            eprintln!("usage: cargo xtask <subcommand>");
            eprintln!();
            eprintln!("subcommands:");
            eprintln!("    assemble         Reassemble the pre-built artifacts");
            eprintln!("    check-blobs      Check that the pre-built artifacts are up-to-date and reproducible");
            eprintln!("    check-host-side  Build the crate in a non-Cortex-M host application and check host side usage of certain types");
            process::exit(1);
        }
    }
}
