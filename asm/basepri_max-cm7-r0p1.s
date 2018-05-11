  .global __basepri_max
  .syntax unified
__basepri_max:
  mrs r1, PRIMASK
  cpsid i
  tst.w r1, #1
  msr BASEPRI_MAX, r0
  it ne
  bxne lr
  cpsie i
  bx lr
