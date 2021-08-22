MEMORY
{
  FLASH (RX) : ORIGIN = 0x10000000, LENGTH = 2048K
  FLASH_CACHE (RWX) : ORIGIN = 0x15000000, LENGTH = 16K
  RAM (RWX) : ORIGIN = 0x20000000, LENGTH = 264K
  BOOTLOADER (RX) : ORIGIN = ORIGIN(RAM)+LENGTH(RAM)-0x100, LENGTH = 0x100 /* ORIGIN = 0x20041F00 */
}

INPUT(bootloader/start.o)
EXTERN(start)

SECTIONS
{
  stack_start = ORIGIN(RAM) + LENGTH(RAM);

  .program : {
    bootloader/start.o(.text);
    *(.main);
    *(.text .text.*);
    *(.rodata .rodata.*);
    . = ALIGN(256);
  } > RAM

  .initialized : {
    *(.interrupt_vector);
    data_source = .;
    data_start = .;
    *(.data .data.*);
    . = ALIGN(4);
    data_end = .;
  } > RAM

  .cleared (NOLOAD) : {
    bss_start = .;
    *(.bss .bss.*);
    . = ALIGN(4);
    bss_end = .;
  } > RAM

  /DISCARD/ : {
    *(.ARM.*);
  }
}

ENTRY(start);