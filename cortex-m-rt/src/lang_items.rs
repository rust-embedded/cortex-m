/// Default panic handler
#[cfg(feature = "abort-on-panic")]
#[lang = "panic_fmt"]
unsafe extern "C" fn panic_fmt(_: ::core::fmt::Arguments, _: &'static str, _: u32, _: u32) -> ! {
    ::core::intrinsics::abort()
}

/// Lang item required to make the normal `main` work in applications
// This is how the `start` lang item works:
// When `rustc` compiles a binary crate, it creates a `main` function that looks
// like this:
//
// ```
// #[export_name = "main"]
// pub extern "C" fn rustc_main(argc: isize, argv: *const *const u8) -> isize {
//     start(main, argc, argv)
// }
// ```
//
// Where `start` is this function and `main` is the binary crate's `main`
// function.
//
// The final piece is that the entry point of our program, the reset handler,
// has to call `rustc_main`. That's covered by the `reset_handler` function in
// root of this crate.
#[lang = "start"]
extern "C" fn start<T>(main: fn() -> T, _argc: isize, _argv: *const *const u8) -> isize
where
    T: Termination,
{
    main();

    0
}

#[lang = "termination"]
pub trait Termination {
    fn report(self) -> i32;
}

impl Termination for () {
    fn report(self) -> i32 {
        0
    }
}
