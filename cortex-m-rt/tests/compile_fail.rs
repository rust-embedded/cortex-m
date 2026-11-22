#[test]
fn compile_fail() {
    // The test programs are `#![no_std]` binaries built for the host, which
    // requires `panic = "abort"`. trybuild doesn't forward `RUSTFLAGS`, but
    // the cargo it spawns does read profile settings from the environment.
    //
    // SAFETY: this is the only test in this binary, so no other threads are
    // accessing the environment concurrently.
    unsafe { std::env::set_var("CARGO_PROFILE_DEV_PANIC", "abort") };

    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile-fail/*.rs");
    // trybuild only runs `cargo check` on compile-fail tests, unless there is
    // at least one pass test, in which case it runs `cargo build` on all of
    // them. Several tests check for duplicate symbol errors, which are only
    // reported during codegen, so we need a full build.
    t.pass("tests/pass/empty.rs");
}
