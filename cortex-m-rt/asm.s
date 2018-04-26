  .global HardFault
HardFault:
  mrs r0, MSP
  b UserHardFault
