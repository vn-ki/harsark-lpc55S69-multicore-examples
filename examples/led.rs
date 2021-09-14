#![no_main]
#![no_std]

use core::sync::atomic::{self, Ordering};

use cortex_m_semihosting::hprintln;
use lpc55s6x::{BLUE, GREEN};
use panic_halt as _;

#[inline(never)]
fn toggle(col: u8) {
    if col == 0 {
        GREEN.toggle();
    } else {
        BLUE.toggle();
    }
}

#[no_mangle]
// #[link_section = ".text_0.1"]
unsafe extern "C" fn main_0() -> ! {
    GREEN.on();

    loop {
        for _ in 1..10000 {}
        // GREEN.toggle();
        toggle(0);
    }
}

#[no_mangle]
#[link_section = ".text_1.1"]
unsafe extern "C" fn main_1() -> ! {
    BLUE.on();
    // hprintln!("hello world");

    loop {
        for _ in 1..10000 {}
        // BLUE.toggle();
        toggle(1);
        // atomic::compiler_fence(Ordering::SeqCst);
    }
}
