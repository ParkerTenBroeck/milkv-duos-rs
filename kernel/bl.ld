OUTPUT_FORMAT("elf64-littleriscv")
OUTPUT_ARCH(riscv)
ENTRY(_stage0_start)

MEMORY {
    RAM (rwx): ORIGIN = 0x80000000, LENGTH = 512M
    RAM_VIRT (rwx): ORIGIN = 0xFFFFFFC000000000, LENGTH = 512M
}

SECTIONS
{
    __KERNEL_START_PHYS__ = ABSOLUTE(0x80000000);
    __KERNEL_START_VIRT__ = ABSOLUTE(0xFFFFFFC000000000);
    __MEM_SIZE__ = ABSOLUTE(512M);

    ASSERT(. == ALIGN(4096), "base address is not aligned on a page boundary.")
    
    .text : {
        . = ALIGN(4K);
        _kernel_start = .;

        __EXEC_START__ = .;
	    *(.text.entry)
        *_os_start.o(.text*)
        *(.vectors)
        *(.text*)
        __EXEC_END__ = .;
        . = ALIGN(4K);
    } > RAM_VIRT AT> RAM

    .ro : {
        . = ALIGN(4K);
        __RO_START__ = .;
        *(.rodata*)
        __RO_END__ = .;
        . = ALIGN(4K);
    } > RAM_VIRT AT> RAM

    .data : {
        . = ALIGN(4K);
        __DATA_START__ = .;
        *(.data*)
        *(.sdata*)
        __DATA_END__ = .;
        . = ALIGN(4K);
    } > RAM_VIRT AT> RAM

    .bss : ALIGN(16) {
        . = ALIGN(4K);
        __BSS_START__ = .;
        *(SORT_BY_ALIGNMENT(.bss*))
        *(SORT_BY_ALIGNMENT(.sbss*))
        *(COMMON)
        __BSS_END__ = .;
        . = ALIGN(4K);
    } > RAM_VIRT AT> RAM

    stacks (NOLOAD) : {
        __STACKS_START__ = .;
        . += 4K*2;
        __STACKS_END__ = .;
    } > RAM_VIRT AT> RAM

    __CODE_SIZE__ = SIZEOF(.text);
    __RO_SIZE__ = SIZEOF(.ro);
    __BSS_SIZE__ = SIZEOF(.bss);
    __DATA_SIZE__ = SIZEOF(.data);
    __STACK_SIZE__ = SIZEOF(stacks);
    __KERNEL_SIZE__ = SIZEOF(.text) + SIZEOF(.ro) + SIZEOF(.bss) + SIZEOF(.data);


    . = ALIGN(4096);   
    _kernel_end = .;
}
