//! Nested Vector Interrupt Controller

use volatile_register::{RO, RW};

use interrupt::Nr;

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// Interrupt Set-Enable
    pub iser: [RW<u32>; 8],
    reserved0: [u32; 24],
    /// Interrupt Clear-Enable
    pub icer: [RW<u32>; 8],
    reserved1: [u32; 24],
    /// Interrupt Set-Pending
    pub ispr: [RW<u32>; 8],
    reserved2: [u32; 24],
    /// Interrupt Clear-Pending
    pub icpr: [RW<u32>; 8],
    reserved3: [u32; 24],
    /// Interrupt Active Bit
    pub iabr: [RO<u32>; 8],
    reserved4: [u32; 56],
    /// Interrupt Priority
    pub ipr: [RW<u8>; 240],
}

impl RegisterBlock {
    /// Clears `interrupt`'s pending state
    pub fn clear_pending<I>(&self, interrupt: I)
    where
        I: Nr,
    {
        let nr = interrupt.nr();

        unsafe { self.icpr[usize::from(nr / 32)].write(1 << (nr % 32)) }
    }

    /// Disables `interrupt`
    pub fn disable<I>(&self, interrupt: I)
    where
        I: Nr,
    {
        let nr = interrupt.nr();

        unsafe { self.icer[usize::from(nr / 32)].write(1 << (nr % 32)) }
    }

    /// Enables `interrupt`
    pub fn enable<I>(&self, interrupt: I)
    where
        I: Nr,
    {
        let nr = interrupt.nr();

        unsafe { self.iser[usize::from(nr / 32)].write(1 << (nr % 32)) }
    }

    /// Gets the "priority" of `interrupt`
    ///
    /// NOTE NVIC encodes priority in the highest bits of a byte so values like
    /// `1` and `2` have the same priority. Also for NVIC priorities, a lower
    /// value (e.g. `16`) has higher priority than a larger value (e.g. `32`).
    pub fn get_priority<I>(&self, interrupt: I) -> u8
    where
        I: Nr,
    {
        let nr = interrupt.nr();

        self.ipr[usize::from(nr)].read()
    }

    /// Is `interrupt` active or pre-empted and stacked
    pub fn is_active<I>(&self, interrupt: I) -> bool
    where
        I: Nr,
    {
        let nr = interrupt.nr();
        let mask = 1 << (nr % 32);

        (self.iabr[usize::from(nr / 32)].read() & mask) == mask
    }

    /// Checks if `interrupt` is enabled
    pub fn is_enabled<I>(&self, interrupt: I) -> bool
    where
        I: Nr,
    {
        let nr = interrupt.nr();
        let mask = 1 << (nr % 32);

        (self.iser[usize::from(nr / 32)].read() & mask) == mask
    }

    /// Checks if `interrupt` is pending
    pub fn is_pending<I>(&self, interrupt: I) -> bool
    where
        I: Nr,
    {
        let nr = interrupt.nr();
        let mask = 1 << (nr % 32);

        (self.ispr[usize::from(nr / 32)].read() & mask) == mask
    }

    /// Forces `interrupt` into pending state
    pub fn set_pending<I>(&self, interrupt: I)
    where
        I: Nr,
    {
        let nr = interrupt.nr();

        unsafe { self.ispr[usize::from(nr / 32)].write(1 << (nr % 32)) }
    }

    /// Sets the "priority" of `interrupt` to `prio`
    ///
    /// NOTE See `get_priority` method for an explanation of how NVIC priorities
    /// work.
    pub unsafe fn set_priority<I>(&self, interrupt: I, prio: u8)
    where
        I: Nr,
    {
        let nr = interrupt.nr();

        self.ipr[usize::from(nr)].write(prio)
    }
}
