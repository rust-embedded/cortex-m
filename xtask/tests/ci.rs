use std::process::Command;
use std::{env, str};
use xtask::{check_blobs, install_targets};

/// List of all compilation targets we support.
///
/// This should generally list all of the bare-metal thumb targets starting at thumbv6.
static ALL_TARGETS: &[&str] = &[
    "thumbv6m-none-eabi",
    "thumbv7m-none-eabi",
    "thumbv7em-none-eabi",
    "thumbv7em-none-eabihf",
    "thumbv8m.base-none-eabi",
    "thumbv8m.main-none-eabi",
    "thumbv8m.main-none-eabihf",
];

/// Same as the list above, except with all "base" targets that have a reduced feature set removed.
static NON_BASE_TARGETS: &[&str] = &[
    "thumbv7m-none-eabi",
    "thumbv7em-none-eabi",
    "thumbv7em-none-eabihf",
    "thumbv8m.main-none-eabi",
    "thumbv8m.main-none-eabihf",
];

fn build(package: &str, target: &str, features: &[&str]) {
    println!("building {} for {} {:?}", package, target, features);
    let mut cargo = Command::new("cargo");
    cargo.args(&["build", "-p", package, "--target", target]);
    for feat in features {
        cargo.args(&["--features", *feat]);
    }

    // Cargo features don't work right when invoked from the workspace root, so change to the
    // package's directory when necessary.
    if package != "cortex-m" {
        cargo.current_dir(package);
    }

    let status = cargo.status().unwrap();
    assert!(status.success(), "failed to execute: {:?}", cargo);
}

#[rustfmt::skip]
static PACKAGE_FEATURES: &[(&str, &[&str], &[&str])] = &[
    ("cortex-m", ALL_TARGETS, &["inline-asm", "cm7-r0p1"]), // no `linker-plugin-lto` since it's experimental
    ("cortex-m-semihosting", ALL_TARGETS, &["inline-asm", "no-semihosting", "jlink-quirks"]),
    ("panic-semihosting", ALL_TARGETS, &["inline-asm", "exit", "jlink-quirks"]),
    ("panic-itm", NON_BASE_TARGETS, &[]),
];

fn check_crates_build(is_nightly: bool) {
    // Build all crates for each supported target.
    for (package, targets, all_features) in PACKAGE_FEATURES {
        for target in *targets {
            // Filters crate features, keeping only those that are supported.
            // Relies on all crates in this repo to use the same convention.
            let should_use_feature = |feat: &str| {
                match feat {
                    // This is nightly-only, so don't use it on stable.
                    "inline-asm" => is_nightly,
                    // This only affects thumbv7em targets.
                    "cm7-r0p1" => target.starts_with("thumbv7em"),

                    _ => true,
                }
            };

            // Every crate must build with the default feature set.
            build(package, target, &[]);

            let used_features = &*all_features
                .iter()
                .copied()
                .filter(|feat| should_use_feature(*feat))
                .collect::<Vec<_>>();

            // (note: we don't test with default features disabled, since we don't use them yet)

            // Every crate must build with each individual feature enabled.
            for feat in used_features {
                build(package, target, &[*feat]);
            }

            // Every crate must build with *all* features enabled.
            build(package, target, used_features);

            // (technically we should be checking the powerset of all features if we wanted to be
            // *really* sure, but that takes too much time and isn't very easy to implement)
        }
    }
}

fn main() {
    // Tests execute in the containing crate's root dir, `cd ..` so that we find `asm` etc.
    env::set_current_dir("..").unwrap();

    install_targets(&mut ALL_TARGETS.iter().cloned(), None);

    // Check that the ASM blobs are up-to-date.
    check_blobs();

    let output = Command::new("rustc").arg("-V").output().unwrap();
    let is_nightly = str::from_utf8(&output.stdout).unwrap().contains("nightly");

    check_crates_build(is_nightly);
}
