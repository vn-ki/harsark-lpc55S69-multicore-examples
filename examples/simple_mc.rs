#![no_main]
#![no_std]

use core::sync::atomic::{self, Ordering};

use cortex_m_semihosting::hprintln;
use lpc55s6x::{BLUE, GREEN};
use panic_halt as _;

use harsark::events;
use harsark::helpers::TaskMask;
use harsark::primitives::*;
use harsark::spawn;
use harsark::tasks::*;

#[inline(never)]
fn toggle(col: u8) {
    if col == 0 {
        GREEN.toggle();
    } else {
        BLUE.toggle();
    }
}
use bare_metal::Mutex;
use core::cell::RefCell;

#[no_mangle]
unsafe extern "C" fn main_0() -> ! {
    const task1: u32 = 1;
    const task2: u32 = 2;
    const task3: u32 = 3;

    static mut stack1: [u32; 512] = [0; 512];
    static mut stack2: [u32; 512] = [0; 512];
    static mut stack3: [u32; 512] = [0; 512];

    static sem2: Semaphore = Semaphore::new(&TaskManager, TaskMask::generate([task2]));
    static sem3: Semaphore = Semaphore::new(&TaskManager, TaskMask::generate([task3]));
    static res1: Resource<[u32; 3]> =
        Resource::new(&TaskManager, &PiStackGlobal, [1, 2, 3], TaskMask::generate([task1, task2]));
    static res2: Resource<[u32; 2]> = Resource::new(
        &TaskManager,
        &PiStackGlobal,
        [4, 5],
        TaskMask::generate([task1, task2, task3]),
    );

    spawn!(&TaskManager, task1, stack1, {
        hprintln!("TASK 1: Enter");
        // If res1 is free, then the closure passed on is executed on the resource.
        res1.acquire(|res| {
            hprintln!("TASK 1 : res1 : {:?}", res);
            sem2.signal_and_release(0);
            sem3.signal_and_release(0);
            hprintln!("TASK 1 : task 2 and 3 dispatched");
        });
        hprintln!("TASK 1: End");
    });
    spawn!(&TaskManager, task2, stack2, {
        hprintln!("TASK 2: Enter");
        res1.acquire(|res| {
            hprintln!("TASK 2 : res1 : {:?}", res);
        });
        hprintln!("TASK 2: End");
    });
    spawn!(&TaskManager, task3, stack3, {
        hprintln!("TASK 3: Enter");
        res2.acquire(|res| {
            hprintln!("TASK 3 : res2 :  {:?}", res);
        });
        hprintln!("TASK 3: End");
    });

    static mut stack0: [u32; 64] = [0; 64];
    init(&TaskManager, &mut stack0);
    release(&TaskManager, TaskMask::generate([task1]));
    start_kernel(&TaskManager)
}

#[no_mangle]
unsafe extern "C" fn main_1() -> ! {
    const task1: u32 = 1;
    const task2: u32 = 2;
    const task3: u32 = 3;

    static mut stack1: [u32; 512] = [0; 512];
    static mut stack2: [u32; 512] = [0; 512];
    static mut stack3: [u32; 512] = [0; 512];

    spawn!(&TaskManager_C1, task1, stack1, {
        hprintln!("TASK 1");
    });
    spawn!(&TaskManager_C1, task2, stack2, {
        hprintln!("TASK 2");
    });
    spawn!(&TaskManager_C1, task3, stack3, {
        hprintln!("TASK 3");
    });

    static mut stack0_C1: [u32; 64] = [0; 64];
    // Initializes the kernel in preemptive mode.
    init(&TaskManager_C1, &mut stack0_C1);

    // Releases tasks task1, task2, task3
    // logging::set_all(true);
    release(&TaskManager_C1, TaskMask::generate([task1, task2, task3]));

    start_kernel(&TaskManager_C1)
}
