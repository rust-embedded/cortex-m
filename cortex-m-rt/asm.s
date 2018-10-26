  # LLD requires that the section flags are explicitly set here
  .section .HardFault, "ax"
  .global HardFault
  # .type and .thumb_func are both required; otherwise its Thumb bit does not
  # get set and an invalid vector table is generated
  .type HardFault,%function
  .thumb_func
HardFault:
  mrs r0, MSP
  b UserHardFault
