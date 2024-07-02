MEMORY
{
    FLASH : ORIGIN = 0x08000000, LENGTH = 1M
    RAM : ORIGIN = 0x20000000, LENGTH = 128K
    CCRAM : ORIGIN = 0x10000000, LENGTH = 64K
}

_stack_start = ORIGIN(RAM) + LENGTH(RAM);

/*_stext = ORIGIN(FLASH) + 0x40C*/

/* Advanced users can place the stack inthe CCRAM */
/* which is smaller but faster. */
/* _stack_start = ORIGIN(CCRAM) + LENGTH(CCRAM); */

/*
### start code address for I/D Code bus  0x0000 0000
### data area (SRAM) starts from address 0x2000 0000 accessed through the system bus
### SRAM1 0x2000 0000 - 0x2001 BFFF
### SRAM2 0x2001 C000 - 0x2001 FFFF
### FLASH 0x0800 0000 - 0x080F FFFF
*/