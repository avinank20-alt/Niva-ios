/* NIVA OS Linker Script
 * Links the bootloader with the kernel for x86_64
 */

ENTRY(_start)

SECTIONS
{
    /* Kernel starts at 1MB */
    . = 1M;
    
    .boot :
    {
        KEEP(*(.multiboot_header))
        KEEP(*(.text.boot))
    }
    
    .text :
    {
        *(.text .text.*)
    }
    
    .rodata :
    {
        *(.rodata .rodata.*)
    }
    
    .data :
    {
        *(.data .data.*)
    }
    
    .bss :
    {
        *(COMMON)
        *(.bss .bss.*)
    }
    
    /DISCARD/ :
    {
        *(.note.GNU-stack)
        *(.gnu_debuglink)
        *(.gnu.lto_*)
    }
}
