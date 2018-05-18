  .global __delay
  .syntax unified
  .thumb_func
__delay:
  nop
  subs r0, #1
  bne __delay
  bx lr
