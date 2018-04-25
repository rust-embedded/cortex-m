;; fn __basepri_max(u8)
;; executed in a critical section to work around a silicon erratum
  .global __basepri_max
__basepri_max:
  mrs r1, PRIMASK
  cpsid i
  tst.w r1, #1
  msr BASEPRI_MAX, r0
  it ne
  bxne lr
  cpsie i
  bx lr
