use std::{env, process};
use xtask::check_host_side;

fn main() {
    let subcommand = env::args().nth(1);
    match subcommand.as_deref() {
        Some("check-host-side") => check_host_side(),
        _ => {
            eprintln!("usage: cargo xtask <subcommand>");
            eprintln!();
            eprintln!("subcommands:");
            eprintln!("    check-host-side  Build the crate in a non-Cortex-M host application and check host side usage of certain types");
            process::exit(1);
        }
    }
}
