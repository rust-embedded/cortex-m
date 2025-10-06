//! Process Stack Pointer support

// This is a useful lint for functions like 'asm::wfi()' but it's not a useful
// lint here.
#![allow(clippy::missing_inline_in_public_items)]

use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
};

/// Represents access to a [`Stack`]
pub struct StackHandle(*mut u32, usize);

impl StackHandle {
    /// Get the pointer to the top of the stack
    pub fn top(&mut self) -> *mut u32 {
        // SAFETY: The stack was this big when we constructed the handle
        unsafe { self.0.add(self.1) }
    }

    /// Get the pointer to the bottom of the stack
    pub fn bottom(&mut self) -> *mut u32 {
        self.0
    }
}

/// A stack you can use as your Process Stack (PSP)
///
/// The const-param N is the size **in 32-bit words**
#[repr(align(8), C)]
pub struct Stack<const N: usize> {
    space: UnsafeCell<[u32; N]>,
    taken: AtomicBool,
}

impl<const N: usize> Stack<N> {
    /// Const-initialise a Stack
    ///
    /// Use a turbofish to specify the size, like:
    ///
    /// ```rust
    /// # use cortex_m::psp::Stack;
    /// static PSP_STACK: Stack::<4096> = Stack::new();
    /// fn example() {
    ///    let handle = PSP_STACK.take_handle();
    ///    // ...
    /// }
    /// ```
    pub const fn new() -> Stack<N> {
        Stack {
            space: UnsafeCell::new([0; N]),
            taken: AtomicBool::new(false),
        }
    }

    /// Return the top of the stack
    pub fn take_handle(&self) -> StackHandle {
        if self.taken.load(Ordering::Acquire) {
            panic!("Cannot get two handles to one stack!");
        }
        self.taken.store(true, Ordering::Release);

        let start = self.space.get() as *mut u32;
        StackHandle(start, N)
    }
}

unsafe impl<const N: usize> Sync for Stack<N> {}

impl<const N: usize> core::default::Default for Stack<N> {
    fn default() -> Self {
        Stack::new()
    }
}

/// Switch to unprivileged mode running on the Process Stack Pointer (PSP)
///
/// In Unprivileged Mode, code can no longer perform privileged operations,
/// such as disabling interrupts.
///
#[cfg(cortex_m)]
pub fn switch_to_unprivileged_psp(mut psp_stack: StackHandle, function: extern "C" fn() -> !) -> ! {
    // set the stack limit
    #[cfg(armv8m_main)]
    unsafe {
        crate::register::psplim::write(psp_stack.bottom() as u32);
    }
    // do the switch
    unsafe {
        crate::asm::enter_unprivileged_psp(psp_stack.top(), function);
    }
}

/// Switch to running on the Process Stack Pointer (PSP), but remain in privileged mode
#[cfg(cortex_m)]
pub fn switch_to_privileged_psp(mut psp_stack: StackHandle, function: extern "C" fn() -> !) -> ! {
    // set the stack limit
    #[cfg(armv8m_main)]
    unsafe {
        crate::register::psplim::write(psp_stack.bottom() as u32);
    }
    // do the switch
    unsafe {
        crate::asm::enter_privileged_psp(psp_stack.top(), function);
    }
}
