  .global __dmb
__dmb:
  dmb 0xF
  bx lr
