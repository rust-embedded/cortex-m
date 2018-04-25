;; fn __basepri_w(u8)
;; executed in a critical section to work around a silicon erratum
  .global __basepri_w
__basepri_w:
  mrs r1, PRIMASK
  cpsid i
  tst.w r1, #1
  msr BASEPRI, r0
  it ne
  bxne lr
  cpsie i
  bx lr
