
enum ButtonStatus {
    Pressed,
    Released
}

pub fn button_init(pin: u32) {

}

pub fn button_configure_interrupt(pin: u32) {

}

pub fn button_read_status(pin: u32) -> ButtonStatus {
    ButtonStatus::Released
}