#![no_main]
#![no_std]

use cortex_m_semihosting::hprintln;
use lpc55s6x::{BLUE, RED};
use panic_halt as _;

#[rtic::app(device = lpc55s6x, cores = 2)]
const APP: () = {
    #[init(core = 0)]
    fn init0(_: init0::Context) {
        RED.on();
    }

    #[idle(core = 0)]
    fn idle0(_: idle0::Context) -> ! {
        loop {
            // hprintln!("on on 0");
            // dim LED
            BLUE.toggle();
            let mut i = 0;
            for _ in 1..10000 {
                i += 1;
            }
        }
    }

    #[init(core = 1)]
    fn init1(_: init1::Context) {
        BLUE.on();
    }

    #[idle(core = 1)]
    fn idle1(_: idle1::Context) -> ! {
        loop {
            // dim LED
            RED.toggle();
            let mut i = 0;
            for _ in 1..10000 {
                i += 1;
            }
        }
    }
};
