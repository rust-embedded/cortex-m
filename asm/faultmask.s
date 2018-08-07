  .global __faultmask
  .thumb_func
__faultmask:
  mrs r0, FAULTMASK
  bx lr
