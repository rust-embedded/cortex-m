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
    _init_array_start = ALIGN(4);
    KEEP(*(.init_array));
    _init_array_end = ALIGN(4);
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

ASSERT(__exceptions - ORIGIN(FLASH) == 0x40,
       "you must define the _EXCEPTIONS symbol");

ASSERT(__interrupts - __exceptions > 0,
       "you must define the _INTERRUPTS symbol");
