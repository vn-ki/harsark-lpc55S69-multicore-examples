set directories ../harsark.rs/
set print asm-demangle on

break DefaultHandler
break HardFault
break rust_begin_unwind
# break PendSV_0
# break harsark::utils::arch::PendSV_0

set history save on
set confirm off

# find commit-hash using `rustc -Vv`
set substitute-path /rustc/b663c0f4f6ff84a8c9df0f708e1f8d628330d973 /home/vn-ki/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust

target extended-remote :2331
load
monitor reset
monitor semihosting enable
# monitor semihosting breakOnError <digit>
# by default (1) output goes to Telnet client, 2 sends to GDB client, 3 would send to both
monitor semihosting IOClient 3

#monitor swo enabletarget 0 0 1 0
# mon SWO EnableTarget 0 48000000 1875000 0
# continue
# stepi

#source ~/.gdbinit-gdb-dashboard
