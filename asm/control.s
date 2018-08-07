  .global __control
  .thumb_func
__control:
  mrs r0, CONTROL
  bx lr
