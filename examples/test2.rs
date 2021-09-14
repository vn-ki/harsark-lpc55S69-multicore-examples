#![no_main]
#![no_std]

use core::sync::atomic::{AtomicBool, Ordering};

pub static TASKMANAGER_LOCK: AtomicBool = AtomicBool::new(false);

pub fn spinlock_try<'a>(lock: &'a AtomicBool) -> Result<bool, bool> {
    lock.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
}

pub fn spinlock<'a>(lock: &'a AtomicBool) {
    while let Err(_) =
        lock.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) {
            // do nothing
        }
}

pub fn spinunlock<'a>(lock: &'a AtomicBool) {
    lock.store(false, Ordering::SeqCst);
}

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
        spinlock(&TASKMANAGER_LOCK);
        // GREEN.toggle();
        toggle(0);
        spinunlock(&TASKMANAGER_LOCK);
    }
}

#[no_mangle]
#[link_section = ".text_1.1"]
unsafe extern "C" fn main_1() -> ! {
    BLUE.on();
    // hprintln!("hello world");

    loop {
        for _ in 1..10000 {}
        spinlock(&TASKMANAGER_LOCK);
        // BLUE.toggle();
        toggle(1);
        spinunlock(&TASKMANAGER_LOCK);
        // atomic::compiler_fence(Ordering::SeqCst);
    }
}
