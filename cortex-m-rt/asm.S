  .cfi_sections .debug_frame

  # Notes for function attributes:
  # .type and .thumb_func are _both_ required, otherwise the Thumb mode bit
  # will not be set and an invalid vector table is generated.
  # LLD requires that section flags are set explicitly.

  .section .HardFaultTrampoline, "ax"
  .global HardFaultTrampoline
  .type HardFaultTrampoline,%function
  .thumb_func
  .cfi_startproc
  # HardFault exceptions are bounced through this trampoline which grabs the
  # stack pointer at the time of the exception and passes it to the user's
  # HardFault handler in r0.
HardFaultTrampoline:
  # Depending on the stack mode in EXC_RETURN, fetch stack pointer from
  # PSP or MSP.
  mov r0, lr
  mov r1, #4
  tst r0, r1
  bne 0f
  mrs r0, MSP
  b HardFault
0:
  mrs r0, PSP
  b HardFault
  .cfi_endproc
  .size HardFaultTrampoline, . - HardFaultTrampoline

  .section .Reset, "ax"
  .global Reset
  .type Reset,%function
  .thumb_func
  .cfi_startproc
  # Main entry point after reset. This jumps to the user __pre_init function,
  # which cannot be called from Rust code without invoking UB, then
  # initialises RAM. If the target has an FPU, it is enabled. Finally, jumps
  # to the user main function.
Reset:
  # ARMv6-M does not initialise LR, but many tools expect it to be 0xFFFF_FFFF
  # when reaching the first call frame, so we set it at startup.
  # ARMv7-M and above initialise LR to 0xFFFF_FFFF at reset.
  ldr r4,=0xffffffff
  mov lr,r4

  # Run user pre-init code, which must be executed immediately after startup,
  # before the potentially time-consuming memory initialisation takes place.
  # Example use cases include disabling default watchdogs or enabling RAM.
  bl __pre_init

  # Restore LR after calling __pre_init (r4 is preserved by subroutines).
  mov lr,r4

  # Initialise .bss memory. `__sbss` and `__ebss` come from the linker script.
  ldr r0,=__sbss
  ldr r1,=__ebss
  mov r2,#0
0:
  cmp r1, r0
  beq 1f
  stm r0!, {r2}
  b 0b
1:

  # Initialise .data memory. `__sdata`, `__sidata`, and `__edata` come from the
  # linker script. Copy from r2 into r0 until r0 reaches r1.
  ldr r0,=__sdata
  ldr r1,=__edata
  ldr r2,=__sidata
2:
  cmp r1, r0
  beq 3f
  # load 1 word from r2 to r3, inc r2
  ldm r2!, {r3}
  # store 1 word from r3 to r0, inc r0
  stm r0!, {r3}
  b 2b
3:

#ifdef HAS_FPU
  # Conditionally enable the FPU.
  # Address of SCB.CPACR.
  ldr r0, =0xE000ED88
  # Enable access to CP10 and CP11 from both privileged and unprivileged mode.
  ldr r1, =(0b1111 << 20)
  # RMW.
  ldr r2, [r0]
  orr r2, r2, r1
  str r2, [r0]
  # Barrier is required on some processors.
  dsb
  isb
#endif

4:
  # Jump to user main function. We use bl for the extended range, but the
  # user main function may not return.
  bl main

  # Trap on return.
  udf

  .cfi_endproc
  .size Reset, . - Reset
