/* Oversized segment: the loader extends the arena into the bottom of the
 * stack, so LENGTH here is ARENA_MAX, not ARENA_SIZE. */
MEMORY
{
  RAM : ORIGIN = 0x200000b8, LENGTH = 32768
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
    . = ALIGN(4);
    __app_end = .;
  } > RAM

  /DISCARD/ : {
    *(.ARM.exidx*)
    *(.ARM.extab*)
    *(.comment)
    *(.debug*)
    *(.eh_frame*)
  }
}
