#![no_main]
#![no_std]

extern crate cortex_m_rt;
use core::sync::atomic::{AtomicBool, Ordering};

#[cfg(target_env = "")] // appease clippy
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    cortex_m::interrupt::disable();
    minitest::log!("{}", info);
    minitest::fail()
}

static PENDSV_FLAG: AtomicBool = AtomicBool::new(false);
static SVCALL_FLAG: AtomicBool = AtomicBool::new(false);
static WANT_FAULT: AtomicBool = AtomicBool::new(false);

const STACK_SIZE_WORDS: usize = 1024;

static STACK: cortex_m::psp::Stack<STACK_SIZE_WORDS> = cortex_m::psp::Stack::new();

#[cortex_m_rt::exception]
fn PendSV() {
    minitest::log!("Hit PendSV!");
    PENDSV_FLAG.store(true, Ordering::SeqCst);
}

#[cortex_m_rt::exception]
fn SVCall() {
    minitest::log!("Handling SWI :)");
    SVCALL_FLAG.store(true, Ordering::SeqCst);
}

#[cortex_m_rt::exception]
unsafe fn HardFault(frame: &cortex_m_rt::ExceptionFrame) -> ! {
    minitest::log!("{:?}", frame);
    if WANT_FAULT.load(Ordering::Relaxed) {
        minitest::log!("Trapped breakpoint OK!");
        minitest::exit()
    } else {
        minitest::fail()
    }
}

#[minitest::tests]
mod tests {
    use crate::{Ordering, PENDSV_FLAG};
    use minitest::log;

    #[init]
    fn init() -> cortex_m::Peripherals {
        log!("Hello world!");
        cortex_m::Peripherals::take().unwrap()
    }

    #[test]
    fn double_take() {
        assert!(cortex_m::Peripherals::take().is_none());
    }

    #[test]
    #[cfg(feature = "rtt")] // QEMU does not model the cycle counter
    fn cycle_count(p: &mut cortex_m::Peripherals) {
        #[cfg(not(armv6m))]
        {
            use cortex_m::peripheral::DWT;

            assert!(p.DWT.has_cycle_counter());

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
        let mut handle = super::STACK.take_handle();
        let top = handle.top();
        let bottom = handle.bottom();
        let delta = unsafe { top.offset_from(bottom) };
        assert_eq!(delta as usize, super::STACK_SIZE_WORDS);
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
        static STACK: cortex_m::psp::Stack<4096> = cortex_m::psp::Stack::new();
        minitest::log!("Switching to PSP...");
        cortex_m::psp::switch_to_unprivileged_psp(STACK.take_handle(), crate::user_fn);
    }
}

/// This code runs on the Process Stack Pointer (i.e. "User mode")
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
