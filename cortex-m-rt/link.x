INCLUDE memory.x

SECTIONS
{
  .text ORIGIN(FLASH) :
  {
    /* Vector table */
    _VECTOR_TABLE = .;
    LONG(_stack_start);

    KEEP(*(.rodata.reset_handler));
    KEEP(*(.rodata.exceptions));
    __exceptions = .;

    KEEP(*(.rodata.interrupts));
    __interrupts = .;

    *(.text .text.*);
    *(.rodata .rodata.*);
  } > FLASH

  .bss : ALIGN(4)
  {
    _sbss = .;
    *(.bss .bss.*);
    _ebss = ALIGN(4);
  } > RAM

  .data : ALIGN(4)
  {
    _sdata = .;
    *(.data .data.*);
    _edata = ALIGN(4);
  } > RAM AT > FLASH

  _sidata = LOADADDR(.data);

  /* Due to an unfortunate combination of legacy concerns,
     toolchain drawbacks, and insufficient attention to detail,
     rustc has no choice but to mark .debug_gdb_scripts as allocatable.
     We really do not want to upload it to our target, so we
     remove the allocatable bit. Unfortunately, it appears
     that the only way to do this in a linker script is
     the extremely obscure "INFO" output section type specifier. */
  .debug_gdb_scripts 0 (INFO) : {
    KEEP(*(.debug_gdb_scripts))
  }

  /DISCARD/ :
  {
    /* Unused unwinding stuff */
    *(.ARM.exidx.*)
    *(.ARM.extab.*)
  }
}

/* Do not exceed this mark in the error messages below                | */
ASSERT(__exceptions - ORIGIN(FLASH) > 8, "
You must specify the exception handlers.
Create a non `pub` static variable with type
`cortex_m::exception::Handlers` and place it in the
'.rodata.exceptions' section. (cf. #[link_section]). Apply the
`#[used]` attribute to the variable to make it reach the linker.");

ASSERT(__exceptions - ORIGIN(FLASH) == 0x40, "
Invalid '.rodata.exceptions' section.
Make sure to place a static with type `cortex_m::exception::Handlers`
in that section (cf. #[link_section]) ONLY ONCE.");

ASSERT(__interrupts - __exceptions > 0, "
You must specify the interrupt handlers.
Create a non `pub` static variable and place it in the
'.rodata.interrupts' section. (cf. #[link_section]). Apply the
`#[used]` attribute to the variable to help it reach the linker.");

ASSERT(__interrupts - __exceptions <= 0x3c0, "
There can't be more than 240 interrupt handlers.
Fix the '.rodata.interrupts' section. (cf. #[link_section])");
