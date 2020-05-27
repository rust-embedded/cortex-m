  .cfi_sections .debug_frame

  .section .text.__get_FPSCR
  .global __get_FPSCR
  .thumb_func
  .cfi_startproc
__get_FPSCR:
  vmrs r0, fpscr
  bx lr
  .cfi_endproc
  .size __get_FPSCR, . - __get_FPSCR

  .section .text.__set_FPSCR
  .global __set_FPSCR
  .thumb_func
  .cfi_startproc
__set_FPSCR:
  vmrs fpscr, r0
  bx lr
  .cfi_endproc
  .size __set_FPSCR, . - __set_FPSCR