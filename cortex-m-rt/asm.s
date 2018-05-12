  .global HardFault
  .thumb_func
HardFault:
  mrs r0, MSP
  bl UserHardFault
