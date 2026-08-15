/* NIVA OS x86_64 Bootloader
 * Multiboot2 compliant bootloader for QEMU
 * This code sets up basic CPU features and passes control to the kernel
 */

.section .multiboot_header
.align 8
.long 0xe85250d6                   # Multiboot2 magic number
.long 0                            # Architecture (x86, 32-bit)
.long header_end - header_start    # Header length
.long 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))  # Checksum
header_start:
.short 0  # End tag type
.short 0  # End tag flags
.long 8   # End tag size
header_end:

.section .text
.global _start
.type _start, @function

_start:
    # At this point:
    # EAX = Multiboot magic number (0x36d76289)
    # EBX = Pointer to multiboot info structure
    
    # Disable interrupts during startup
    cli
    
    # Load 64-bit GDT
    lgdt gdt_descriptor
    
    # Enable PAE (Physical Address Extension)
    mov %cr4, %eax
    or $0x20, %eax
    mov %eax, %cr4
    
    # Set up long mode page tables
    mov $page_table_l4, %eax
    mov %eax, %cr3
    
    # Enable long mode
    mov $0xC0000080, %ecx  # EFER MSR
    rdmsr
    or $0x100, %eax        # LME flag
    wrmsr
    
    # Enable paging
    mov %cr0, %eax
    or $0x80000000, %eax
    mov %eax, %cr0
    
    # Jump to 64-bit code
    ljmp $0x8, $boot64

.align 16
page_table_l4:
    .quad page_table_l3 + 0x3  # Present + writable
    .fill 510, 8, 0
    .quad page_table_l4 + 0x3

page_table_l3:
    .quad page_table_l2 + 0x3
    .fill 511, 8, 0

page_table_l2:
    .quad 0x0 + 0x83           # 2MB page, present + writable + huge
    .quad 0x200000 + 0x83
    .quad 0x400000 + 0x83
    .quad 0x600000 + 0x83
    .fill 508, 8, 0

.code64
boot64:
    # Initialize stack
    mov $stack_top, %rsp
    
    # Clear registers
    xor %rax, %rax
    xor %rbx, %rbx
    xor %rcx, %rcx
    xor %rdx, %rdx
    xor %rsi, %rsi
    xor %rdi, %rdi
    
    # Call kernel_main
    call kernel_main
    
    # If we return, halt
    cli
    hlt
    jmp .

.section .data
.align 8
gdt:
    # Null descriptor
    .quad 0x0000000000000000
    
    # Code descriptor (64-bit)
    .quad 0x00af9a000000ffff
    
    # Data descriptor (64-bit)
    .quad 0x00af92000000ffff

.align 8
gdt_descriptor:
    .word . - gdt - 1
    .quad gdt

.section .bss
.align 4096
stack_bottom:
    .skip 4096
stack_top:
