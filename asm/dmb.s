  .global __dmb
  .thumb_func
__dmb:
  dmb 0xF
  bx lr
