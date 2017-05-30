INCLUDE memory.x

SECTIONS
{
  .vector_table ORIGIN(FLASH) :
  {
    /* Vector table */
    _svector_table = .;
    LONG(_stack_start);

    KEEP(*(.vector_table.reset_handler));

    KEEP(*(.rodata.exceptions));
    _eexceptions = .;

    KEEP(*(.rodata.interrupts));
    _einterrupts = .;
  } > FLASH

  PROVIDE(_stext = _einterrupts);
  
  .text _stext : ALIGN(4)
  {
    /* Put reset handler first in .text section so it ends up as the entry */
    /* point of the program. */
    KEEP(*(.reset_handler));

    *(.text .text.*);
  } > FLASH

  .rodata : ALIGN(4)
  {
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

  /* The heap starts right after the .bss + .data section ends */
  _sheap = _edata;

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
ASSERT(_eexceptions - ORIGIN(FLASH) > 8, "
You must specify the exception handlers.
Create a non `pub` static variable with type
`cortex_m::exception::Handlers` and place it in the
'.rodata.exceptions' section. (cf. #[link_section]). Apply the
`#[used]` attribute to the variable to make it reach the linker.");

ASSERT(_eexceptions - ORIGIN(FLASH) == 0x40, "
Invalid '.rodata.exceptions' section.
Make sure to place a static with type `cortex_m::exception::Handlers`
in that section (cf. #[link_section]) ONLY ONCE.");

ASSERT(_einterrupts - _eexceptions > 0, "
You must specify the interrupt handlers.
Create a non `pub` static variable and place it in the
'.rodata.interrupts' section. (cf. #[link_section]). Apply the
`#[used]` attribute to the variable to help it reach the linker.");

ASSERT(_einterrupts - _eexceptions <= 0x3c0, "
There can't be more than 240 interrupt handlers.
Fix the '.rodata.interrupts' section. (cf. #[link_section])");

ASSERT(_einterrupts <= _stext, "
The '.text' section can't be placed inside '.vector_table' section.
Set '_stext' to an adress greater than '_einterrupts'");

ASSERT(_stext < ORIGIN(FLASH) + LENGTH(FLASH), "
The '.text' section must be placed inside the FLASH memory
Set '_stext' to an address smaller than 'ORIGIN(FLASH) + LENGTH(FLASH)");
