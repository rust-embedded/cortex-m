  .cfi_sections .debug_frame

  .section .text.__bkpt
  .global __bkpt
  .thumb_func
  .cfi_startproc
__bkpt:
  bkpt
  bx lr
  .cfi_endproc
  .size __bkpt, . - __bkpt

  .section .text.__control_r
  .global __control_r
  .thumb_func
  .cfi_startproc
__control_r:
  mrs r0, CONTROL
  bx lr
  .cfi_endproc
  .size __control_r, . - __control_r

  .section .text.__control_w
  .global __control_w
  .thumb_func
  .cfi_startproc
__control_w:
  msr CONTROL, r0
  bx lr
  .cfi_endproc
  .size __control_w, . - __control_w


  .section .text.__cpsid
  .global __cpsid
  .thumb_func
  .cfi_startproc
__cpsid:
  cpsid i
  bx lr
  .cfi_endproc
  .size __cpsid, . - __cpsid

  .section .text.__cpsie
  .global __cpsie
  .thumb_func
  .cfi_startproc
__cpsie:
  cpsie i
  bx lr
  .cfi_endproc
  .size __cpsie, . - __cpsie

  .section .text.__delay
  .global __delay
  .syntax unified
  .thumb_func
  .cfi_startproc
__delay:
1:
  nop
  subs r0, #1
  bne 1b  // Branch to 1 instead of __delay does not generate R_ARM_THM_JUMP8 relocation, which breaks linking on the thumbv6m-none-eabi target
  bx lr
  .cfi_endproc
  .size __delay, . - __delay

  .section .text.__dmb
  .global __dmb
  .thumb_func
  .cfi_startproc
__dmb:
  dmb 0xF
  bx lr
  .cfi_endproc
  .size __dmb, . - __dmb

  .section .text.__dsb
  .global __dsb
  .thumb_func
  .cfi_startproc
__dsb:
  dsb 0xF
  bx lr
  .cfi_endproc
  .size __dsb, . - __dsb

  .section .text.__isb
  .global __isb
  .thumb_func
  .cfi_startproc
__isb:
  isb 0xF
  bx lr
  .cfi_endproc
  .size __isb, . - __isb

  .section .text.__msp_r
  .global __msp_r
  .thumb_func
  .cfi_startproc
__msp_r:
  mrs r0, MSP
  bx lr
  .cfi_endproc
  .size __msp_r, . - __msp_r

  .section .text.__msp_w
  .global __msp_w
  .thumb_func
__msp_w:
  msr MSP, r0
  bx lr
  .size __msp_w, . - __msp_w

  .section .text.__nop
  .global __nop
  .thumb_func
  .cfi_startproc
__nop:
  bx lr
  .cfi_endproc
  .size __nop, . - __nop

  .section .text.__primask
  .global __primask
  .thumb_func
  .cfi_startproc
__primask:
  mrs r0, PRIMASK
  bx lr
  .cfi_endproc
  .size __primask, . - __primask

  .section .text.__psp_r
  .global __psp_r
  .thumb_func
  .cfi_startproc
__psp_r:
  mrs r0, PSP
  bx lr
  .cfi_endproc
  .size __psp_r, . - __psp_r

  .section .text.__psp_w
  .global __psp_w
  .thumb_func
__psp_w:
  msr PSP, r0
  bx lr
  .size __psp_w, . - __psp_w

  .section .text.__sev
  .global __sev
  .thumb_func
  .cfi_startproc
__sev:
  sev
  bx lr
  .cfi_endproc
  .size __sev, . - __sev


  .section .text.__udf
  .global __udf
  .thumb_func
  .cfi_startproc
__udf:
  udf
  .cfi_endproc
  .size __udf, . - __udf

  .section .text.__wfe
  .global __wfe
  .thumb_func
  .cfi_startproc
__wfe:
  wfe
  bx lr
  .cfi_endproc
  .size __wfe, . - __wfe


  .section .text.__wfi
  .global __wfi
  .thumb_func
  .cfi_startproc
__wfi:
  wfi
  bx lr
  .cfi_endproc
  .size __wfi, . - __wfi
