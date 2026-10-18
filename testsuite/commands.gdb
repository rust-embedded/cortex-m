target extended-remote :1234
break main
layout asm
layout regs
set logging file ./target/debug.log
set logging enabled on
stepi
