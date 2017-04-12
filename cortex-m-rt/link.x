INCLUDE memory.x

SECTIONS
{
  .text ORIGIN(FLASH) :
  {
    /* Vector table */
    _VECTOR_TABLE = .;
    LONG(ORIGIN(RAM) + LENGTH(RAM));

    KEEP(*(.rodata.reset_handler));
    KEEP(*(.rodata.exceptions));
    __exceptions = .;

    KEEP(*(.rodata.interrupts));
    __interrupts = .;

    *(.text.*);
    *(.rodata.*);
  } > FLASH

  .bss : ALIGN(4)
  {
    _sbss = .;
    *(.bss.*);
    _ebss = ALIGN(4);
  } > RAM

  .data : ALIGN(4)
  {
    _sdata = .;
    *(.data.*);
    _edata = ALIGN(4);
  } > RAM AT > FLASH

  _sidata = LOADADDR(.data);

  /DISCARD/ :
  {
    /* Unused unwinding stuff */
    *(.ARM.exidx.*)
    *(.ARM.extab.*)
  }
}

ASSERT(__exceptions - ORIGIN(FLASH) == 0x40, "
You must specify the exception handlers.
Create a non `pub` static variable with type
`cortex_m::exception::Handlers` and place it in the
'.rodata.exceptions' section. (cf. #[link_section])
Apply the `#[used]` attribute to the variable to help it reach the linker.");

ASSERT(__interrupts - __exceptions > 0, "
You must specify the interrupt handlers.
Create a non `pub` static variable and place it in the
'.rodata.interrupts' section. (cf. #[link_section])
Apply the `#[used]` attribute to the variable to help it reach the linker.");

ASSERT(__interrupts - __exceptions <= 0x3c0, "
There can't be more than 240 interrupt handlers.
Fix the '.rodata.interrupts' section. (cf. #[link_section])");
