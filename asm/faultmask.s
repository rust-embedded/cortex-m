  .global __faultmask
__faultmask:
  mrs r0, FAULTMASK
  bx lr
