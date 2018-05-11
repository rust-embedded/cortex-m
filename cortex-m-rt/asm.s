  .global HardFault
HardFault:
  mrs r0, MSP
  bl UserHardFault
