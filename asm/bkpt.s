  .global __bkpt
  .thumb_func
__bkpt:
  bkpt
  bx lr
