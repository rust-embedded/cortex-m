  .global __basepri_max
  .thumb_func
__basepri_max:
  msr BASEPRI_MAX, r0
  bx lr
