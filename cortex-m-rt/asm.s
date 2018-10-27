  # LLD requires that the section flags are explicitly set here
  .section .HardFaultTrampoline, "ax"
  .global HardFaultTrampoline
  # .type and .thumb_func are both required; otherwise its Thumb bit does not
  # get set and an invalid vector table is generated
  .type HardFaultTrampoline,%function
  .thumb_func
HardFaultTrampoline:
  mrs r0, MSP
  b HardFault
