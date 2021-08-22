MEMORY
{
  FLASH (RX) : ORIGIN = 0x10000000, LENGTH = 2048K
  FLASH_CACHE (RWX) : ORIGIN = 0x15000000, LENGTH = 16K
  RAM (RWX) : ORIGIN = 0x20000000, LENGTH = 264K
  BOOTLOADER (RX) : ORIGIN = ORIGIN(RAM)+LENGTH(RAM)-0x100, LENGTH = 0x100 /* ORIGIN = 0x20041F00 */
}

INPUT(bootloader/boot2_w25q080.o)
INPUT(bootloader/start.o)
EXTERN(_stage2_boot)
EXTERN(start)

SECTIONS
{
  stack_start = ORIGIN(RAM) + LENGTH(RAM);

  .bootloader : {
    bootloader/boot2_w25q080.o(.text);
    . = ALIGN(256);
  } > BOOTLOADER AT>FLASH =0x00

  .program : {
    *(.interrupt_vector);
    bootloader/start.o(.text);
    *(.main);
    *(.text .text.*);
    *(.rodata .rodata.*);
    . = ALIGN(4);
  } > FLASH

  .initialized : {
    data_source = LOADADDR(.initialized);
    data_start = .;
    *(.data .data.*);
    . = ALIGN(4);
    data_end = .;
  } > RAM AT>FLASH

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

ENTRY(bootloader);