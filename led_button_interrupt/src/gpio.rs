 use crate::reg::* ;  
 use crate::mcu::* ;

pub fn enable_gpio_clock(port: u32) {
    let rcc_ahbenr_reg_addr = (RCC_BASE_ADDR + 0x14) as *mut u32;

    match port {
        GPIOA_BASE_ADDR => {
            reg_set_bit(rcc_ahbenr_reg_addr, 17, true);
        },

        GPIOB_BASE_ADDR => {
            reg_set_bit(rcc_ahbenr_reg_addr, 18, true);
        }


        GPIOE_BASE_ADDR => {
            reg_set_bit(rcc_ahbenr_reg_addr, 21, true);
        }

        _ => { }
    }

} 
    
pub fn set_gpio_mode_output (port: u32, pin: u32) {
    let mode_offset: u32= 0;
    let gpio_mode_reg_addr = (port + mode_offset) as *mut u32;

    let bit_position = pin * 2;
    let mode_mask = 0x3 << bit_position;
    let mode_mask_value = 0x1 << bit_position;

    unsafe {
        let mut gpio_mode_reg_value = read_register(gpio_mode_reg_addr);
        gpio_mode_reg_value = clear_bits(gpio_mode_reg_value, mode_mask);
        gpio_mode_reg_value = set_bits(gpio_mode_reg_value, mode_mask_value);

        write_register(gpio_mode_reg_addr, gpio_mode_reg_value);
    }
}

pub fn set_gpio_otyper_push_pull(port: u32, pin: u32) {
    let otyper_offset: u32 = 0x04;
    let gpio_otyper_reg_addr = (port + otyper_offset) as *mut u32;

    let otyper_mask = 0x1 << pin;
    let otyper_mask_value = 0 << pin;

    unsafe {
        let mut gpio_otyper_reg_value = read_register(gpio_otyper_reg_addr);
        gpio_otyper_reg_value = clear_bits(gpio_otyper_reg_value, otyper_mask);
        gpio_otyper_reg_value = set_bits(gpio_otyper_reg_value, otyper_mask_value);
        write_register(gpio_otyper_reg_addr, gpio_otyper_reg_value);
    }
}

pub enum PinState {
    High, 
    Low, 
    Toggle
}

pub fn set_gpio_pin_state(port: u32, pin: u32, state: PinState) {
    let bsrr_offset = 0x18;
    let gpio_bsrr_reg_addr = (port + bsrr_offset) as *mut u32;

    match state {
        PinState::High => {
            set_reg_val(gpio_bsrr_reg_addr, 1 << pin)
        }, 

        PinState::Low => {
            set_reg_val(gpio_bsrr_reg_addr, 1 << (pin + 16));
        },

        PinState::Toggle => {
            let odr_offset = 0x14;
            let gpio_odr_reg_adr = (port + odr_offset) as *mut u32;

            if register_read_bit(gpio_odr_reg_adr, pin) {
                set_reg_val(gpio_bsrr_reg_addr, 1 << (pin + 16));
            }
            else {
                set_reg_val(gpio_bsrr_reg_addr, 1 << pin)
            }
        }
    }
}