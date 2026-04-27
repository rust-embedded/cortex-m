//! Nested Vector Interrupt Controller
use crate::interrupt::InterruptNumber;
use crate::peripheral::NVIC;

/// NVIC base address.
pub const BASE_ADDRESS: usize = 0xE000_E100;

/// NVIC register block.
#[derive(derive_mmio::Mmio)]
#[repr(C)]
pub struct RegisterBlock {
    /// Interrupt Set-Enable
    iser: [u32; 16],

    _reserved0: [u32; 16],

    /// Interrupt Clear-Enable
    icer: [u32; 16],

    _reserved1: [u32; 16],

    /// Interrupt Set-Pending
    ispr: [u32; 16],

    _reserved2: [u32; 16],

    /// Interrupt Clear-Pending
    icpr: [u32; 16],

    _reserved3: [u32; 16],

    /// Interrupt Active Bit (not present on Cortex-M0 variants)
    #[cfg(not(armv6m))]
    #[mmio(PureRead)]
    iabr: [u32; 16],
    #[cfg(armv6m)]
    _reserved4: [u32; 16],

    _reserved5: [u32; 16],

    #[cfg(armv8m)]
    /// Interrupt Target Non-secure (only present on Arm v8-M)
    itns: [u32; 16],
    #[cfg(not(armv8m))]
    _reserved6: [u32; 16],

    _reserved7: [u32; 16],

    /// Interrupt Priority
    ///
    /// On ARMv7-M, 124 word-sized registers are available. Each of those
    /// contains of 4 interrupt priorities of 8 byte each.The architecture
    /// specifically allows accessing those along byte boundaries.
    ///
    /// On ARMv6-M, the registers must only be accessed along word boundaries,
    /// so convenient byte-sized representation wouldn't work on that
    /// architecture.
    #[cfg(not(armv6m))]
    ipr: [u32; 124],

    /// Interrupt Priority
    ///
    /// On ARMv7-M, 124 word-sized registers are available. Each of those
    /// contains of 4 interrupt priorities of 8 byte each.The architecture
    /// specifically allows accessing those along byte boundaries.
    ///
    /// On ARMv6-M, the registers must only be accessed along word boundaries,
    /// so convenient byte-sized representation wouldn't work on that
    /// architecture.
    #[cfg(armv6m)]
    ipr: [u32; 8],

    #[cfg(not(armv6m))]
    _reserved8: [u32; 580],

    /// Software Trigger Interrupt
    #[cfg(not(armv6m))]
    #[mmio(Write)]
    stir: u32,
}

impl RegisterBlock {
    /// Creates a new instance of the NVIC register block.
    ///
    /// # Safety
    ///
    /// This potentially allows to create multiple instances of the NVIC register block, which
    /// might only be valid in certain multi-core environments.
    #[inline]
    pub const unsafe fn new_mmio_fixed() -> MmioRegisterBlock<'static> {
        unsafe { RegisterBlock::new_mmio_at(BASE_ADDRESS) }
    }
}

impl NVIC {
    /// Request an IRQ in software
    ///
    /// Writing a value to the INTID field is the same as manually pending an interrupt by setting
    /// the corresponding interrupt bit in an Interrupt Set Pending Register. This is similar to
    /// [`NVIC::pend`].
    ///
    /// This method is not available on ARMv6-M chips.
    ///
    /// [`NVIC::pend`]: #method.pend
    #[cfg(not(armv6m))]
    #[inline]
    pub fn request<I>(&mut self, interrupt: I)
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();

        self.write_stir(u32::from(nr));
    }

    /// Disables `interrupt`
    #[inline]
    pub fn mask<I>(interrupt: I)
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        // SAFETY: this is a write to a stateless register.
        let mut nvic = unsafe { Self::steal() };
        // SAFETY: InterruptNumber is an unsafe trait, we can assume correct implementation.
        unsafe { nvic.write_icer_unchecked(usize::from(nr / 32), 1 << (nr % 32)) }
    }

    /// Enables `interrupt`
    ///
    /// This function is `unsafe` because it can break mask-based critical sections
    #[inline]
    pub unsafe fn unmask<I>(interrupt: I)
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        // SAFETY: this is a write to a stateless register,
        let mut nvic = unsafe { Self::steal() };
        // SAFETY: InterruptNumber is an unsafe trait, we can assume correct implementation.
        unsafe { nvic.write_iser_unchecked(usize::from(nr / 32), 1 << (nr % 32)) }
    }

    /// Returns the NVIC priority of `interrupt`
    ///
    /// *NOTE* The NVIC encodes priorities in the *most-significant* bits of the 8-bit block for
    /// each interrupt. This means that the priority value passed to this function MUST be shifted
    /// by (8 - NUMBER_OF_PRIORITY_BITS), where NUMBER_OF_PRIORITY_BITS can be different between
    /// cores. Also for NVIC priorities, a lower value (e.g. `0b0000_0000`) has higher
    /// priority (urgency) than a larger value (e.g. `0b0010_0000`).
    #[inline]
    pub fn get_priority<I>(interrupt: I) -> u8
    where
        I: InterruptNumber,
    {
        #[cfg(not(armv6m))]
        {
            let nr = interrupt.number();
            // SAFETY: atomic read with no side effects
            let nvic = unsafe { Self::steal() };
            let ipr_ptr = nvic.pointer_to_ipr_start() as *const u8;
            // SAFETY:
            //  - atomic read with no side effects
            //  - InterruptNumber is an unsafe trait, we can assume correct implementation.
            unsafe { core::ptr::read_volatile(ipr_ptr.offset(nr as isize)) }
        }

        #[cfg(armv6m)]
        {
            // SAFETY: atomic read with no side effects
            let nvic = unsafe { Self::steal() };
            // SAFETY: InterruptNumber is an unsafe trait, we can assume correct implementation.
            let ipr_n = unsafe { nvic.read_ipr_unchecked(Self::ipr_index(interrupt)) };
            ((ipr_n >> Self::ipr_shift(interrupt)) & 0x0000_00ff) as u8
        }
    }

    /// Is `interrupt` active or pre-empted and stacked
    #[cfg(not(armv6m))]
    #[inline]
    pub fn is_active<I>(interrupt: I) -> bool
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        let mask = 1 << (nr % 32);

        // SAFETY: atomic read with no side effects
        let nvic = unsafe { Self::steal() };
        // SAFETY: InterruptNumber is an unsafe trait, we can assume correct implementation.
        unsafe { nvic.read_iabr_unchecked(usize::from(nr / 32)) & mask == mask }
    }

    /// Checks if `interrupt` is enabled
    #[inline]
    pub fn is_enabled<I>(interrupt: I) -> bool
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        let mask = 1 << (nr % 32);

        // SAFETY: atomic read with no side effects
        let nvic = unsafe { Self::steal() };
        // SAFETY: InterruptNumber is an unsafe trait, we can assume correct implementation.
        unsafe { nvic.read_iser_unchecked(usize::from(nr / 32)) & mask == mask }
    }

    /// Checks if `interrupt` is pending
    #[inline]
    pub fn is_pending<I>(interrupt: I) -> bool
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        let mask = 1 << (nr % 32);

        // SAFETY: atomic read with no side effects
        let nvic = unsafe { Self::steal() };
        // SAFETY: InterruptNumber is an unsafe trait, we can assume correct implementation.
        unsafe { nvic.read_ispr_unchecked(usize::from(nr / 32)) & mask == mask }
    }

    /// Forces `interrupt` into pending state
    #[inline]
    pub fn pend<I>(interrupt: I)
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();

        // SAFETY: atomic stateless write; Register doesn't store any state
        let mut nvic = unsafe { Self::steal() };
        // SAFETY: InterruptNumber is an unsafe trait, we can assume correct implementation.
        unsafe { nvic.write_ispr_unchecked(usize::from(nr / 32), 1 << (nr % 32)) }
    }

    /// Sets the "priority" of `interrupt` to `prio`
    ///
    /// *NOTE* The NVIC encodes priorities in the *most-significant* bits of the 8-bit block for
    /// each interrupt. This means that the priority value passed to this function MUST be shifted
    /// by (8 - NUMBER_OF_PRIORITY_BITS), where NUMBER_OF_PRIORITY_BITS can be different between
    /// cores. Also for NVIC priorities, a lower value (e.g. `0b0000_0000`) has higher
    /// priority (urgency) than a larger value (e.g. `0b0010_0000`).
    ///
    /// On ARMv6-M, updating an interrupt priority requires a read-modify-write operation. On
    /// ARMv7-M, the operation is performed in a single atomic write operation.
    ///
    /// # Unsafety
    ///
    /// Changing priority levels can break priority-based critical sections (see
    /// [`register::basepri`](crate::register::basepri)) and compromise memory safety.
    #[inline]
    pub unsafe fn set_priority<I>(&mut self, interrupt: I, prio: u8)
    where
        I: InterruptNumber,
    {
        #[cfg(not(armv6m))]
        {
            let nr = interrupt.number();
            let ipr_ptr = self.pointer_to_ipr_start() as *mut u8;
            // SAFETY:
            // - atomic stateless write; IPR doesn't store any state
            // - InterruptNumber is an unsafe trait, we can assume correct implementation.
            unsafe { core::ptr::write_volatile(ipr_ptr.offset(nr as isize), prio) }
        }

        #[cfg(armv6m)]
        {
            // SAFETY: InterruptNumber is an unsafe trait, we can assume correct implementation.
            unsafe {
                self.modify_ipr_unchecked(Self::ipr_index(interrupt), |value| {
                    let mask = 0x0000_00ff << Self::ipr_shift(interrupt);
                    let prio = u32::from(prio) << Self::ipr_shift(interrupt);

                    (value & !mask) | prio
                });
            }
        }
    }

    /// Clears `interrupt`'s pending state
    #[inline]
    pub fn unpend<I>(interrupt: I)
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();

        // SAFETY: atomic stateless write; ICPR doesn't store any state
        let mut nvic = unsafe { Self::steal() };
        // SAFETY: InterruptNumber is an unsafe trait, we can assume correct implementation.
        unsafe { nvic.write_icpr_unchecked(usize::from(nr / 32), 1 << (nr % 32)) }
    }

    #[cfg(armv6m)]
    #[inline]
    fn ipr_index<I>(interrupt: I) -> usize
    where
        I: InterruptNumber,
    {
        usize::from(interrupt.number()) / 4
    }

    #[cfg(armv6m)]
    #[inline]
    fn ipr_shift<I>(interrupt: I) -> usize
    where
        I: InterruptNumber,
    {
        (usize::from(interrupt.number()) % 4) * 8
    }
}
