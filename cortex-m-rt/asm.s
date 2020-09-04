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
  ldr r0,=0xffffffff
  mov lr,r0
  b Reset
  .cfi_endproc
  .size PreResetTrampoline, . - PreResetTrampoline
