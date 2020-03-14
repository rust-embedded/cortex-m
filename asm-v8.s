  .section .text.__tt
  .global __tt
  .thumb_func
__tt:
  tt r0, r0
  bx lr

  .section .text.__ttt
  .global __ttt
  .thumb_func
__ttt:
  ttt r0, r0
  bx lr

  .section .text.__tta
  .global __tta
  .thumb_func
__tta:
  tta r0, r0
  bx lr

  .section .text.__ttat
  .global __ttat
  .thumb_func
__ttat:
  ttat r0, r0
  bx lr
