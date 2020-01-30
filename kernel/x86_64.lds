PROVIDE(KERNEL_LMA = 0x100000);
PROVIDE(KERNEL_VMA = 0xffffffff80000000);

SECTIONS {
    . = KERNEL_LMA + SIZEOF_HEADERS;

    PROVIDE(_image_start = KERNEL_LMA);

    .multiboot : AT(ADDR(.multiboot)) {
        KEEP(*(.multiboot))
        *(.multiboot)
    }


    .text.boot : AT(ADDR(.text.boot)) {
        KEEP(*(.text.boot))
        *(.text.boot)
    }

    . += KERNEL_VMA;
    .text : AT(ADDR(.text) - KERNEL_VMA) {
        *(.text.*)
    }
    . -= KERNEL_VMA;


    .rodata.boot : AT(ADDR(.rodata.boot)) {
        *(.rodata.boot)
    }

    . += KERNEL_VMA;
    .rodata : AT(ADDR(.rodata) - KERNEL_VMA) {
        *(.rodata.*)
    }
    . -= KERNEL_VMA;


    .data.boot : AT(ADDR(.data.boot)) {
        *(.data.boot)
    }

    . += KERNEL_VMA;
    .data : AT(ADDR(.data) - KERNEL_VMA) {
        *(.data.*)
    }
    . -= KERNEL_VMA;


    .bss.boot : AT(ADDR(.bss.boot)) {
        *(.bss.boot)
    }

    . += KERNEL_VMA;
    .bss : AT(ADDR(.bss) - KERNEL_VMA) {
        *(.bss.*)
    }
    . -= KERNEL_VMA;


    PROVIDE(_image_bss = ADDR(.bss.boot));
    PROVIDE(_image_end = .);
}
