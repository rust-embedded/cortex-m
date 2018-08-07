  .global __basepri_w
  .syntax unified
  .thumb_func
__basepri_w:
  mrs r1, PRIMASK
  cpsid i
  tst.w r1, #1
  msr BASEPRI, r0
  it ne
  bxne lr
  cpsie i
  bx lr
