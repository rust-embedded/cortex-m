  .global __msp_r
  .thumb_func
__msp_r:
  mrs r0, MSP
  bx lr
