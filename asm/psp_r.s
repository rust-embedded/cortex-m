  .global __psp_r
  .thumb_func
__psp_r:
  mrs r0, PSP
  bx lr
