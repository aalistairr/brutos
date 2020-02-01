#!/usr/bin/env python3

ERROR_CODE_VECTORS = [8, 10, 11, 12, 13, 14, 17, 21]
HANDLERS = {
    0: "divide_error",
    1: "debug_exception",
    2: "nmi",
    3: "breakpoint",
    4: "overflow",
    5: "bound_range_exceeded",
    6: "invalid_opcode",
    7: "no_math_coprocessor",
    8: "double_fault",
    9: "coprocessor_segment_overrun",
    10: "invalid_tss",
    11: "segment_not_present",
    12: "stack_segment_fault",
    13: "general_protection",
    14: "page_fault",
    16: "fp_error",
    17: "alignment_check",
    18: "machine_check",
    19: "simd_error",
    20: "virtualization_exception",
    21: "control_protection_exception",
}


def is_reserved(i):
    return i == 15 or (i >= 22 and i <= 31)


class Builder:
    def __init__(self, i, indent=8):
        self.asm = ""
        self.has_error_code = i in ERROR_CODE_VECTORS
        self.indent = indent

    def write(self, s):
        self.asm += ' ' * self.indent
        self.asm += s
        self.asm += '\n'


print('pub mod vectors {')
for (vector, name) in HANDLERS.items():
    print(f'    pub const {name.upper()}: usize = {vector};')
print('}')
print()

print(f'pub const HANDLERS: [unsafe fn(); 256] = [')
for i in range(0, 256):
    if not is_reserved(i):
        print(f'    int_handler_entry_{i},')
    else:
        print(f'    halt,')
print(f'];')

print("""
#[naked]
unsafe fn halt() {
    asm!("cli\nhlt" :::: "volatile");
}
""")

# Stack layout:
# (aligned to 16 bytes)
#   SS                  +0x28   +0x20
#   RSP                 +0x20   +0x18
#   RFLAGS              +0x18   +0x10
#   CS                  +0x10   +0x08
#   RIP                 +0x08   +0x00
#   (error code)        +0x00   -----


for i in range(0, 256):
    if is_reserved(i):
        continue

    handler = HANDLERS.get(i)
    if handler == None:
        handler = "any"

    b = Builder(i)
    if b.has_error_code:
        b.write(f'cmpq $$0x8, 0x10(%rsp)')
    else:
        b.write(f'cmpq $$0x8, 0x8(%rsp)')
    b.write(f'je 1f')
    b.write(f'swapgs')
    b.write(f'1:')
    b.write(f'')
    b.write(f'mov %rax, %gs:0x00')
    b.write(f'mov %rcx, %gs:0x10')
    b.write(f'mov %rdx, %gs:0x18')
    b.write(f'mov %rdi, %gs:0x20')
    b.write(f'mov %rsi, %gs:0x28')
    b.write(f'mov  %r8, %gs:0x40')
    b.write(f'mov  %r9, %gs:0x48')
    b.write(f'mov %r10, %gs:0x50')
    b.write(f'mov %r11, %gs:0x58')
    b.write(f'')

    b.write(f'mov $${i}, %rdi')

    if b.has_error_code:
        b.write(f'mov 0x10(%rsp), %rsi')
        b.write(f'mov (%rsp), %rdx')
    else:
        b.write(f'mov 0x8(%rsp), %rsi')
        b.write(f'sub $$0x8, %rsp')

    b.write(f'call int_handler_{handler}')
    b.write(f'add $$0x8, %rsp')

    b.write(f'')
    b.write(f'mov %gs:0x58, %r11')
    b.write(f'mov %gs:0x50, %r10')
    b.write(f'mov %gs:0x48,  %r9')
    b.write(f'mov %gs:0x40,  %r8')
    b.write(f'mov %gs:0x28, %rsi')
    b.write(f'mov %gs:0x20, %rdi')
    b.write(f'mov %gs:0x18, %rdx')
    b.write(f'mov %gs:0x10, %rcx')
    b.write(f'mov %gs:0x00, %rax')
    b.write(f'')
    if b.has_error_code:
        b.write(f'cmpq $$0x8, 0x16(%rsp)')
    else:
        b.write(f'cmpq $$0x8, 0x8(%rsp)')
    b.write(f'je 1f')
    b.write(f'swapgs')
    b.write(f'1:')
    b.write(f'')
    b.write(f'iretq')

    print(
        f"""
#[naked]
pub unsafe fn int_handler_entry_{i}() {{
    asm!("\n{b.asm}    " :::: "volatile");
}}
""")
