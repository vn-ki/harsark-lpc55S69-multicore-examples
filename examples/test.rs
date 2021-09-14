#![no_main]
#![no_std]

use core::sync::atomic::{self, Ordering};

use cortex_m_semihosting::hprintln;
use lpc55s6x::{BLUE, GREEN};
use panic_halt as _;
use cortex_m::asm::{delay, bkpt};

use harsark::events;
use harsark::helpers::TaskMask;
use harsark::primitives::*;
use harsark::spawn;
use harsark::tasks::*;

static mut C1_stack1: [u32; 1024] = [0; 1024];
static mut C1_stack2: [u32; 1024] = [0; 1024];
static mut C1_stack3: [u32; 1024] = [0; 1024];

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

const task1: u32 = 1;
const task2: u32 = 2;
const task3: u32 = 3;

static sres: SharedResource<[u32; 2]> = SharedResource::new(
    [6, 5],
    TaskMask::generate([task1, task2, task3]),
    TaskMask::generate([task2]),
);

#[no_mangle]
unsafe extern "C" fn main_0() -> ! {
    static mut stack1: [u32; 1024] = [0; 1024];
    static mut stack2: [u32; 1024] = [0; 1024];
    static mut stack3: [u32; 1024] = [0; 1024];

    static sem2: Semaphore = Semaphore::new(&TaskManager, TaskMask::generate([task2]));
    static sem3: Semaphore = Semaphore::new(&TaskManager, TaskMask::generate([task3]));
    static res1: Resource<[u32; 3]> = Resource::new(
        &TaskManager,
        &PiStackGlobal,
        [1, 2, 3],
        TaskMask::generate([task1, task2]),
    );
    static res2: Resource<[u32; 2]> = Resource::new(
        &TaskManager,
        &PiStackGlobal,
        [4, 5],
        TaskMask::generate([task1, task2, task3]),
    );
    // static sres: SharedResource<[u32; 2]> = SharedResource::new([6, 5], TaskMask::generate([task1, task2, task3]), TaskMask::generate([task2]));
    let sres_c0 = sres.core0();

    spawn!(&TaskManager, task1, stack1, {
        hprintln!("TASK 1: Enter");
        // If res1 is free, then the closure passed on is executed on the resource.
        res1.acquire(|res| {
            hprintln!("CORE0 TASK 1 : res1 : {:?}", res);
            // release(TaskMask::generate([task3]));
            sem2.signal_and_release(0);
            sem3.signal_and_release(0);
            for i in 0..10000 {}
            hprintln!("CORE0 TASK 1 : task 2 and 3 dispatched");
        });
        hprintln!("CORE0 TASK 1: End");
    });
    spawn!(&TaskManager, task2, stack2, {
        hprintln!("CORE0 TASK 2: Enter");
        // let sres_c0 = sres.core0();
        // if let Err(e) = sres_c0.acquire(|res| {
        //     hprintln!("CORE 0 TASK 2 : sres :  {:?}", res);
        // }) {
        //     hprintln!("err occ: {:?}", e);
        //     panic!("ahhhhhhh");
        // }
        // hprintln!("did i reach here?");
        res1.acquire(|res| {
            hprintln!("CORE0 TASK 2 : res1 : {:?}", res);
        });
        hprintln!("CORE0 TASK 2: End");
    });
    spawn!(&TaskManager, task3, stack3, {
        hprintln!("TASK 3: Enter");
        let sres_c0 = sres.core0();
        if let Err(e) = sres_c0.acquire(|res| {
            hprintln!("CORE 0 TASK 3 : sres :  {:?}", res);
        }) {
            hprintln!("err occ: {:?}", e);
        }
        res2.acquire(|res| {
            hprintln!("CORE0 TASK 3 : res2 :  {:?}", res);
        });
        hprintln!("CORE0 TASK 3: End");
    });

    static mut stack: [u32; 64] = [0; 64];
    init(&TaskManager, &mut stack);
    release(&TaskManager, TaskMask::generate([task1]));
    start_kernel(&TaskManager)
}

#[no_mangle]
unsafe extern "C" fn main_1() -> ! {
    const task1: u32 = 1;
    const task2: u32 = 2;
    const task3: u32 = 3;
    static sem3: Semaphore = Semaphore::new(&TaskManager_C1, TaskMask::generate([task3]));

    spawn!(&TaskManager_C1, task1, C1_stack1, {
        hprintln!("CORE 1 TASK 1");
    });
    spawn!(&TaskManager_C1, task2, C1_stack2, {
        hprintln!("CORE 1 TASK 2 ENTER");
        let sres_c1 = sres.core1();
        if let Err(e) = sres_c1.acquire(|res| {
            hprintln!("CORE1 TASK 2 : sres :  {:?} acquired", res);
            delay(10000);
            hprintln!("task 3 is released");
            sem3.signal_and_release(0);
            delay(10000);
            hprintln!("CORE1 TASK 2 : sres released");
        }) {
            hprintln!("err occ: {:?}", e);
        }
        hprintln!("CORE1 TASK 2 EXIT");
    });
    spawn!(&TaskManager_C1, task3, C1_stack3, {
        hprintln!("CORE1 TASK 3");
        bkpt();
        delay(10000);
    });

    static mut stack0_C1: [u32; 64] = [0; 64];
    // Initializes the kernel in preemptive mode.
    init(&TaskManager_C1, &mut stack0_C1);

    // Releases tasks task1, task2, task3
    // logging::set_all(true);
    release(&TaskManager_C1, TaskMask::generate([task1, task2]));

    start_kernel(&TaskManager_C1)
}
