GDT_CODE_KERN = 8
ERROR_CODE_VECTORS = [8, 10, 11, 12, 13, 14, 17, 21]
HANDLERS = {
    0: 'divide_error',
    1: 'debug_exception',
    2: 'nmi',
    3: 'breakpoint',
    4: 'overflow',
    5: 'bound_range_exceeded',
    6: 'invalid_opcode',
    7: 'no_math_coprocessor',
    8: 'double_fault',
    9: 'coprocessor_segment_overrun',
    10: 'invalid_tss',
    11: 'segment_not_present',
    12: 'stack_segment_fault',
    13: 'general_protection',
    14: 'page_fault',
    16: 'fp_error',
    17: 'alignment_check',
    18: 'machine_check',
    19: 'simd_error',
    20: 'virtualization_exception',
    21: 'control_protection_exception',
}


def is_reserved(i):
    return i == 15 or (i >= 22 and i <= 31)


INDENT_COUNT = 4
INDENT = ' ' * INDENT_COUNT

fn_name = None
asm = ''


def function(name):
    global fn_name
    global asm
    if fn_name != None or asm != '':
        raise "still in a function"
    fn_name = name
    asm = ''


def write_asm(s, indent):
    global asm
    asm += indent
    asm += s
    asm += '\n'


def l(s):
    write_asm(s, INDENT)


def i(s):
    write_asm(s, INDENT * 2)


def n():
    global asm
    asm += '\n'


def emit():
    global fn_name
    global asm
    if fn_name == None or asm == '':
        raise "not in a function"

    print(f"""
#[naked]
#[no_mangle]
pub unsafe fn {fn_name}() {{
    asm!("\n{asm}{INDENT}" :::: "volatile");
}}
""")
    fn_name = None
    asm = ''


print('pub mod vectors {')
for (vector, name) in HANDLERS.items():
    print(f'{INDENT}pub const {name.upper()}: usize = {vector};')
print('}')
print()

print(f'pub const ENTRY_FUNCTIONS: [unsafe fn(); 256] = [')
for vector in range(0, 256):
    if not is_reserved(vector):
        print(f'{INDENT}interrupt_{vector}_entry,')
    else:
        print(f'{INDENT}interrupt_entry_halt,')
print(f'];')
print()


function(f'interrupt_entry_halt')
i(f'cli')
i(f'hlt')
emit()

for vector in range(0, 256):
    if is_reserved(vector):
        continue
    handler = HANDLERS.get(vector)
    if handler == None:
        handler = "any"

    function(f'interrupt_{vector}_entry')
    if vector in ERROR_CODE_VECTORS:
        i(f'xchg (%rsp), %rdx')
    else:
        i(f'push %rdx')
    i(f'push %rdi')
    i(f'push %rax')
    i(f'mov $${vector}, %rdi')
    i(f'mov $$int_handler_{handler}, %rax')
    i(f'jmp interrupt_x_entry')
    emit()

# stack layout:
# --- (aligned to 0x10)
#   SS                  +0x68
#   RSP                 +0x60
#   RFLAGS              +0x58
#   CS                  +0x50
#   RIP                 +0x48
#   %rdx                +0x40   (arg 2: error code)
#   %rdi                +0x38   (arg 0: vector)
#   %rax                +0x30   (handler function)
# --- (aligned to 0x10)
function(f'interrupt_x_entry')
i(f'push %rcx')  # +0x28
i(f'push %rsi')  # +0x20
i(f'push %r8')  # +0x18
i(f'push %r9')  # +0x10
i(f'push %r10')  # +0x08
i(f'push %r11')  # +0x00
# --- (aligned to 0x10)
n()
i(f'mov 0x50(%rsp), %rsi')  # (arg 1: CS)
i(f'cmpq $${GDT_CODE_KERN}, %rsi')
i(f'je 1f')
i(f'swapgs')
l(f'1:')
n()
i(f'call *%rax')
i(f'cli')
n()
i(f'cmpq $${GDT_CODE_KERN}, 0x50(%rsp)')
i(f'je 1f')
i(f'swapgs')
l(f'1:')
n()
i(f'pop %r11')
i(f'pop %r10')
i(f'pop %r9')
i(f'pop %r8')
i(f'pop %rsi')
i(f'pop %rcx')
n()
i(f'pop %rax')
i(f'pop %rdi')
i(f'pop %rdx')
n()
i(f'iretq')
emit()

print(asm)
