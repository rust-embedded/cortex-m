  .global __psp_w
  .thumb_func
__psp_w:
  msr PSP, r0
  bx lr
