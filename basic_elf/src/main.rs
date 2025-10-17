#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

mod startup_stm32f303;
use core::panic::PanicInfo;


static mut SCORES_GLOBAL : [i32; 5] = [1, 2, 3, 4, 5];
const _NUMBERS : [i32; 5] = [1, 2, 3, 4, 5];
static mut BUFFER: [u8; 1024] = [0; 1024];

#[unsafe(no_mangle)]
fn main() -> ! {

    let mut _total_score: i32 = 0;

    unsafe {
        for score in SCORES_GLOBAL {
            _total_score += score;
        }
    }

    unsafe {
        BUFFER[0] = 1;
    }

    loop {
    }
}

#[panic_handler]
fn basic_panic(_info: &PanicInfo) -> ! {
    loop {}
}