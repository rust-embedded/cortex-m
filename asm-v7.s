  .syntax unified
  .cfi_sections .debug_frame

  .section .text.__basepri_max
  .global __basepri_max
  .thumb_func
  .cfi_startproc
__basepri_max:
  msr BASEPRI_MAX, r0
  bx lr
  .cfi_endproc
  .size __basepri_max, . - __basepri_max

  .section .text.__basepri_r
  .global __basepri_r
  .thumb_func
  .cfi_startproc
__basepri_r:
  mrs r0, BASEPRI
  bx lr
  .cfi_endproc
  .size __basepri_r, . - __basepri_r

  .section .text.__basepri_w
  .global __basepri_w
  .thumb_func
  .cfi_startproc
__basepri_w:
  msr BASEPRI, r0
  bx lr
  .cfi_endproc
  .size __basepri_w, . - __basepri_w

  .section .text.__faultmask
  .global __faultmask
  .thumb_func
  .cfi_startproc
__faultmask:
  mrs r0, FAULTMASK
  bx lr
  .cfi_endproc
  .size __faultmask, . - __faultmask

  .section .text.__enable_icache
  .global __enable_icache
  .thumb_func
  .cfi_startproc
__enable_icache:
  ldr r0, =0xE000ED14       @ CCR
  mrs r2, PRIMASK           @ save critical nesting info
  cpsid i                   @ mask interrupts
  ldr r1, [r0]              @ read CCR
  orr.w r1, r1, #(1 << 17)  @ Set bit 17, IC
  str r1, [r0]              @ write it back
  dsb                       @ ensure store completes
  isb                       @ synchronize pipeline
  msr PRIMASK, r2           @ unnest critical section
  bx lr
  .cfi_endproc
  .size __enable_icache, . - __enable_icache

  .section .text.__enable_dcache
  .global __enable_dcache
  .thumb_func
  .cfi_startproc
__enable_dcache:
  ldr r0, =0xE000ED14       @ CCR
  mrs r2, PRIMASK           @ save critical nesting info
  cpsid i                   @ mask interrupts
  ldr r1, [r0]              @ read CCR
  orr.w r1, r1, #(1 << 16)  @ Set bit 16, DC
  str r1, [r0]              @ write it back
  dsb                       @ ensure store completes
  isb                       @ synchronize pipeline
  msr PRIMASK, r2           @ unnest critical section
  bx lr
  .cfi_endproc
  .size __enable_dcache, . - __enable_dcache
