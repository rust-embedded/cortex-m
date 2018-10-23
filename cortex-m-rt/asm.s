  .section .text.HardFault
  .global HardFault
  .thumb_func
HardFault:
  push {r0, lr}
  mrs r0, MSP
  bl UserHardFault
  pop {r0, pc}
