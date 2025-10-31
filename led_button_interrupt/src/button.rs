use crate::gpio::*;

enum ButtonStatus {
    Pressed,
    Released,
}

pub enum Trigger {
    FallingEdge,
    RisingEdge,
}

pub enum Mode {
    Input,
    Interrupt(Trigger),
}

pub fn button_init(port: u32, pin: u32, mode: Mode) {
    enable_gpio_clock(port);
    set_gpio_mode_input(port, pin);

    match mode {
        Mode::Interrupt(trigger) => {
            match trigger {
                Trigger::FallingEdge => {
                    // Configure the PIN for falling edge detection
                }
                Trigger::RisingEdge => {
                    // Configure the PIN for rising edge detection
                }
            }
        }

        Mode::Input => {
            // Do nothing
        }
    }
}

// pub fn button_configure_interrupt(pin: u32) {

// }

// pub fn button_read_status(pin: u32) -> ButtonStatus {
//     ButtonStatus::Released
// }
