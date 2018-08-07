  .global __cpsid
  .thumb_func
__cpsid:
  cpsid i
  bx lr
