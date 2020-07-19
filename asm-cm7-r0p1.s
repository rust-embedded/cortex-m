  .cfi_sections .debug_frame

  .section .text.__basepri_max_cm7_r0p1
  .global __basepri_max_cm7_r0p1
  .syntax unified
  .thumb_func
  .cfi_startproc
__basepri_max_cm7_r0p1:
  mrs r1, PRIMASK
  cpsid i
  tst.w r1, #1
  msr BASEPRI_MAX, r0
  it ne
  bxne lr
  cpsie i
  bx lr
  .cfi_endproc
  .size __basepri_max_cm7_r0p1, . - __basepri_max_cm7_r0p1

  .section .text.__basepri_w_cm7_r0p1
  .global __basepri_w_cm7_r0p1
  .syntax unified
  .thumb_func
  .cfi_startproc
__basepri_w_cm7_r0p1:
  mrs r1, PRIMASK
  cpsid i
  tst.w r1, #1
  msr BASEPRI, r0
  it ne
  bxne lr
  cpsie i
  bx lr
  .cfi_endproc
  .size __basepri_w_cm7_r0p1, . - __basepri_w_cm7_r0p1
