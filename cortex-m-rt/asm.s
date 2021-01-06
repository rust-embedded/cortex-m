  .cfi_sections .debug_frame

  # LLD requires that the section flags are explicitly set here
  .section .HardFaultTrampoline, "ax"
  .global HardFaultTrampoline
  # .type and .thumb_func are both required; otherwise its Thumb bit does not
  # get set and an invalid vector table is generated
  .type HardFaultTrampoline,%function
  .thumb_func
  .cfi_startproc
HardFaultTrampoline:
  # depending on the stack mode in EXC_RETURN, fetch stack pointer from
  # PSP or MSP
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

  .section .text.FpuTrampoline, "ax"
  .global FpuTrampoline
  # .type and .thumb_func are both required; otherwise its Thumb bit does not
  # get set and an invalid vector table is generated
  .type FpuTrampoline,%function
  .thumb_func
  .cfi_startproc
  # This enables the FPU and jumps to the main function.
FpuTrampoline:
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
  # Hand execution over to `main`.
  bl main
  # Note: `main` must not return. `bl` is used only because it has a wider range than `b`.
  .cfi_endproc
  .size FpuTrampoline, . - FpuTrampoline

  # ARMv6-M leaves LR in an unknown state on Reset
  # this trampoline sets LR before it's pushed onto the stack by Reset
  .section .PreResetTrampoline, "ax"
  .global PreResetTrampoline
  # .type and .thumb_func are both required; otherwise its Thumb bit does not
  # get set and an invalid vector table is generated
  .type PreResetTrampoline,%function
  .thumb_func
  .cfi_startproc
PreResetTrampoline:
  # set LR to the initial value used by the ARMv7-M (0xFFFF_FFFF)
  ldr r4,=0xffffffff
  mov lr,r4

  # run the pre-init code
  bl __pre_init

  # the call above clobbers LR, but tools may expect LR to be 0xFFFFFFFF when reaching the first
  # call frame, so we restore it to its previous value (r4 is preserved by subroutines)
  mov lr,r4

  # initialize .data and .bss memory
  ldr r0,=__sbss
  ldr r1,=__ebss
  ldr r2,=0
0:
  cmp r1, r0
  beq 1f
  stm r0!, {r2}
  b 0b
1:

  # copy to here
  ldr r0,=__sdata
  # ...up to here
  ldr r1,=__edata
  # copy from here
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

  # jump to Rust
  b Reset
  .cfi_endproc
  .size PreResetTrampoline, . - PreResetTrampoline
