  .global __basepri_max
__basepri_max:
  msr BASEPRI_MAX, r0
  bx lr
