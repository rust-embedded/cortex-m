//! Process Stack Pointer support

// This is a useful lint for functions like 'asm::wfi()' but it's not a useful
// lint here.
#![allow(clippy::missing_inline_in_public_items)]

use core::cell::UnsafeCell;

/// A stack you can use as your Process Stack (PSP)
///
/// The const-param N is the size **in 32-bit words**
#[repr(align(8), C)]
pub struct Stack<const N: usize> {
    space: UnsafeCell<[u32; N]>,
}

impl<const N: usize> Stack<N> {
    /// Const-initialise a Stack
    ///
    /// Use a turbofish to specify the size, like:
    ///
    /// ```rust
    /// # use cortex_m::psp::Stack;
    /// static PSP_STACK: Stack::<4096> = Stack::new();
    /// ```
    pub const fn new() -> Stack<N> {
        Stack {
            space: UnsafeCell::new([0; N]),
        }
    }

    /// Return the top of the stack
    pub fn get_top(&self) -> *mut u32 {
        let start = self.space.get() as *mut u32;
        unsafe { start.add(N) }
    }
}

unsafe impl<const N: usize> Sync for Stack<N> {}

impl<const N: usize> core::default::Default for Stack<N> {
    fn default() -> Self {
        Stack::new()
    }
}

/// Switch to running on the PSP
#[cfg(cortex_m)]
pub fn switch_to_psp<const N: usize>(psp_stack: &Stack<N>, function: extern "C" fn() -> !) -> ! {
    let stack_top = psp_stack.get_top();
    unsafe { crate::asm::enter_unprivileged(stack_top, function) }
}
