  .global __primask
  .thumb_func
__primask:
  mrs r0, PRIMASK
  bx lr
