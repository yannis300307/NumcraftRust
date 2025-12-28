MEMORY {
  SRAM (rw) : ORIGIN = 0x00000000, LENGTH = 16K
  FLASH (rx) : ORIGIN = 0x90200200 + (0), LENGTH = 6M
}

SECTIONS {
  .entrypoint ORIGIN(FLASH) : {
    KEEP(*(.entrypoint))
  } >FLASH

  .init_array : {
    . = ALIGN(4);
    _init_array_start = .;
    KEEP (*(.init_array*))
    _init_array_end = .;
  } >FLASH

  .text : {
    . = ALIGN(4);
    *(.text)
    *(.text.*)
  } >FLASH

  .rodata : {
    . = ALIGN(4);
    *(.rodata)
    *(.rodata.*)
  } >FLASH

  .ARM.exidx : {
    __exidx_start = .;
/*    *(.ARM.exidx* .gnu.linkonce.armexidx.*) */
    __exidx_end = .;
  } >FLASH

  .data : {
    . = ALIGN(4);
    _data_section_start_flash = LOADADDR(.data);
    _data_section_start_ram = .;
    *(.data)
    *(.data.*)
    _data_section_end_ram = .;
  } >SRAM AT> FLASH

  .bss : {
    . = ALIGN(4);
    _bss_section_start_ram = .;
    *(.bss)
    *(.bss.*)
    *(COMMON)
    _bss_section_end_ram = .;
  } >SRAM
}
