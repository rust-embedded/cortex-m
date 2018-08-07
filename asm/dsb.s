  .global __dsb
  .thumb_func
__dsb:
  dsb 0xF
  bx lr
