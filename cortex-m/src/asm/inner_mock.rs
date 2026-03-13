#[inline(always)]
pub(crate) unsafe fn __bkpt() {
    unimplemented!()
}

#[inline(always)]
pub(crate) unsafe fn __control_r() -> u32 {
    unimplemented!()
}

#[inline(always)]
pub(crate) unsafe fn __control_w(_w: u32) {
    unimplemented!()
}

#[inline(always)]
pub(crate) unsafe fn __cpsid() {
    unimplemented!()
}

#[inline(always)]
pub(crate) unsafe fn __cpsie() {
    unimplemented!()
}

#[inline(always)]
pub(crate) unsafe fn __delay(_cyc: u32) {
    unimplemented!()
}

#[inline(always)]
pub(crate) unsafe fn __dmb() {
    unimplemented!()
}

#[inline(always)]
pub(crate) unsafe fn __dsb() {
    unimplemented!()
}

#[inline(always)]
pub(crate) unsafe fn __isb() {
    unimplemented!()
}

#[inline(always)]
pub(crate) unsafe fn __msp_r() -> u32 {
    unimplemented!()
}

#[inline(always)]
pub(crate) unsafe fn __msp_w(_val: u32) {
    unimplemented!()
}

// NOTE: No FFI shim, this requires inline asm.
#[inline(always)]
pub(crate) unsafe fn __apsr_r() -> u32 {
    unimplemented!()
}

#[inline(always)]
pub(crate) unsafe fn __nop() {
    unimplemented!()
}

// NOTE: No FFI shim, this requires inline asm.
#[inline(always)]
pub(crate) unsafe fn __pc_r() -> u32 {
    unimplemented!()
}

#[inline(always)]
pub(crate) unsafe fn __pc_w(_val: u32) {
    unimplemented!()
}

#[inline(always)]
pub(crate) unsafe fn __lr_r() -> u32 {
    unimplemented!()
}

#[inline(always)]
pub(crate) unsafe fn __lr_w(_val: u32) {
    unimplemented!()
}

#[inline(always)]
pub(crate) unsafe fn __primask_r() -> u32 {
    unimplemented!()
}

#[inline(always)]
pub(crate) unsafe fn __psp_r() -> u32 {
    unimplemented!()
}

#[inline(always)]
pub(crate) unsafe fn __psp_w(_val: u32) {
    unimplemented!()
}

#[inline(always)]
pub(crate) unsafe fn __sev() {
    unimplemented!()
}

#[inline(always)]
pub(crate) unsafe fn __udf() -> ! {
    unimplemented!()
}

#[inline(always)]
pub(crate) unsafe fn __wfe() {
    unimplemented!()
}

#[inline(always)]
pub(crate) unsafe fn __wfi() {
    unimplemented!()
}

/// Semihosting syscall.
#[inline(always)]
pub(crate) unsafe fn __sh_syscall(mut _nr: u32, _arg: u32) -> u32 {
    unimplemented!()
}

/// Set CONTROL.SPSEL to 0, write `msp` to MSP, branch to `rv`.
#[inline(always)]
pub(crate) unsafe fn __bootstrap(_msp: u32, _rv: u32) -> ! {
    unimplemented!()
}

pub(crate) use v7m::*;

mod v7m {
    #[inline(always)]
    pub(crate) unsafe fn __basepri_max(_val: u8) {
        unimplemented!()
    }

    #[inline(always)]
    pub(crate) unsafe fn __basepri_r() -> u8 {
        unimplemented!()
    }

    #[inline(always)]
    pub(crate) unsafe fn __basepri_w(_val: u8) {
        unimplemented!()
    }

    #[inline(always)]
    pub(crate) unsafe fn __faultmask_r() -> u32 {
        unimplemented!()
    }

    // Should this be safe?
    #[inline(always)]
    pub(crate) unsafe fn __enable_icache() {
        unimplemented!()
    }

    // Should this be safe?
    #[inline(always)]
    pub(crate) unsafe fn __enable_dcache() {
        unimplemented!()
    }
}
