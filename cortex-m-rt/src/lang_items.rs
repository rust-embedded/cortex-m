/// Default panic handler
#[lang = "panic_fmt"]
#[linkage = "weak"]
unsafe extern "C" fn panic_fmt(
    _args: ::core::fmt::Arguments,
    _file: &'static str,
    _line: u32,
) -> ! {
    match () {
        #[cfg(feature = "panic-over-itm")]
        () => {
            use cortex_m::itm;
            use cortex_m::peripheral::ITM;

            let port = &(*ITM.get()).stim[0];
            iprint!(port, "panicked at '");
            itm::write_fmt(port, _args);
            iprintln!(port, "', {}:{}", _file, _line);
        }
        #[cfg(feature = "panic-over-semihosting")]
        () => {
            hprint!("panicked at '");
            ::cortex_m_semihosting::io::write_fmt(_args);
            hprintln!("', {}:{}", _file, _line);
        }
        #[cfg(not(any(feature = "panic-over-itm",
                      feature = "panic-over-semihosting")))]
        () => {}
    }

    asm!("bkpt" :::: "volatile");

    loop {}
}

/// Lang item required to make the normal `main` work in applications
// This is how the `start` lang item works:
// When `rustc` compiles a binary crate, it creates a `main` function that looks
// like this:
//
// ```
// #[export_name = "main"]
// pub extern "C" fn rustc_main(argc: isize, argv: *const *const u8) -> isize {
//     start(main)
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
extern "C" fn start(
    main: fn(),
    _argc: isize,
    _argv: *const *const u8,
) -> isize {
    main();

    0
}
