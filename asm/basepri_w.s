  .global __basepri_w
  .thumb_func
__basepri_w:
  msr BASEPRI, r0
  bx lr
