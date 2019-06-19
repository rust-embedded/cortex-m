//! Nested Vector Interrupt Controller

use volatile_register::RW;
#[cfg(not(armv6m))]
use volatile_register::{RO, WO};

use interrupt::Nr;
use peripheral::NVIC;

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// Interrupt Set-Enable
    pub iser: [RW<u32>; 16],

    _reserved0: [u32; 16],

    /// Interrupt Clear-Enable
    pub icer: [RW<u32>; 16],

    _reserved1: [u32; 16],

    /// Interrupt Set-Pending
    pub ispr: [RW<u32>; 16],

    _reserved2: [u32; 16],

    /// Interrupt Clear-Pending
    pub icpr: [RW<u32>; 16],

    _reserved3: [u32; 16],

    /// Interrupt Active Bit (not present on Cortex-M0 variants)
    #[cfg(not(armv6m))]
    pub iabr: [RO<u32>; 16],
    #[cfg(armv6m)]
    _reserved4: [u32; 16],

    _reserved5: [u32; 48],

    #[cfg(not(armv6m))]
    /// Interrupt Priority
    ///
    /// On ARMv7-M, 124 word-sized registers are available. Each of those
    /// contains of 4 interrupt priorities of 8 byte each.The architecture
    /// specifically allows accessing those along byte boundaries, so they are
    /// represented as 496 byte-sized registers, for convenience, and to allow
    /// atomic priority updates.
    ///
    /// On ARMv6-M, the registers must only be accessed along word boundaries,
    /// so convenient byte-sized representation wouldn't work on that
    /// architecture.
    pub ipr: [RW<u8>; 496],

    #[cfg(armv6m)]
    /// Interrupt Priority
    ///
    /// On ARMv7-M, 124 word-sized registers are available. Each of those
    /// contains of 4 interrupt priorities of 8 byte each.The architecture
    /// specifically allows accessing those along byte boundaries, so they are
    /// represented as 496 byte-sized registers, for convenience, and to allow
    /// atomic priority updates.
    ///
    /// On ARMv6-M, the registers must only be accessed along word boundaries,
    /// so convenient byte-sized representation wouldn't work on that
    /// architecture.
    pub ipr: [RW<u32>; 8],

    #[cfg(not(armv6m))]
    _reserved6: [u32; 580],

    #[cfg(not(armv6m))]
    /// Software Trigger Interrupt
    pub stir: WO<u32>,
}

impl NVIC {
    #[cfg(not(armv6m))]
    /// Request an IRQ in software
    ///
    /// Writing a value to the INTID field is the same as manually pending an interrupt by setting
    /// the corresponding interrupt bit in an Interrupt Set Pending Register. This is similar to
    /// `set_pending`.
    ///
    /// This method is not available on ARMv6-M chips.
    pub fn request<I>(&mut self, interrupt: I)
    where
        I: Nr,
    {
        let nr = interrupt.nr();

        unsafe {
            self.stir.write(nr as u32);
        }
    }

    /// Clears `interrupt`'s pending state
    #[deprecated(since = "0.5.8", note = "Use `NVIC::unpend`")]
    pub fn clear_pending<I>(&mut self, interrupt: I)
    where
        I: Nr,
    {
        Self::unpend(interrupt)
    }

    /// Disables `interrupt`
    pub fn mask<I>(interrupt: I)
    where
        I: Nr,
    {
        let nr = interrupt.nr();
        // NOTE(unsafe) this is a write to a stateless register
        unsafe { (*Self::ptr()).icer[usize::from(nr / 32)].write(1 << (nr % 32)) }
    }

    /// Enables `interrupt`
    ///
    /// This function is `unsafe` because it can break mask-based critical sections
    pub unsafe fn unmask<I>(interrupt: I)
    where
        I: Nr,
    {
        let nr = interrupt.nr();
        // NOTE(ptr) this is a write to a stateless register
        (*Self::ptr()).iser[usize::from(nr / 32)].write(1 << (nr % 32))
    }

    /// Disables `interrupt`
    #[deprecated(since = "0.6.1", note = "Use `NVIC::mask`")]
    pub fn disable<I>(&mut self, interrupt: I)
    where
        I: Nr,
    {
        Self::mask(interrupt)
    }

    /// **WARNING** This method is a soundness hole in the API; it should actually be an `unsafe`
    /// function. Use `NVIC::unmask` which has the right unsafety.
    #[deprecated(since = "0.6.1", note = "Use `NVIC::unmask`")]
    pub fn enable<I>(&mut self, interrupt: I)
    where
        I: Nr,
    {
        unsafe { Self::unmask(interrupt) }
    }

    /// Returns the NVIC priority of `interrupt`
    ///
    /// *NOTE* NVIC encodes priority in the highest bits of a byte so values like `1` and `2` map
    /// to the same priority. Also for NVIC priorities, a lower value (e.g. `16`) has higher
    /// priority (urgency) than a larger value (e.g. `32`).
    pub fn get_priority<I>(interrupt: I) -> u8
    where
        I: Nr,
    {
        #[cfg(not(armv6m))]
        {
            let nr = interrupt.nr();
            // NOTE(unsafe) atomic read with no side effects
            unsafe { (*Self::ptr()).ipr[usize::from(nr)].read() }
        }

        #[cfg(armv6m)]
        {
            // NOTE(unsafe) atomic read with no side effects
            let ipr_n = unsafe { (*Self::ptr()).ipr[Self::ipr_index(&interrupt)].read() };
            let prio = (ipr_n >> Self::ipr_shift(&interrupt)) & 0x000000ff;
            prio as u8
        }
    }

    /// Is `interrupt` active or pre-empted and stacked
    #[cfg(not(armv6m))]
    pub fn is_active<I>(interrupt: I) -> bool
    where
        I: Nr,
    {
        let nr = interrupt.nr();
        let mask = 1 << (nr % 32);

        // NOTE(unsafe) atomic read with no side effects
        unsafe { ((*Self::ptr()).iabr[usize::from(nr / 32)].read() & mask) == mask }
    }

    /// Checks if `interrupt` is enabled
    pub fn is_enabled<I>(interrupt: I) -> bool
    where
        I: Nr,
    {
        let nr = interrupt.nr();
        let mask = 1 << (nr % 32);

        // NOTE(unsafe) atomic read with no side effects
        unsafe { ((*Self::ptr()).iser[usize::from(nr / 32)].read() & mask) == mask }
    }

    /// Checks if `interrupt` is pending
    pub fn is_pending<I>(interrupt: I) -> bool
    where
        I: Nr,
    {
        let nr = interrupt.nr();
        let mask = 1 << (nr % 32);

        // NOTE(unsafe) atomic read with no side effects
        unsafe { ((*Self::ptr()).ispr[usize::from(nr / 32)].read() & mask) == mask }
    }

    /// Forces `interrupt` into pending state
    pub fn pend<I>(interrupt: I)
    where
        I: Nr,
    {
        let nr = interrupt.nr();

        // NOTE(unsafe) atomic stateless write; ICPR doesn't store any state
        unsafe { (*Self::ptr()).ispr[usize::from(nr / 32)].write(1 << (nr % 32)) }
    }

    /// Forces `interrupt` into pending state
    #[deprecated(since = "0.5.8", note = "Use `NVIC::pend`")]
    pub fn set_pending<I>(&mut self, interrupt: I)
    where
        I: Nr,
    {
        Self::pend(interrupt)
    }

    /// Sets the "priority" of `interrupt` to `prio`
    ///
    /// *NOTE* See [`get_priority`](struct.NVIC.html#method.get_priority) method for an explanation
    /// of how NVIC priorities work.
    ///
    /// On ARMv6-M, updating an interrupt priority requires a read-modify-write operation. On
    /// ARMv7-M, the operation is performed in a single atomic write operation.
    ///
    /// # Unsafety
    ///
    /// Changing priority levels can break priority-based critical sections (see
    /// [`register::basepri`](../register/basepri/index.html)) and compromise memory safety.
    pub unsafe fn set_priority<I>(&mut self, interrupt: I, prio: u8)
    where
        I: Nr,
    {
        #[cfg(not(armv6m))]
        {
            let nr = interrupt.nr();
            self.ipr[usize::from(nr)].write(prio)
        }

        #[cfg(armv6m)]
        {
            self.ipr[Self::ipr_index(&interrupt)].modify(|value| {
                let mask = 0x000000ff << Self::ipr_shift(&interrupt);
                let prio = u32::from(prio) << Self::ipr_shift(&interrupt);

                (value & !mask) | prio
            })
        }
    }

    /// Clears `interrupt`'s pending state
    pub fn unpend<I>(interrupt: I)
    where
        I: Nr,
    {
        let nr = interrupt.nr();

        // NOTE(unsafe) atomic stateless write; ICPR doesn't store any state
        unsafe { (*Self::ptr()).icpr[usize::from(nr / 32)].write(1 << (nr % 32)) }
    }

    #[cfg(armv6m)]
    fn ipr_index<I>(interrupt: &I) -> usize
    where
        I: Nr,
    {
        usize::from(interrupt.nr()) / 4
    }

    #[cfg(armv6m)]
    fn ipr_shift<I>(interrupt: &I) -> usize
    where
        I: Nr,
    {
        (usize::from(interrupt.nr()) % 4) * 8
    }
}
