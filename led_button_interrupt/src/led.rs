use crate::{gpio::*, mcu::GPIOE_BASE_ADDR};

pub fn led_init(port: u32, pin: u32) {
    // 1. Enable GPIO peripheral
    enable_gpio_clock(GPIOE_BASE_ADDR);
    // 2. Set gpio pin mode
    set_gpio_mode_output(port, pin);

    // 3. Set output type = pushpull
    set_gpio_otyper_push_pull(port, pin);

    // 4. Set output speed (optional)
}

pub fn led_on(port: u32, pin: u32) {
    set_gpio_pin_state(port, pin, PinState::High);
}

pub fn led_off(port: u32, pin: u32) {
    set_gpio_pin_state(port, pin, PinState::Low);
}

pub fn led_toggle(port: u32, pin: u32) {
    set_gpio_pin_state(port, pin, PinState::Toggle);
}
