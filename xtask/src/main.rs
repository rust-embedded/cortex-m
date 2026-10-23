use xtask::{ALL_TARGETS, check_crates_build, check_host_side, install_targets};

#[rustfmt::skip]
fn main() {
    let subcommand = std::env::args().nth(1);
    match subcommand.as_deref() {
        Some("install-targets") => install_targets(ALL_TARGETS),
        Some("check-host-side") => check_host_side(),
        Some("check-crates-build") => {
            let output = std::process::Command::new("rustc")
                .arg("-V")
                .output()
                .unwrap();
            let is_nightly = std::str::from_utf8(&output.stdout).unwrap().contains("nightly");
            check_crates_build(is_nightly)
        }
        _ => {
            eprintln!("usage: cargo xtask <subcommand>");
            eprintln!();
            eprintln!("subcommands:");
            eprintln!("    check-host-side      Build the crate in a non-Cortex-M host application and check host side usage of certain types");
            eprintln!("    check-crates-build   Check the workspace can be built for all known targets");
            eprintln!("    install-targets      Use rustup to install all the appropriate targets");
            eprintln!();
            eprintln!("You can also run 'cargo test' in the ./xtask directory");
            std::process::exit(1);
        }
    }
}
