/* # Developer notes

- Symbols that start with a double underscore (__) are considered "private"
- Symbols that start with a single underscore (_) are considered "semi-public"; they can be
  overridden in a user linker script, but should not be referred from user code (e.g. `extern "C" {
  static mut __sbss }`).
- `EXTERN` forces the linker to keep a symbol in the final binary. We use this to make sure a
  symbol if not dropped if it appears in or near the front of the linker arguments and "it's not
  needed" by any of the preceding objects (linker arguments)
- `PROVIDE` is used to provide default values that can be overridden by a user linker script
- On alignment: it's important for correctness that the VMA boundaries of both .bss and .data *and*
  the LMA of .data are all 4-byte aligned. These alignments are assumed by the RAM initialization
  routine. There's also a second benefit: 4-byte aligned boundaries means that you won't see
  "Address (..) is out of bounds" in the disassembly produced by `objdump`.
*/

INCLUDE memory.x
INCLUDE interrupts.x

/* # Entry point */
ENTRY(__reset);
EXTERN(__RESET_VECTOR); /* depends on the `__reset` symbol */

EXTERN(__EXCEPTIONS); /* depends on all the these PROVIDED symbols */

/* # Exception vectors */
/* This is effectively weak aliasing at the linker level */
/* The user can override any of these aliases by defining the corresponding symbol themselves (cf.
   the `exception!` macro) */
EXTERN(DefaultHandler);
PROVIDE(NMI = DefaultHandler);
PROVIDE(HardFault = DefaultHandler);
PROVIDE(MemManage = DefaultHandler);
PROVIDE(BusFault = DefaultHandler);
PROVIDE(UsageFault = DefaultHandler);
PROVIDE(SVC = DefaultHandler);
PROVIDE(DebugMon = DefaultHandler);
PROVIDE(PendSV = DefaultHandler);
PROVIDE(SysTick = DefaultHandler);

/* # Interrupt vectors */
EXTERN(__INTERRUPTS); /* to be provided by the device crate or equivalent */

/* # User overridable symbols I */
/* Lets the user place the stack in a different RAM region */
PROVIDE(_stack_start = ORIGIN(RAM) + LENGTH(RAM));

/* # Sections */
SECTIONS
{
  /* ## Sections in FLASH */
  /* ### Vector table */
  .vector_table ORIGIN(FLASH) : ALIGN(4)
  {
    /* Initial Stack Pointer (SP) value */
    __STACK_START = .; /* Just to get a nicer name in the disassembly */
    LONG(_stack_start);

    /* Reset vector */
    KEEP(*(.vector_table.reset_vector));
    __reset_vector = ABSOLUTE(.);

    /* Exceptions */
    KEEP(*(.vector_table.exceptions));
    __eexceptions = ABSOLUTE(.);

    /* Device specific interrupts */
    KEEP(*(.vector_table.interrupts));
    __einterrupts = ABSOLUTE(.);
  } > FLASH

  /* ### .text */
  .text _stext :
  {
    *(.text .text.*);
    __etext = ABSOLUTE(.);
  } > FLASH

  /* ### .rodata */
  .rodata :
  {
    . = ALIGN(4); /* 4-byte align the end (VMA) of this section */
    /* __srodata = ABSOLUTE(.); */

    *(.rodata .rodata.*);

    . = ALIGN(4); /* 4-byte align the end (VMA) of this section */
    __erodata = ABSOLUTE(.);
  } > FLASH

  /* ## Sections in RAM */
  /* ### .data */
  .data : AT(__erodata) /* LMA */
  {
    . = ALIGN(4); /* 4-byte align the start (VMA) of this section */
    __sdata = ABSOLUTE(.);

    *(.data .data.*);

    . = ALIGN(4); /* 4-byte align the end (VMA) of this section */
    __edata = ABSOLUTE(.);
  } > RAM

  /* ### .bss */
  .bss :
  {
    . = ALIGN(4); /* 4-byte align the start (VMA) of this section */
    __sbss = ABSOLUTE(.);

    *(.bss .bss.*);

    . = ALIGN(4); /* 4-byte align the end (VMA) of this section */
    __ebss = ABSOLUTE(.);
  } > RAM

  /* ## Fake output .got section */
  /* Dynamic relocations are unsupported. This section is only used to detect relocatable code in
     the input files and raise an error if relocatable code is found */
  .got :
  {
    __sgot = ABSOLUTE(.);
    KEEP(*(.got .got.*));
    __egot = ABSOLUTE(.);
  } > FLASH

  /* ## Discarded sections */
  /DISCARD/ :
  {
    /* Unused exception related info that only wastes space */
    *(.ARM.exidx.*);
  }
}

/* # User overridable symbols II */
/* (The user overridable symbols are split in two parts because LLD demands that the RHS of PROVIDE
    to be defined before the PROVIDE invocation) */
/* Lets the user override this to place .text a bit further than the vector table. Required by
microcontrollers that store their configuration right after the vector table. */
PROVIDE(_stext = __einterrupts);

/* # Hardcoded symbols */
/* Place `.bss` at the start of the RAM region */
__sidata = LOADADDR(.data);
/* Place the heap right after `.bss` and `.data` */
__sheap = __edata;

/* # Sanity checks */

/* Do not exceed this mark in the error messages below                | */
ASSERT(__reset_vector == ORIGIN(FLASH) + 0x8,
"Reset vector is missing. This is a cortex-m-rt bug.
Please file a bug report at:
https://github.com/japaric/cortex-m-rt/issues");

ASSERT(__eexceptions - ORIGIN(FLASH) == 0x40, "
The exception handlers are missing. This is a cortex-m-rt bug.
Please file a bug report at:
https://github.com/japaric/cortex-m-rt/issues");

ASSERT(__einterrupts - __eexceptions > 0, "
The interrupt handlers are missing. If you are not linking to a device
crate then you supply the interrupt handlers yourself. Check the
documentation.");

ASSERT(__einterrupts - __eexceptions <= 0x3c0, "
There can't be more than 240 interrupt handlers. This may be a bug in
your device crate, or you may have registered more than 240 interrupt
handlers.");

ASSERT(__einterrupts <= _stext, "
The '.text' section can't be placed inside '.vector_table' section.
Set '_stext' to an address greater than '__einterrupts'");

ASSERT(_stext < ORIGIN(FLASH) + LENGTH(FLASH), "
The '.text' section must be placed inside the FLASH memory
Set '_stext' to an address smaller than 'ORIGIN(FLASH) + LENGTH(FLASH)");

/* This has been temporarily omitted because it's not supported by LLD */
/* ASSERT(__sbss % 4 == 0 && __ebss % 4 == 0, " */
/* .bss is not 4-byte aligned at its boundaries. This is a cortex-m-rt bug."); */

/* ASSERT(__sdata % 4 == 0 && __edata % 4 == 0, " */
/* .data is not 4-byte aligned at its boundaries. This is a cortex-m-rt bug."); */

/* ASSERT(__sidata % 4 == 0, " */
/* __sidata is not 4-byte aligned. This is a cortex-m-rt bug."); */

ASSERT(__sgot == __egot, "
.got section detected in the input files. Dynamic relocations are not
supported. If you are linking to C code compiled using the `gcc` crate
then modify your build script to compile the C code _without_ the
-fPIC flag. See the documentation of the `gcc::Config.fpic` method for
details.");
