  .global __basepri_r
  .thumb_func
__basepri_r:
  mrs r0, BASEPRI
  bx lr
