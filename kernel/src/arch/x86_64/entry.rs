global_asm!(
    r#"
.section .multiboot,"ax"
.align 8
multiboot_header:
.long 0xe85250d6
.long 0
.long multiboot_header_end - multiboot_header
.long -(0xe85250d6 + 0 + (multiboot_header_end - multiboot_header))

mb_address_tag_start:
.short 2
.short 0
.long mb_address_tag_end - mb_address_tag_start
.long multiboot_header
.long _image_start
.long _image_bss
.long _image_end
mb_address_tag_end:

mb_entry_address_tag_start:
.short 3
.short 0
.long mb_entry_address_tag_end - mb_entry_address_tag_start
.long _start
mb_entry_address_tag_end:

.long 0

mb_end_tag:
.short 0
.short 0
.long 8
mb_end_tag_end:

multiboot_header_end:
"#
);

global_asm!(include_str!("page_tables.S"));

global_asm!(
    "
.section .rodata.boot
gdt:
.quad 0x0
.quad 0x0020980000000000
.quad 0x0000920000000000
gdtr:
.short gdtr - gdt - 1
.quad gdt
"
);

#[repr(C, align(0x1000))]
#[derive(Copy, Clone)]
struct StackPage([u8; 0x1000]);
const STACK_LEN: usize = 16;
const STACK_SIZE: usize = STACK_LEN * 0x1000;
#[link_section = ".bss"]
static mut STACK: [StackPage; STACK_LEN] = [StackPage([0; 0x1000]); STACK_LEN];

#[naked]
#[no_mangle]
#[link_section = ".text.boot"]
pub unsafe extern "C" fn _start() {
    asm!("
        .code32
        cmp $$0x36d76289, %eax
        jne halt32

        mov %ebx, %edi

        mov $$PHYS_IDENT_PML4, %eax
        mov %eax, %cr3

        mov %cr4, %eax
        or $$0x20, %eax
        mov %eax, %cr4

        mov $$0xc0000080, %ecx
        rdmsr
        or $$0x100, %eax
        wrmsr

        mov %cr0, %eax
        or $$0x80000000, %eax
        or $$(1 << 16), %eax    // WP
        mov %eax, %cr0

        lgdt gdtr
        jmpl $$0x8,$$trampoline

        halt32:
        cli
        hlt
        jmp halt32

        .code64
        trampoline:
        mov $$0x10, %rax
        mov %rax, %ds
        mov %rax, %es
        mov %rax, %fs
        mov %rax, %gs
        mov %rax, %ss

        movabsq $0, %rax
        add $1, %rax
        mov %rax, %rsp

        mov %cr4, %rax
        or $$(1 << 16), %rax
        mov %rax, %cr4

        mov $$1f, %rax
        add $$0xffffffff80000000, %rax
        jmpq *%rax
        1:
        movq $$0, PHYS_IDENT_PML4

        movabsq $2, %rax
        jmpq *%rax
        "
        :: "s" (&STACK), "n" (STACK_SIZE), "s" (super::multiboot2_entry as usize)
    );
}
