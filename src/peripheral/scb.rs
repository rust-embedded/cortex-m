//! System Control Block

use core::ptr;

use volatile_register::RW;

#[cfg(not(armv6m))]
use super::cpuid::CsselrCacheType;
#[cfg(not(armv6m))]
use super::CBP;
#[cfg(not(armv6m))]
use super::CPUID;
use super::SCB;

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// Interrupt Control and State
    pub icsr: RW<u32>,

    /// Vector Table Offset (not present on Cortex-M0 variants)
    pub vtor: RW<u32>,

    /// Application Interrupt and Reset Control
    pub aircr: RW<u32>,

    /// System Control
    pub scr: RW<u32>,

    /// Configuration and Control
    pub ccr: RW<u32>,

    /// System Handler Priority (word accessible only on Cortex-M0 variants)
    ///
    /// On ARMv7-M, `shpr[0]` points to SHPR1
    ///
    /// On ARMv6-M, `shpr[0]` points to SHPR2
    #[cfg(not(armv6m))]
    pub shpr: [RW<u8>; 12],
    #[cfg(armv6m)]
    _reserved1: u32,
    /// System Handler Priority (word accessible only on Cortex-M0 variants)
    ///
    /// On ARMv7-M, `shpr[0]` points to SHPR1
    ///
    /// On ARMv6-M, `shpr[0]` points to SHPR2
    #[cfg(armv6m)]
    pub shpr: [RW<u32>; 2],

    /// System Handler Control and State
    pub shcrs: RW<u32>,

    /// Configurable Fault Status (not present on Cortex-M0 variants)
    #[cfg(not(armv6m))]
    pub cfsr: RW<u32>,
    #[cfg(armv6m)]
    _reserved2: u32,

    /// HardFault Status (not present on Cortex-M0 variants)
    #[cfg(not(armv6m))]
    pub hfsr: RW<u32>,
    #[cfg(armv6m)]
    _reserved3: u32,

    /// Debug Fault Status (not present on Cortex-M0 variants)
    #[cfg(not(armv6m))]
    pub dfsr: RW<u32>,
    #[cfg(armv6m)]
    _reserved4: u32,

    /// MemManage Fault Address (not present on Cortex-M0 variants)
    #[cfg(not(armv6m))]
    pub mmfar: RW<u32>,
    #[cfg(armv6m)]
    _reserved5: u32,

    /// BusFault Address (not present on Cortex-M0 variants)
    #[cfg(not(armv6m))]
    pub bfar: RW<u32>,
    #[cfg(armv6m)]
    _reserved6: u32,

    /// Auxiliary Fault Status (not present on Cortex-M0 variants)
    #[cfg(not(armv6m))]
    pub afsr: RW<u32>,
    #[cfg(armv6m)]
    _reserved7: u32,

    _reserved8: [u32; 18],

    /// Coprocessor Access Control (not present on Cortex-M0 variants)
    #[cfg(not(armv6m))]
    pub cpacr: RW<u32>,
    #[cfg(armv6m)]
    _reserved9: u32,
}

/// FPU access mode
#[cfg(has_fpu)]
#[derive(Clone, Copy, Debug)]
pub enum FpuAccessMode {
    /// FPU is not accessible
    Disabled,
    /// FPU is accessible in Privileged and User mode
    Enabled,
    /// FPU is accessible in Privileged mode only
    Privileged,
}

#[cfg(has_fpu)]
mod fpu_consts {
    pub const SCB_CPACR_FPU_MASK: u32 = 0b11_11 << 20;
    pub const SCB_CPACR_FPU_ENABLE: u32 = 0b01_01 << 20;
    pub const SCB_CPACR_FPU_USER: u32 = 0b10_10 << 20;
}

#[cfg(has_fpu)]
use self::fpu_consts::*;

#[cfg(has_fpu)]
impl SCB {
    /// Shorthand for `set_fpu_access_mode(FpuAccessMode::Disabled)`
    pub fn disable_fpu(&mut self) {
        self.set_fpu_access_mode(FpuAccessMode::Disabled)
    }

    /// Shorthand for `set_fpu_access_mode(FpuAccessMode::Enabled)`
    pub fn enable_fpu(&mut self) {
        self.set_fpu_access_mode(FpuAccessMode::Enabled)
    }

    /// Gets FPU access mode
    pub fn fpu_access_mode() -> FpuAccessMode {
        // NOTE(unsafe) atomic read operation with no side effects
        let cpacr = unsafe { (*Self::ptr()).cpacr.read() };

        if cpacr & SCB_CPACR_FPU_MASK == SCB_CPACR_FPU_ENABLE | SCB_CPACR_FPU_USER {
            FpuAccessMode::Enabled
        } else if cpacr & SCB_CPACR_FPU_MASK == SCB_CPACR_FPU_ENABLE {
            FpuAccessMode::Privileged
        } else {
            FpuAccessMode::Disabled
        }
    }

    /// Sets FPU access mode
    ///
    /// *IMPORTANT* Any function that runs fully or partly with the FPU disabled must *not* take any
    /// floating-point arguments or have any floating-point local variables. Because the compiler
    /// might inline such a function into a caller that does have floating-point arguments or
    /// variables, any such function must be also marked #[inline(never)].
    pub fn set_fpu_access_mode(&mut self, mode: FpuAccessMode) {
        let mut cpacr = self.cpacr.read() & !SCB_CPACR_FPU_MASK;
        match mode {
            FpuAccessMode::Disabled => (),
            FpuAccessMode::Privileged => cpacr |= SCB_CPACR_FPU_ENABLE,
            FpuAccessMode::Enabled => cpacr |= SCB_CPACR_FPU_ENABLE | SCB_CPACR_FPU_USER,
        }
        unsafe { self.cpacr.write(cpacr) }
    }
}

impl SCB {
    /// Returns the active exception number
    pub fn vect_active() -> VectActive {
        let icsr = unsafe { ptr::read(&(*SCB::ptr()).icsr as *const _ as *const u32) };

        match icsr as u8 {
            0 => VectActive::ThreadMode,
            2 => VectActive::Exception(Exception::NonMaskableInt),
            3 => VectActive::Exception(Exception::HardFault),
            #[cfg(not(armv6m))]
            4 => VectActive::Exception(Exception::MemoryManagement),
            #[cfg(not(armv6m))]
            5 => VectActive::Exception(Exception::BusFault),
            #[cfg(not(armv6m))]
            6 => VectActive::Exception(Exception::UsageFault),
            #[cfg(any(armv8m, target_arch = "x86_64"))]
            7 => VectActive::Exception(Exception::SecureFault),
            11 => VectActive::Exception(Exception::SVCall),
            #[cfg(not(armv6m))]
            12 => VectActive::Exception(Exception::DebugMonitor),
            14 => VectActive::Exception(Exception::PendSV),
            15 => VectActive::Exception(Exception::SysTick),
            irqn => VectActive::Interrupt { irqn: irqn - 16 },
        }
    }
}

/// Processor core exceptions (internal interrupts)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Exception {
    /// Non maskable interrupt
    NonMaskableInt,

    /// Hard fault interrupt
    HardFault,

    /// Memory management interrupt (not present on Cortex-M0 variants)
    #[cfg(not(armv6m))]
    MemoryManagement,

    /// Bus fault interrupt (not present on Cortex-M0 variants)
    #[cfg(not(armv6m))]
    BusFault,

    /// Usage fault interrupt (not present on Cortex-M0 variants)
    #[cfg(not(armv6m))]
    UsageFault,

    /// Secure fault interrupt (only on ARMv8-M)
    #[cfg(any(armv8m, target_arch = "x86_64"))]
    SecureFault,

    /// SV call interrupt
    SVCall,

    /// Debug monitor interrupt (not present on Cortex-M0 variants)
    #[cfg(not(armv6m))]
    DebugMonitor,

    /// Pend SV interrupt
    PendSV,

    /// System Tick interrupt
    SysTick,
}

impl Exception {
    /// Returns the IRQ number of this `Exception`
    ///
    /// The return value is always within the closed range `[-1, -14]`
    pub fn irqn(&self) -> i8 {
        match *self {
            Exception::NonMaskableInt => -14,
            Exception::HardFault => -13,
            #[cfg(not(armv6m))]
            Exception::MemoryManagement => -12,
            #[cfg(not(armv6m))]
            Exception::BusFault => -11,
            #[cfg(not(armv6m))]
            Exception::UsageFault => -10,
            #[cfg(any(armv8m, target_arch = "x86_64"))]
            Exception::SecureFault => -9,
            Exception::SVCall => -5,
            #[cfg(not(armv6m))]
            Exception::DebugMonitor => -4,
            Exception::PendSV => -2,
            Exception::SysTick => -1,
        }
    }
}

/// Active exception number
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VectActive {
    /// Thread mode
    ThreadMode,

    /// Processor core exception (internal interrupts)
    Exception(Exception),

    /// Device specific exception (external interrupts)
    Interrupt {
        /// Interrupt number. This number is always within half open range `[0, 240)`
        irqn: u8,
    },
}

impl VectActive {
    /// Converts a `byte` into `VectActive`
    pub fn from(vect_active: u8) -> Option<Self> {
        Some(match vect_active {
            0 => VectActive::ThreadMode,
            2 => VectActive::Exception(Exception::NonMaskableInt),
            3 => VectActive::Exception(Exception::HardFault),
            #[cfg(not(armv6m))]
            4 => VectActive::Exception(Exception::MemoryManagement),
            #[cfg(not(armv6m))]
            5 => VectActive::Exception(Exception::BusFault),
            #[cfg(not(armv6m))]
            6 => VectActive::Exception(Exception::UsageFault),
            #[cfg(any(armv8m, target_arch = "x86_64"))]
            7 => VectActive::Exception(Exception::SecureFault),
            11 => VectActive::Exception(Exception::SVCall),
            #[cfg(not(armv6m))]
            12 => VectActive::Exception(Exception::DebugMonitor),
            14 => VectActive::Exception(Exception::PendSV),
            15 => VectActive::Exception(Exception::SysTick),
            irqn if irqn >= 16 => VectActive::Interrupt { irqn },
            _ => return None,
        })
    }
}

#[cfg(not(armv6m))]
mod scb_consts {
    pub const SCB_CCR_IC_MASK: u32 = (1 << 17);
    pub const SCB_CCR_DC_MASK: u32 = (1 << 16);
}

#[cfg(not(armv6m))]
use self::scb_consts::*;

#[cfg(not(armv6m))]
impl SCB {
    /// Enables I-Cache if currently disabled
    #[inline]
    pub fn enable_icache(&mut self) {
        // Don't do anything if ICache is already enabled
        if Self::icache_enabled() {
            return;
        }

        // NOTE(unsafe) All CBP registers are write-only and stateless
        let mut cbp = unsafe { CBP::new() };

        // Invalidate I-Cache
        cbp.iciallu();

        // Enable I-Cache
        unsafe { self.ccr.modify(|r| r | SCB_CCR_IC_MASK) };

        ::asm::dsb();
        ::asm::isb();
    }

    /// Disables I-Cache if currently enabled
    #[inline]
    pub fn disable_icache(&mut self) {
        // Don't do anything if ICache is already disabled
        if !Self::icache_enabled() {
            return;
        }

        // NOTE(unsafe) All CBP registers are write-only and stateless
        let mut cbp = unsafe { CBP::new() };

        // Disable I-Cache
        unsafe { self.ccr.modify(|r| r & !SCB_CCR_IC_MASK) };

        // Invalidate I-Cache
        cbp.iciallu();

        ::asm::dsb();
        ::asm::isb();
    }

    /// Returns whether the I-Cache is currently enabled
    #[inline]
    pub fn icache_enabled() -> bool {
        ::asm::dsb();
        ::asm::isb();

        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*Self::ptr()).ccr.read() & SCB_CCR_IC_MASK == SCB_CCR_IC_MASK }
    }

    /// Invalidates I-Cache
    #[inline]
    pub fn invalidate_icache(&mut self) {
        // NOTE(unsafe) All CBP registers are write-only and stateless
        let mut cbp = unsafe { CBP::new() };

        // Invalidate I-Cache
        cbp.iciallu();

        ::asm::dsb();
        ::asm::isb();
    }

    /// Enables D-cache if currently disabled
    #[inline]
    pub fn enable_dcache(&mut self, cpuid: &mut CPUID) {
        // Don't do anything if DCache is already enabled
        if Self::dcache_enabled() {
            return;
        }

        // Invalidate anything currently in the DCache
        self.invalidate_dcache(cpuid);

        // Now turn on the DCache
        unsafe { self.ccr.modify(|r| r | SCB_CCR_DC_MASK) };

        ::asm::dsb();
        ::asm::isb();
    }

    /// Disables D-cache if currently enabled
    #[inline]
    pub fn disable_dcache(&mut self, cpuid: &mut CPUID) {
        // Don't do anything if DCache is already disabled
        if !Self::dcache_enabled() {
            return;
        }

        // Turn off the DCache
        unsafe { self.ccr.modify(|r| r & !SCB_CCR_DC_MASK) };

        // Clean and invalidate whatever was left in it
        self.clean_invalidate_dcache(cpuid);
    }

    /// Returns whether the D-Cache is currently enabled
    #[inline]
    pub fn dcache_enabled() -> bool {
        ::asm::dsb();
        ::asm::isb();

        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*Self::ptr()).ccr.read() & SCB_CCR_DC_MASK == SCB_CCR_DC_MASK }
    }

    /// Invalidates D-cache
    ///
    /// Note that calling this while the dcache is enabled will probably wipe out your
    /// stack, depending on optimisations, breaking returning to the call point.
    /// It's used immediately before enabling the dcache, but not exported publicly.
    #[inline]
    fn invalidate_dcache(&mut self, cpuid: &mut CPUID) {
        // NOTE(unsafe) All CBP registers are write-only and stateless
        let mut cbp = unsafe { CBP::new() };

        // Read number of sets and ways
        let (sets, ways) = cpuid.cache_num_sets_ways(0, CsselrCacheType::DataOrUnified);

        // Invalidate entire D-Cache
        for set in 0..sets {
            for way in 0..ways {
                cbp.dcisw(set, way);
            }
        }

        ::asm::dsb();
        ::asm::isb();
    }

    /// Cleans D-cache
    #[inline]
    pub fn clean_dcache(&mut self, cpuid: &mut CPUID) {
        // NOTE(unsafe) All CBP registers are write-only and stateless
        let mut cbp = unsafe { CBP::new() };

        // Read number of sets and ways
        let (sets, ways) = cpuid.cache_num_sets_ways(0, CsselrCacheType::DataOrUnified);

        for set in 0..sets {
            for way in 0..ways {
                cbp.dccsw(set, way);
            }
        }

        ::asm::dsb();
        ::asm::isb();
    }

    /// Cleans and invalidates D-cache
    #[inline]
    pub fn clean_invalidate_dcache(&mut self, cpuid: &mut CPUID) {
        // NOTE(unsafe) All CBP registers are write-only and stateless
        let mut cbp = unsafe { CBP::new() };

        // Read number of sets and ways
        let (sets, ways) = cpuid.cache_num_sets_ways(0, CsselrCacheType::DataOrUnified);

        for set in 0..sets {
            for way in 0..ways {
                cbp.dccisw(set, way);
            }
        }

        ::asm::dsb();
        ::asm::isb();
    }

    /// Invalidates D-cache by address
    ///
    /// `addr`: the address to invalidate
    /// `size`: size of the memory block, in number of bytes
    ///
    /// Invalidates cache starting from the lowest 32-byte aligned address represented by `addr`,
    /// in blocks of 32 bytes until at least `size` bytes have been invalidated.
    #[inline]
    pub fn invalidate_dcache_by_address(&mut self, addr: usize, size: usize) {
        // No-op zero sized operations
        if size == 0 {
            return;
        }

        // NOTE(unsafe) All CBP registers are write-only and stateless
        let mut cbp = unsafe { CBP::new() };

        ::asm::dsb();

        // Cache lines are fixed to 32 bit on Cortex-M7 and not present in earlier Cortex-M
        const LINESIZE: usize = 32;
        let num_lines = ((size - 1) / LINESIZE) + 1;

        let mut addr = addr & 0xFFFF_FFE0;

        for _ in 0..num_lines {
            cbp.dcimvac(addr as u32);
            addr += LINESIZE;
        }

        ::asm::dsb();
        ::asm::isb();
    }

    /// Cleans D-cache by address
    ///
    /// `addr`: the address to clean
    /// `size`: size of the memory block, in number of bytes
    ///
    /// Cleans cache starting from the lowest 32-byte aligned address represented by `addr`,
    /// in blocks of 32 bytes until at least `size` bytes have been cleaned.
    #[inline]
    pub fn clean_dcache_by_address(&mut self, addr: usize, size: usize) {
        // No-op zero sized operations
        if size == 0 {
            return;
        }

        // NOTE(unsafe) All CBP registers are write-only and stateless
        let mut cbp = unsafe { CBP::new() };

        ::asm::dsb();

        // Cache lines are fixed to 32 bit on Cortex-M7 and not present in earlier Cortex-M
        const LINESIZE: usize = 32;
        let num_lines = ((size - 1) / LINESIZE) + 1;

        let mut addr = addr & 0xFFFF_FFE0;

        for _ in 0..num_lines {
            cbp.dccmvac(addr as u32);
            addr += LINESIZE;
        }

        ::asm::dsb();
        ::asm::isb();
    }

    /// Cleans and invalidates D-cache by address
    ///
    /// `addr`: the address to clean and invalidate
    /// `size`: size of the memory block, in number of bytes
    ///
    /// Cleans and invalidates cache starting from the lowest 32-byte aligned address represented
    /// by `addr`, in blocks of 32 bytes until at least `size` bytes have been cleaned and
    /// invalidated.
    #[inline]
    pub fn clean_invalidate_dcache_by_address(&mut self, addr: usize, size: usize) {
        // No-op zero sized operations
        if size == 0 {
            return;
        }

        // NOTE(unsafe) All CBP registers are write-only and stateless
        let mut cbp = unsafe { CBP::new() };

        ::asm::dsb();

        // Cache lines are fixed to 32 bit on Cortex-M7 and not present in earlier Cortex-M
        const LINESIZE: usize = 32;
        let num_lines = ((size - 1) / LINESIZE) + 1;

        let mut addr = addr & 0xFFFF_FFE0;

        for _ in 0..num_lines {
            cbp.dccimvac(addr as u32);
            addr += LINESIZE;
        }

        ::asm::dsb();
        ::asm::isb();
    }
}

const SCB_SCR_SLEEPDEEP: u32 = 0x1 << 2;

impl SCB {
    /// Set the SLEEPDEEP bit in the SCR register
    pub fn set_sleepdeep(&mut self) {
        unsafe {
            self.scr.modify(|scr| scr | SCB_SCR_SLEEPDEEP);
        }
    }

    /// Clear the SLEEPDEEP bit in the SCR register
    pub fn clear_sleepdeep(&mut self) {
        unsafe {
            self.scr.modify(|scr| scr & !SCB_SCR_SLEEPDEEP);
        }
    }
}

const SCB_AIRCR_VECTKEY: u32 = 0x05FA << 16;
const SCB_AIRCR_PRIGROUP_MASK: u32 = 0x5 << 8;
const SCB_AIRCR_SYSRESETREQ: u32 = 1 << 2;

impl SCB {
    /// Initiate a system reset request to reset the MCU
    pub fn system_reset(&mut self) -> ! {
        ::asm::dsb();
        unsafe {
            self.aircr.modify(
                |r| {
                    SCB_AIRCR_VECTKEY | // otherwise the write is ignored
            r & SCB_AIRCR_PRIGROUP_MASK | // keep priority group unchanged
            SCB_AIRCR_SYSRESETREQ
                }, // set the bit
            )
        };
        ::asm::dsb();
        loop {
            // wait for the reset
            ::asm::nop(); // avoid rust-lang/rust#28728
        }
    }
}

const SCB_ICSR_PENDSVSET: u32 = 1 << 28;
const SCB_ICSR_PENDSVCLR: u32 = 1 << 27;

const SCB_ICSR_PENDSTSET: u32 = 1 << 26;
const SCB_ICSR_PENDSTCLR: u32 = 1 << 25;

impl SCB {
    /// Set the PENDSVSET bit in the ICSR register which will pend the PendSV interrupt
    pub fn set_pendsv() {
        unsafe {
            (*Self::ptr()).icsr.write(SCB_ICSR_PENDSVSET);
        }
    }

    /// Check if PENDSVSET bit in the ICSR register is set meaning PendSV interrupt is pending
    pub fn is_pendsv_pending() -> bool {
        unsafe { (*Self::ptr()).icsr.read() & SCB_ICSR_PENDSVSET == SCB_ICSR_PENDSVSET }
    }

    /// Set the PENDSVCLR bit in the ICSR register which will clear a pending PendSV interrupt
    pub fn clear_pendsv() {
        unsafe {
            (*Self::ptr()).icsr.write(SCB_ICSR_PENDSVCLR);
        }
    }

    /// Set the PENDSTCLR bit in the ICSR register which will clear a pending SysTick interrupt
    #[inline]
    pub fn set_pendst() {
        unsafe {
            (*Self::ptr()).icsr.write(SCB_ICSR_PENDSTSET);
        }
    }

    /// Check if PENDSTSET bit in the ICSR register is set meaning SysTick interrupt is pending
    #[inline]
    pub fn is_pendst_pending() -> bool {
        unsafe { (*Self::ptr()).icsr.read() & SCB_ICSR_PENDSTSET == SCB_ICSR_PENDSTSET }
    }

    /// Set the PENDSTCLR bit in the ICSR register which will clear a pending SysTick interrupt
    #[inline]
    pub fn clear_pendst() {
        unsafe {
            (*Self::ptr()).icsr.write(SCB_ICSR_PENDSTCLR);
        }
    }
}

/// System handlers, exceptions with configurable priority
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SystemHandler {
    // NonMaskableInt, // priority is fixed
    // HardFault, // priority is fixed
    /// Memory management interrupt (not present on Cortex-M0 variants)
    #[cfg(not(armv6m))]
    MemoryManagement,

    /// Bus fault interrupt (not present on Cortex-M0 variants)
    #[cfg(not(armv6m))]
    BusFault,

    /// Usage fault interrupt (not present on Cortex-M0 variants)
    #[cfg(not(armv6m))]
    UsageFault,

    /// Secure fault interrupt (only on ARMv8-M)
    #[cfg(any(armv8m, target_arch = "x86_64"))]
    SecureFault,

    /// SV call interrupt
    SVCall,

    // #[cfg(not(armv6m))]
    // DebugMonitor, // unclear whether this has configurable priority
    /// Pend SV interrupt
    PendSV,

    /// System Tick interrupt
    SysTick,

    // Make this enum extensible
    #[doc(hidden)]
    __DO_NOT_MATCH_AGAINST_THIS_VARIANT__,
}

impl SystemHandler {
    fn index(&self) -> u8 {
        match *self {
            #[cfg(not(armv6m))]
            SystemHandler::MemoryManagement => 4,
            #[cfg(not(armv6m))]
            SystemHandler::BusFault => 5,
            #[cfg(not(armv6m))]
            SystemHandler::UsageFault => 6,
            #[cfg(any(armv8m, target_arch = "x86_64"))]
            SystemHandler::SecureFault => 7,
            SystemHandler::SVCall => 11,
            SystemHandler::PendSV => 14,
            SystemHandler::SysTick => 15,
            SystemHandler::__DO_NOT_MATCH_AGAINST_THIS_VARIANT__ => unreachable!(),
        }
    }
}

impl SCB {
    /// Returns the hardware priority of `system_handler`
    ///
    /// *NOTE*: Hardware priority does not exactly match logical priority levels. See
    /// [`NVIC.get_priority`](struct.NVIC.html#method.get_priority) for more details.
    pub fn get_priority(system_handler: SystemHandler) -> u8 {
        let index = system_handler.index();

        #[cfg(not(armv6m))]
        {
            // NOTE(unsafe) atomic read with no side effects
            unsafe { (*Self::ptr()).shpr[usize::from(index - 4)].read() }
        }

        #[cfg(armv6m)]
        {
            // NOTE(unsafe) atomic read with no side effects
            let shpr = unsafe { (*Self::ptr()).shpr[usize::from((index - 8) / 4)].read() };
            let prio = (shpr >> (index % 4)) & 0x000000ff;
            prio as u8
        }
    }

    /// Sets the hardware priority of `system_handler` to `prio`
    ///
    /// *NOTE*: Hardware priority does not exactly match logical priority levels. See
    /// [`NVIC.get_priority`](struct.NVIC.html#method.get_priority) for more details.
    ///
    /// On ARMv6-M, updating a system handler priority requires a read-modify-write operation. On
    /// ARMv7-M, the operation is performed in a single, atomic write operation.
    ///
    /// # Unsafety
    ///
    /// Changing priority levels can break priority-based critical sections (see
    /// [`register::basepri`](../register/basepri/index.html)) and compromise memory safety.
    pub unsafe fn set_priority(&mut self, system_handler: SystemHandler, prio: u8) {
        let index = system_handler.index();

        #[cfg(not(armv6m))]
        {
            self.shpr[usize::from(index - 4)].write(prio)
        }

        #[cfg(armv6m)]
        {
            self.shpr[usize::from((index - 8) / 4)].modify(|value| {
                let shift = index % 4;
                let mask = 0x000000ff << shift;
                let prio = u32::from(prio) << shift;

                (value & !mask) | prio
            });
        }
    }
}
