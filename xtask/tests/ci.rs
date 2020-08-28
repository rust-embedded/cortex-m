use std::process::Command;
use std::{env, str};
use xtask::{check_blobs, install_targets};

static TARGETS: &[&str] = &[
    "thumbv6m-none-eabi",
    "thumbv7m-none-eabi",
    "thumbv7em-none-eabi",
    "thumbv7em-none-eabihf",
    "thumbv8m.base-none-eabi",
    "thumbv8m.main-none-eabi",
    "thumbv8m.main-none-eabihf",
];

fn build(target: &str, features: &[&str]) {
    println!("building for {} {:?}", target, features);
    let mut cargo = Command::new("cargo");
    cargo.args(&["build", "--target", target]);
    for feat in features {
        cargo.args(&["--features", *feat]);
    }

    let status = cargo.status().unwrap();
    assert!(status.success());
}

fn main() {
    // Tests execute in the containing crate's root dir, `cd ..` so that we find `asm` etc.
    env::set_current_dir("..").unwrap();

    install_targets(&mut TARGETS.iter().cloned(), None);

    // Check that the ASM blobs are up-to-date.
    check_blobs();

    let output = Command::new("rustc").arg("-V").output().unwrap();
    let is_nightly = str::from_utf8(&output.stdout).unwrap().contains("nightly");

    // Build `cortex-m` for each supported target.
    for target in TARGETS {
        build(*target, &[]);

        if is_nightly {
            // This may fail when nightly breaks. That's fine, the CI job isn't essential.
            build(*target, &["inline-asm"]);
        }

        if target.starts_with("thumbv7em") {
            // These can target Cortex-M7s, which have an errata workaround.
            build(*target, &["cm7-r0p1"]);

            if is_nightly {
                build(*target, &["inline-asm", "cm7-r0p1"]);
            }
        }
    }
}
