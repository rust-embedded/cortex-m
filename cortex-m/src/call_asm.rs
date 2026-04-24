/// An internal macro to invoke an assembly routine.
///
/// Depending on whether the unstable `inline-asm` feature is enabled, this will either call into
/// the inline assembly implementation directly, or through the FFI shim (see `asm/lib.rs`).
macro_rules! call_asm {
    ( $func:ident ( $($args:ident: $tys:ty),* ) $(-> $ret:ty)? ) => {{
        #[allow(unused_unsafe)] // The inline-asm path may not require unsafe, but the FFI path does; both share this block.
        // SAFETY: Delegates to either inline assembly intrinsics (whose preconditions are
        // upheld by the caller of the public wrapper) or to FFI symbols provided by the
        // pre-compiled assembly blobs shipped with this crate.
        unsafe {
            match () {
                #[cfg(feature = "inline-asm")]
                () => crate::asm::inline::$func($($args),*),

                #[cfg(not(feature = "inline-asm"))]
                () => {
                    extern "C" {
                        fn $func($($args: $tys),*) $(-> $ret)?;
                    }

                    $func($($args),*)
                },
            }
        }
    }};
}
