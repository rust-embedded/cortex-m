  .cfi_sections .debug_frame

  .section .text.__tt
  .global __tt
  .thumb_func
  .cfi_startproc
__tt:
  tt r0, r0
  bx lr
  .cfi_endproc
  .size __tt, . - __tt

  .section .text.__ttt
  .global __ttt
  .thumb_func
  .cfi_startproc
__ttt:
  ttt r0, r0
  bx lr
  .cfi_endproc
  .size __ttt, . - __ttt

  .section .text.__tta
  .global __tta
  .thumb_func
  .cfi_startproc
__tta:
  tta r0, r0
  bx lr
  .cfi_endproc
  .size __tta, . - __tta


  .section .text.__ttat
  .global __ttat
  .thumb_func
  .cfi_startproc
__ttat:
  ttat r0, r0
  bx lr
  .cfi_endproc
  .size __ttat, . - __ttat
