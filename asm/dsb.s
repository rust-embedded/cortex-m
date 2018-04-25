  .global __dsb
__dsb:
  dsb 0xF
  bx lr
