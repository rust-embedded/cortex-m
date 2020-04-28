  .section .text.__msplim_r
  .global __msplim_r
  .size __msplim_r, . - __msplim_r
  .thumb_func
__msplim_r:
  mrs r0, MSPLIM
  bx lr

  .section .text.__msplim_w
  .global __msplim_w
  .size __msplim_w, . - __msplim_w
  .thumb_func
__msplim_w:
  msr MSPLIM, r0
  bx lr

  .section .text.__psplim_r
  .global __psplim_r
  .size __psplim_r, . - __psplim_r
  .thumb_func
__psplim_r:
  mrs r0, PSPLIM
  bx lr

  .section .text.__psplim_w
  .global __psplim_w
  .size __psplim_w, . - __psplim_w
  .thumb_func
__psplim_w:
  msr PSPLIM, r0
  bx lr

