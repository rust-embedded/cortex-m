  # LLD requires that the section flags are explicitly set here
  .section .HardFaultTrampoline, "ax"
  .global HardFaultTrampoline
  # .type and .thumb_func are both required; otherwise its Thumb bit does not
  # get set and an invalid vector table is generated
  .type HardFaultTrampoline,%function
  .thumb_func
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
