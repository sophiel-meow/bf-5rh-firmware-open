MEMORY
{
  RAM : ORIGIN = 0x2000006c, LENGTH = 8192
}

ENTRY(app_entry);

SECTIONS
{
  .text : {
    *(.text .text.*)
  } > RAM

  .rodata : {
    *(.rodata .rodata.*)
  } > RAM

  .data : {
    *(.data .data.*)
  } > RAM

  .bss (NOLOAD) : {
    *(.bss .bss.*)
    *(COMMON)
  } > RAM

  /DISCARD/ : {
    *(.ARM.exidx*)
    *(.ARM.extab*)
    *(.comment)
    *(.debug*)
    *(.eh_frame*)
  }
}
