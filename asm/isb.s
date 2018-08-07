  .global __isb
  .thumb_func
__isb:
  isb 0xF
  bx lr
