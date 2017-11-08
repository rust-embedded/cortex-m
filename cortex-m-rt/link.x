INCLUDE memory.x

/* With multiple codegen units the rlib produced for this crate has several object files in it. */
/* Because the linker is Smart it may not look into all the object files and not pick up the */
/* .vector_table.exceptions section. But we want it to! To workaround the problem we create an */
/* undefined reference to the EXCEPTIONS symbol (located in .vector_table.exceptions); this way the */
/* linker will look at all the object of the rlib and pick up our EXCEPTIONS symbol */
EXTERN(EXCEPTIONS);

/* Create an undefined reference to the INTERRUPTS symbol. This is required to
   force the linker to *not* drop the INTERRUPTS symbol if it comes from an
   object file that's passed to the linker *before* this crate */
EXTERN(INTERRUPTS);

PROVIDE(_stack_start = ORIGIN(RAM) + LENGTH(RAM));

SECTIONS
{
  .vector_table ORIGIN(FLASH) : ALIGN(4)
  {
    /* Vector table */
    _svector_table = .;
    LONG(_stack_start);

    KEEP(*(.vector_table.reset_vector));

    KEEP(*(.vector_table.exceptions));
    _eexceptions = .;

    KEEP(*(.vector_table.interrupts));
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
    . = ALIGN(4);
  } > FLASH

  /* limits of the .stack region */
  _sstack = ORIGIN(RAM);
  _estack = _stack_start;

  /* ficticious region that represents the memory available for the stack */
  .stack _sstack (INFO) : ALIGN(4)
  {
    . += (_estack - _sstack);
  }

  PROVIDE(_sbss = ORIGIN(RAM));
  .bss _sbss : ALIGN(4)
  {
    *(.bss .bss.*);
    . = ALIGN(4);
    _ebss = .;
  } > RAM

  .data : ALIGN(4)
  {
    _sidata = LOADADDR(.data);
    _sdata = .;
    *(.data .data.*);
    . = ALIGN(4);
    _edata = .;
  } > RAM AT > FLASH

  /* fake output .got section */
  /* Dynamic relocations are unsupported. This section is only used to detect
     relocatable code in the input files and raise an error if relocatable code
     is found */
  .got :
  {
    _sgot = .;
    KEEP(*(.got .got.*));
    _egot = .;
  } > RAM AT > FLASH

  /* The heap starts right after the .bss + .data section ends */
  _sheap = _edata;

  /* Due to an unfortunate combination of legacy concerns,
     toolchain drawbacks, and insufficient attention to detail,
     rustc has no choice but to mark .debug_gdb_scripts as allocatable.
     We really do not want to upload it to our target, so we
     remove the allocatable bit. Unfortunately, it appears
     that the only way to do this in a linker script is
     the extremely obscure "INFO" output section type specifier. */
  /* a rustc hack will force the program to read the first byte of this section,
     so we'll set the (fake) start address of this section to something we're
     sure can be read at runtime: the start of the .text section */
  .debug_gdb_scripts _stext (INFO) : {
    KEEP(*(.debug_gdb_scripts))
  }

  /DISCARD/ :
  {
    *(.ARM.exidx.*)
  }
}

/* Do not exceed this mark in the error messages below                | */
ASSERT(_eexceptions - ORIGIN(FLASH) > 8, "
The exception handlers are missing. This is likely a cortex-m-rt bug.
Please file a bug report at:
https://github.com/japaric/cortex-m-rt/issues");

ASSERT(_eexceptions - ORIGIN(FLASH) == 0x40, "
Invalid '.vector_table.exceptions' section. This is likely a
cortex-m-rt bug. Please file a bug report at:
https://github.com/japaric/cortex-m-rt/issues");

ASSERT(_einterrupts - _eexceptions > 0, "
The interrupt handlers are missing. If you are not linking to a device
crate then you supply the interrupt handlers yourself. Check the
documentation.");

ASSERT(_einterrupts - _eexceptions <= 0x3c0, "
There can't be more than 240 interrupt handlers. This may be a bug in
your device crate, or you may have registered more than 240 interrupt
handlers.");

ASSERT(_einterrupts <= _stext, "
The '.text' section can't be placed inside '.vector_table' section.
Set '_stext' to an address greater than '_einterrupts'");

ASSERT(_stext < ORIGIN(FLASH) + LENGTH(FLASH), "
The '.text' section must be placed inside the FLASH memory
Set '_stext' to an address smaller than 'ORIGIN(FLASH) + LENGTH(FLASH)");

ASSERT(_sgot == _egot, "
.got section detected in the input files. Dynamic relocations are not
supported. If you are linking to C code compiled using the `gcc` crate
then modify your build script to compile the C code _without_ the
-fPIC flag. See the documentation of the `gcc::Config.fpic` method for
details.");
