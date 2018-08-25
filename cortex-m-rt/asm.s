  .section .text.HardFault
  .global HardFault
  .thumb_func
HardFault:
  mrs r0, MSP
  nop
  bl UserHardFault
