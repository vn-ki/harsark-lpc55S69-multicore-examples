target extended-remote :2331

set print asm-demangle on

break DefaultHandler
break HardFault
break rust_begin_unwind

monitor reset
monitor semihosting enable

#break *0xb5d0
#break *0x1ad6
#break shared.rs:76
#break shared.rs:78
break *0x2ce
break *0x502

set logging overwrite on
set logging file log.txt
set logging on

set confirm off

load

# start the process but immediately halt the processor
#c
#x 0xe0001004
#c
#x 0xe0001004
#quit
