Migrating from cortex-m 0.7 to 0.8
======

## Overview and reasoning

When writing Rust code for Cortex-M based systems, the `cortex-m` crate will usually be
part of the dependency graph in some shape or form. It is often transitively included by PACs
that implement the
[`InterruptNumber`](https://docs.rs/cortex-m/0.7/cortex_m/interrupt/trait.InterruptNumber.html)
trait but it also might be included to get access to its `asm` module or import a
[`critical-section`](https://github.com/rust-embedded/critical-section) [implementation](https://docs.rs/cortex-m/latest/cortex_m/index.html#critical-section-single-core).

Additionally, only one version of `cortex-m` may exist in the dependency graph due to the
[`links`](https://github.com/rust-embedded/cortex-m/blob/c-m-v0.7.10/cortex-m/Cargo.toml#L12) manifest key.
This key might be dropped in future versions, but it will remain part of v0.7.
The combination of the prevalence of this crate and requiring one version currently makes any breaking
release something that can break the ecosystem. There are multiple reasons why a version bump is
still worth it:

1. A stable release of fundamental crates like `cortex-m` is a [stated project goal](https://github.com/rust-embedded/wg/issues/383). A release of v0.8 is an important step towards that.
2. Important breaking changes are piling up. A planned version change allows merging those after
   a last patch release which currently is [v0.7.10](https://github.com/rust-embedded/cortex-m/releases/tag/c-m-v0.7.10).
3. There has been some effort to make changes to `cortex-m` simpler in the future by introducing
   a [shared type crate](https://docs.rs/cortex-m-types/latest/cortex_m_types/). The highest transition cost is
   mostly paid once.

This migration guide also provides dependents with information on how to make this transition as
smooth as possible for different users.

## PAC authors / PAC generation

To make future changes of `cortex-m` less painful, some of the fundamental types were moved
into the [`cortex-m-types` crate](https://docs.rs/cortex-m-types/latest/cortex_m_types/).
This includes traits like [`InterruptNumber`](https://docs.rs/cortex-m-types/latest/cortex_m_types/trait.InterruptNumber.html)
which are the abstraction expected by some other `cortex-m` components like the [NVIC driver API](https://docs.rs/cortex-m/latest/cortex_m/peripheral/struct.NVIC.html).

Most PACs are auto-generated. The most common auto-generator tools in the ecosystem are
[`svd2rust`](https://github.com/rust-embedded/svd2rust) and [`chiptool`](https://github.com/embassy-rs/chiptool).

`svd2rust` received support for the new trait through patches [#959](https://github.com/rust-embedded/svd2rust/pull/959) and [#962](https://github.com/rust-embedded/svd2rust/pull/962). Once `svd2rust` v0.38
is released, it should be sufficient to re-generate your PAC with the new version.

`chiptool` also received a patch ([#144](https://github.com/embassy-rs/chiptool/pull/144)) which
adds generation for the new trait implementation. Once that has landed, re-generation with the
new version should be sufficient.

This preparation work can already be done before a new `cortex-m` version is released.
For manual implementations, the following example shows the old and the new trait implementation.

Old trait implementation:

```rust
/// Enumeration of all the interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0 - UART0 interrupt
    UART0 = 0,
    /// 1 - TIMER0 interrupt
    TIMER0 = 1,
    /// 3 - GPIO interrupt
    GPIO = 3,
    /// ... more interrupts.
}

unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
```

New trait implementation:

```rust
unsafe impl cortex_m_types::InterruptNumber for Interrupt {
    const MAX_INTERRUPT_NUMBER: usize = 3;

    #[inline(always)]
    fn number(self) -> usize {
        self as usize
    }

    /// Tries to convert a number to a valid interrupt.
    #[inline]
    fn from_number(value: usize) -> cortex_m_types::result::Result<Self> {
        if value > Self::MAX_INTERRUPT_NUMBER {
            return Err(cortex_m_types::result::Error::IndexOutOfBounds {
                index: value,
                min: 0,
                max: Self::MAX_INTERRUPT_NUMBER,
            });
        }
        match value {
            0 => Ok(Interrupt::UART0),
            1 => Ok(Interrupt::TIMER0),
            3 => Ok(Interrupt::GPIO),
            _ => Err(cortex_m_types::result::Error::InvalidVariant(value)),
        }
    }
}
```

### Reasoning

This trait is usually implemented in the PAC, which also provides the interrupt handlers.
`cortex-m` and the PAC have to depend on the same version of the trait. The trait changes a lot
less frequently than the crate itself, so a common solution is to move the trait
definitions into a separate crate. Both `cortex-m` and the PACs can now depend on the shared type
crate, which allows `cortex-m` to change as long as the type crate version remains the same.

## HAL authors

TODO

## Application authors

TODO
