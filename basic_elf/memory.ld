ENTRY(reset_handler)

/* Define memory region */
MEMORY
{
    FLASH (rx)      : ORIGIN = 0x08000000, LENGTH = 256K
    RAM (rwx)       : ORIGIN = 0x20000000, LENGTH = 40K
    CCMRAM (rwx)    : ORIGIN = 0x10000000, LENGTH = 8K
}

_start_of_stack = ORIGIN(RAM) + LENGTH(RAM);
_min_stack_size = 0x400;
_min_heap_size = 0x400;

SECTIONS
{
    .text :
    {
        . = ORIGIN(FLASH);
        LONG(_start_of_stack);
        KEEP(*(.isr_vector));
        *(.text)
        *(.text.*)
        . = ALIGN(4);
    } > FLASH

    .rodata :
    {
        . = ALIGN(4);
        *(.rodata)
        *(.rodata*)
        . = ALIGN(4);
    } > FLASH

    .data :
    {
        _sidata = LOADADDR(.data);
        . = ALIGN(4);
        _stada = .;
        *(.data)
        *(.data*)
        . = ALIGN(4);
        _edata = .;
    } > RAM AT > FLASH

    .bss :
    {
        . = ALIGN(4);
        _sbss = .;
        *(.bss)
        *(.bss*)
        . = ALIGN(4);
        _ebss = .;
    } > RAM

    .ram_usage_check :
    {
        . = ALIGN(8);
        . = . + _min_stack_size;
        . = . + _min_heap_size;
        . = ALIGN(8);
    } > RAM
}

INCLUDE "device_STM32F303.x"