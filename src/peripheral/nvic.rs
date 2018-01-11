//! Nested Vector Interrupt Controller

use volatile_register::{RO, RW};

use peripheral::NVIC;
use interrupt::Nr;

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// Interrupt Set-Enable
    pub iser: [RW<u32>; 16],
    reserved0: [u32; 16],
    /// Interrupt Clear-Enable
    pub icer: [RW<u32>; 16],
    reserved1: [u32; 16],
    /// Interrupt Set-Pending
    pub ispr: [RW<u32>; 16],
    reserved2: [u32; 16],
    /// Interrupt Clear-Pending
    pub icpr: [RW<u32>; 16],
    reserved3: [u32; 16],
    /// Interrupt Active Bit
    pub iabr: [RO<u32>; 16],
    reserved4: [u32; 48],

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
}

impl NVIC {
    /// Clears `interrupt`'s pending state
    pub fn clear_pending<I>(&mut self, interrupt: I)
    where
        I: Nr,
    {
        let nr = interrupt.nr();

        unsafe { self.icpr[usize::from(nr / 32)].write(1 << (nr % 32)) }
    }

    /// Disables `interrupt`
    pub fn disable<I>(&mut self, interrupt: I)
    where
        I: Nr,
    {
        let nr = interrupt.nr();

        unsafe { self.icer[usize::from(nr / 32)].write(1 << (nr % 32)) }
    }

    /// Enables `interrupt`
    pub fn enable<I>(&mut self, interrupt: I)
    where
        I: Nr,
    {
        let nr = interrupt.nr();

        unsafe { self.iser[usize::from(nr / 32)].write(1 << (nr % 32)) }
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
    pub fn set_pending<I>(&mut self, interrupt: I)
    where
        I: Nr,
    {
        let nr = interrupt.nr();

        unsafe { self.ispr[usize::from(nr / 32)].write(1 << (nr % 32)) }
    }

    /// Sets the "priority" of `interrupt` to `prio`
    ///
    /// *NOTE* See [`get_priority`](struct.NVIC.html#method.get_priority) method for an explanation
    /// of how NVIC priorities work.
    ///
    /// On ARMv6-M, updating an interrupt priority requires a read-modify-write operation. On
    /// ARMv7-M, the operation is performed in a single atomic write operation.
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
