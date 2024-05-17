//! Coprocessor access assembly instructions.



/// Internal function to create an inlined MCR instruction.
/// This instruction moves one Register to a Coprocessor Register.
/// For this function to compile to a single instruction, compile to opt-level > 2.
#[inline(always)]
pub fn mcr<const CP: u32, const OP1: u32, const CRN: u32, const CRM: u32, const OP2: u32>(value: u32) {
    unsafe {
        core::arch::asm!(
            "MCR p{cp}, #{op1}, {0}, c{crn}, c{crm}, #{op2}",
            in(reg) value,
            cp  = const CP,
            op1 = const OP1,
            crn = const CRN,
            crm = const CRM,
            op2 = const OP2,
            options(nostack, nomem)
        )
    }
}



/// Internal function to create an inlined MRC instruction.
/// This instruction moves one Coprocessor Register to a Register.
/// For this function to compile to a single instruction, compile to opt-level > 2.
#[inline(always)]
pub fn mrc<const CP: u32, const OP1: u32, const CRN: u32, const CRM: u32, const OP2: u32>() -> u32 {
    // Preallocate the value.
    let a: u32;

    unsafe {
        core::arch::asm!(
            "MRC p{cp}, #{op1}, {0}, c{crn}, c{crm}, #{op2}",
            out(reg) a,
            cp  = const CP,
            op1 = const OP1,
            crn = const CRN,
            crm = const CRM,
            op2 = const OP2,
            options(nostack, nomem)
        )
    }

    a
}



/// Internal function to create an inlined MCRR instruction.
/// This instruction moves two Registers to Coprocessor Registers.
/// For this function to compile to a single instruction, compile to opt-level > 2.
#[inline(always)]
pub fn mcrr<const CP: u32, const OP1: u32, const CRM: u32>(a: u32, b: u32) {
    unsafe {
        core::arch::asm!(
            "MCRR p{cp}, #{op1}, {0}, {1}, c{crm}",
            in(reg) a,
            in(reg) b,
            cp  = const CP,
            op1 = const OP1,
            crm = const CRM,
            options(nostack, nomem)
        )
    }
}



/// Internal function to create an inlined MRRC instruction.
/// This instruction moves two Coprocessor Registers to Registers.
/// For this function to compile to a single instruction, compile to opt-level > 2.
#[inline(always)]
pub fn mrrc<const CP: u32, const OPC: u32, const CRM: u32>() -> (u32, u32) {
    // Preallocate the values.
    let a: u32;
    let b: u32;

    unsafe {
        core::arch::asm!(
            "MRRC p{cp}, #{opc}, {0}, {1}, c{crm}",
            out(reg) a,
            out(reg) b,
            cp  = const CP,
            opc = const OPC,
            crm = const CRM,
            options(nostack, nomem)
        )
    }

    (a, b)
}
