  .section .text.__syscall
  .global __syscall
  .thumb_func
__syscall:
  bkpt 0xAB
  bx lr
