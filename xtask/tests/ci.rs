use std::process::Command;
use xtask::{ALL_TARGETS, check_crates_build, check_host_side, install_targets};

fn main() {
    // Tests execute in the containing crate's root dir, `cd ..` so that we find `asm` etc.
    std::env::set_current_dir("..").unwrap();

    install_targets(ALL_TARGETS);

    let output = Command::new("rustc").arg("-V").output().unwrap();
    let is_nightly = std::str::from_utf8(&output.stdout)
        .unwrap()
        .contains("nightly");

    check_crates_build(is_nightly);

    // Check host-side applications of the crate.
    check_host_side();
}
