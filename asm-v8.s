  .section .text.__tt
  .global __tt
  .thumb_func
__tt:
  tt r0, r0
  bx lr
  .size __tt, . - __tt

  .section .text.__ttt
  .global __ttt
  .thumb_func
__ttt:
  ttt r0, r0
  bx lr
  .size __ttt, . - __ttt

  .section .text.__tta
  .global __tta
  .thumb_func
__tta:
  tta r0, r0
  bx lr
  .size __tta, . - __tta


  .section .text.__ttat
  .global __ttat
  .thumb_func
__ttat:
  ttat r0, r0
  bx lr
  .size __ttat, . - __ttat
