  .global __msp_w
  .thumb_func
__msp_w:
  msr MSP, r0
  bx lr
