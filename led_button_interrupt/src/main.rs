#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]
#![allow(dead_code)]

mod startup_stm32f303;
mod led;
// mod button;
mod mcu;
mod board;
mod reg;
mod gpio;

use core::panic::PanicInfo;

use led:: *;
// use button:: *;
use board::*;

#[unsafe(no_mangle)]
fn main() -> ! {

    led_init(BLUE_LED_PORT, BLUE_LED_PIN);
    led_off(BLUE_LED_PORT, BLUE_LED_PIN);
    led_on(BLUE_LED_PORT, BLUE_LED_PIN);
    led_off(BLUE_LED_PORT, BLUE_LED_PIN);
    // button_init(BUTTON_PIN);
    // button_configure_interrupt(BUTTON_PIN);

    loop {
    }
}

#[panic_handler]
fn basic_panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Button interrupt handler
#[allow(non_snake_case)]
fn EXTI0_Handler() {
    led_toggle(BLUE_LED_PORT, BLUE_LED_PIN);
}