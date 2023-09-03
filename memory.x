/*MEMORY*/
/*{*/
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* TODO Adjust these memory regions to match your device memory layout */
  /* These values correspond to the LM3S6965, one of the few devices QEMU can emulate */
/*  FLASH : ORIGIN = 0x00000000, LENGTH = 256K*/
/*  RAM : ORIGIN = 0x20000000, LENGTH = 64K*/
/*}*/

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* You may want to use this variable to locate the call stack and static
   variables in different memory regions. Below is shown the default value */
/* _stack_start = ORIGIN(RAM) + LENGTH(RAM); */

/* You can use this symbol to customize the location of the .text section */
/* If omitted the .text section will be placed right after the .vector_table
   section */
/* This is required only on microcontrollers that store some configuration right
   after the vector table */
/* _stext = ORIGIN(FLASH) + 0x400; */

/* Example of putting non-initialized variables into custom RAM locations. */
/* This assumes you have defined a region RAM2 above, and in the Rust
   sources added the attribute `#[link_section = ".ram2bss"]` to the data
   you want to place there. */
/* Note that the section will not be zero-initialized by the runtime! */
/* SECTIONS {
     .ram2bss (NOLOAD) : ALIGN(4) {
       *(.ram2bss);
       . = ALIGN(4);
     } > RAM2
   } INSERT AFTER .bss;
*/

/* Linker script for the nRF9160 in Non-secure mode. It assumes you have the
Nordic Secure Partition Manager installed at the bottom of flash and that
the SPM is set to boot a non-secure application from the FLASH origin below. */

MEMORY
{
    /*
     * This is where the Bootloader, Secure Partition Manager or
     * Trusted-Firmware-M lives.
     */
    SECURE_FLASH : ORIGIN = 0x00000000, LENGTH = 256K
    /*
     * This is where your non-secure Rust application lives. Note that SPM must agree this
     * is where your application lives, or it will jump to garbage and crash the CPU.
     */
    FLASH        : ORIGIN = 0x00050000, LENGTH = 768K
    /*
     * This RAM is reserved for the Secure-Mode code located in the `SECURE_FLASH` region.
     */
    SECURE_RAM   : ORIGIN = 0x20000000, LENGTH = 64K
    /*
     * This RAM is available to both the Cortex-M33 and the LTE core (well,
        technically anything between `0x2000_0000` and `0x2001_FFFF` is
        shareable, but we just gave the first 64 KiB to Secure Mode). Shared
        buffers must be placed here.
     */
    SHARED_RAM   : ORIGIN = 0x20010000, LENGTH = 64K
    /*
     * This RAM is available to your non-secure Rust application.
     */
    RAM          : ORIGIN = 0x20020000, LENGTH = 128K
}

SECTIONS
{
    /* This section contains the buffers used by `libnrf_modem` to talk between the Cortex-M33 and the LTE core */
    .shared_ram (NOLOAD) : ALIGN(4)
    {
        . = ALIGN(4);
        *(.shared_ram .shared_ram.*);
        . = ALIGN(4);
    } > SHARED_RAM
}
