use core::ptr;

pub fn clear_bits(value: u32, mask: u32) -> u32 {
    value & !mask
}

pub fn set_bits(value: u32, mask_value: u32) -> u32 {
    value | mask_value
}

pub unsafe fn read_register(addr: *mut u32) -> u32 {
    unsafe { ptr::read_volatile(addr) }
}

pub unsafe fn write_register(addr: *mut u32, value: u32) {
    unsafe { ptr::write_volatile(addr, value) }
}

pub fn set_reg_val(addr: *mut u32, value: u32) {
    unsafe {
        write_register(addr, value);
    }
}

pub fn register_read_bit(addr: *mut u32, bit: u32) -> bool {
    unsafe {
        let reg_value = read_register(addr);
        (reg_value & 1 << bit) != 0
    }
}

pub fn reg_set_bit(addr: *mut u32, bit_position: u32, bit_val: bool) {
    unsafe {
        let reg_value = read_register(addr);

        let updated_value = if bit_val {
            reg_value | (1 << bit_position)
        } else {
            reg_value & !(1 << bit_position)
        };

        write_register(addr, updated_value);
    }
}
