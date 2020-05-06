
  .cfi_sections .debug_frame

  .section .text.__msplim_r
  .global __msplim_r
  .thumb_func
  .cfi_startproc
__msplim_r:
  mrs r0, MSPLIM
  bx lr
  .cfi_endproc
  .size __msplim_r, . - __msplim_r

  .section .text.__msplim_w
  .global __msplim_w
  .thumb_func
  .cfi_startproc
__msplim_w:
  msr MSPLIM, r0
  bx lr
  .cfi_endproc
  .size __msplim_w, . - __msplim_w

  .section .text.__psplim_r
  .global __psplim_r
  .thumb_func
  .cfi_startproc
__psplim_r:
  mrs r0, PSPLIM
  bx lr
  .cfi_endproc
  .size __psplim_r, . - __psplim_r

  .section .text.__psplim_w
  .global __psplim_w
  .thumb_func
  .cfi_startproc
__psplim_w:
  msr PSPLIM, r0
  bx lr
  .cfi_endproc
  .size __psplim_w, . - __psplim_w

