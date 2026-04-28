#![no_main]
#![no_std]

use core::sync::atomic::{AtomicBool, Ordering};
#[cfg(feature = "hardware")]
use defmt_rtt as _;
#[cfg(feature = "qemu")]
use defmt_semihosting as _;

#[cfg(all(not(feature = "hardware"), not(feature = "qemu")))]
compile_error!("Either the `hardware` or `qemu` feature must be enabled");

#[cfg(target_env = "")] // appease clippy
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    cortex_m::interrupt::disable();
    defmt::info!("{}", info);
    fail()
}

static PENDSV_FLAG: AtomicBool = AtomicBool::new(false);
static SVCALL_FLAG: AtomicBool = AtomicBool::new(false);
static WANT_FAULT: AtomicBool = AtomicBool::new(false);

#[cortex_m_rt::exception]
fn PendSV() {
    defmt::info!("Hit PendSV!");
    PENDSV_FLAG.store(true, Ordering::SeqCst);
}

#[cortex_m_rt::exception]
fn SVCall() {
    defmt::info!("Handling SWI :)");
    SVCALL_FLAG.store(true, Ordering::SeqCst);
}

#[cortex_m_rt::exception]
unsafe fn HardFault(frame: &cortex_m_rt::ExceptionFrame) -> ! {
    defmt::info!(
        "Exception Frame {{ r0: {:08x}, r1: {:08x}, r2: {:08x}, r3: {:08x}, r12: {:08x}, lr: {:08x}, pc: {:08x}, xpsr: {:08x} }}",
        frame.r0(),
        frame.r1(),
        frame.r2(),
        frame.r3(),
        frame.r12(),
        frame.lr(),
        frame.pc(),
        frame.xpsr()
    );
    if WANT_FAULT.load(Ordering::Relaxed) {
        defmt::info!("Trapped breakpoint OK!");
        exit()
    } else {
        fail()
    }
}

#[defmt_test::tests]
#[cfg(test)]
mod tests {
    const STACK_SIZE_WORDS: usize = 1024;
    static STACK: cortex_m::psp::Stack<STACK_SIZE_WORDS> = cortex_m::psp::Stack::new();

    use crate::{Ordering, PENDSV_FLAG};
    use defmt::info;

    #[init]
    fn init() -> cortex_m::Peripherals {
        info!("Hello world!");
        cortex_m::Peripherals::take().unwrap()
    }

    #[test]
    fn double_take() {
        assert!(cortex_m::Peripherals::take().is_none());
    }

    #[test]
    #[cfg(feature = "hardware")] // QEMU does not model the cycle counter
    fn cycle_count(p: &mut cortex_m::Peripherals) {
        #[cfg(not(armv6m))]
        {
            use cortex_m::peripheral::DWT;

            assert!(DWT::has_cycle_counter());

            p.DCB.enable_trace();
            p.DWT.disable_cycle_counter();

            const TEST_COUNT: u32 = 0x5555_AAAA;
            p.DWT.set_cycle_count(TEST_COUNT);
            assert_eq!(DWT::cycle_count(), TEST_COUNT);

            p.DWT.enable_cycle_counter();
            assert!(DWT::cycle_count() > TEST_COUNT);
        }

        #[cfg(armv6m)]
        {
            assert!(!p.DWT.has_cycle_counter());
        }
    }

    #[test]
    fn critical_section_nesting() {
        PENDSV_FLAG.store(false, Ordering::SeqCst);
        critical_section::with(|_| {
            critical_section::with(|_| {
                cortex_m::peripheral::SCB::set_pendsv();
                assert!(!PENDSV_FLAG.load(Ordering::SeqCst));
            });
            assert!(!PENDSV_FLAG.load(Ordering::SeqCst));
        });
        assert!(PENDSV_FLAG.load(Ordering::SeqCst));
    }

    #[test]
    fn interrupt_free_nesting() {
        PENDSV_FLAG.store(false, Ordering::SeqCst);
        cortex_m::interrupt::free(|_| {
            cortex_m::interrupt::free(|_| {
                cortex_m::peripheral::SCB::set_pendsv();
                assert!(!PENDSV_FLAG.load(Ordering::SeqCst));
            });
            assert!(!PENDSV_FLAG.load(Ordering::SeqCst));
        });
        assert!(PENDSV_FLAG.load(Ordering::SeqCst));
    }

    #[test]
    fn check_stack_handles() {
        let mut handle = STACK.take_handle();
        let top = handle.top();
        let bottom = handle.bottom();
        let delta = unsafe { top.offset_from(bottom) };
        assert_eq!(delta as usize, STACK_SIZE_WORDS);
    }

    #[test]
    fn check_asm() {
        // Data Memory Barrier - harmless
        cortex_m::asm::dmb();
        // Data Sync Barrier - harmless
        cortex_m::asm::dsb();
        // Instruction Sync Barrier - harmless
        cortex_m::asm::isb();
        // A NOP loop - harmless
        cortex_m::asm::delay(100);
        // A single NOP - harmless
        cortex_m::asm::nop();
        // Set the event flag
        cortex_m::asm::sev();
        // Wait for Event (will not block - flag is set)
        cortex_m::asm::wfe();
        // Pend an interrupt, the wait for it
        cortex_m::peripheral::SCB::set_pendsv();
        cortex_m::interrupt::free(|_| {
            cortex_m::peripheral::SCB::set_pendsv();
            // wfi will turn interrupts back on
            cortex_m::asm::wfi();
        });
        // Print to the debug console with a semihosting syscall
        let msg = c"This is a test\n";
        const SYS_WRITE0: u32 = 0x04;
        unsafe {
            cortex_m::asm::semihosting_syscall(SYS_WRITE0, msg.as_ptr() as usize as u32);
        }
    }

    // this test must be last!
    #[test]
    fn run_psp() {
        static STACK: cortex_m::psp::Stack<2048> = cortex_m::psp::Stack::new();
        info!("Switching to PSP...");
        cortex_m::psp::switch_to_unprivileged_psp(STACK.take_handle(), crate::user_fn);
    }
}

/// This code runs on the Process Stack Pointer (i.e. "User mode")
#[cfg(test)]
extern "C" fn user_fn() -> ! {
    // should not be set
    assert!(!SVCALL_FLAG.load(Ordering::SeqCst));
    // this should fire the SVCall handler
    unsafe {
        core::arch::asm!("swi 0x00");
    }
    // check we hit the SVCall handler
    assert!(SVCALL_FLAG.load(Ordering::SeqCst));
    // now test breakpoints, and exit the tests at the same time
    // (bkpt will trip the HardFault handler)
    crate::WANT_FAULT.store(true, Ordering::Relaxed);
    loop {
        cortex_m::asm::bkpt();
    }
}

/// Stop all tests without failure.
pub fn exit() -> ! {
    cortex_m_semihosting::debug::exit(cortex_m_semihosting::debug::EXIT_SUCCESS);
    unreachable!()
}

/// Stop all tests and report a failure.
pub fn fail() -> ! {
    cortex_m_semihosting::debug::exit(cortex_m_semihosting::debug::EXIT_FAILURE);
    unreachable!()
}
