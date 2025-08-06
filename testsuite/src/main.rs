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

static EXCEPTION_FLAG: AtomicBool = AtomicBool::new(false);

#[cortex_m_rt::exception]
fn PendSV() {
    EXCEPTION_FLAG.store(true, Ordering::SeqCst);
}

#[minitest::tests]
mod tests {
    use crate::{Ordering, EXCEPTION_FLAG};
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
        EXCEPTION_FLAG.store(false, Ordering::SeqCst);
        critical_section::with(|_| {
            critical_section::with(|_| {
                cortex_m::peripheral::SCB::set_pendsv();
                assert!(!EXCEPTION_FLAG.load(Ordering::SeqCst));
            });
            assert!(!EXCEPTION_FLAG.load(Ordering::SeqCst));
        });
        assert!(EXCEPTION_FLAG.load(Ordering::SeqCst));
    }

    #[test]
    fn interrupt_free_nesting() {
        EXCEPTION_FLAG.store(false, Ordering::SeqCst);
        cortex_m::interrupt::free(|_| {
            cortex_m::interrupt::free(|_| {
                cortex_m::peripheral::SCB::set_pendsv();
                assert!(!EXCEPTION_FLAG.load(Ordering::SeqCst));
            });
            assert!(!EXCEPTION_FLAG.load(Ordering::SeqCst));
        });
        assert!(EXCEPTION_FLAG.load(Ordering::SeqCst));
    }
}
