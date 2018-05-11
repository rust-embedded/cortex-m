  .global __basepri_w
__basepri_w:
  msr BASEPRI, r0
  bx lr
