pub mod vectors {
    pub const DIVIDE_ERROR: usize = 0;
    pub const DEBUG_EXCEPTION: usize = 1;
    pub const NMI: usize = 2;
    pub const BREAKPOINT: usize = 3;
    pub const OVERFLOW: usize = 4;
    pub const BOUND_RANGE_EXCEEDED: usize = 5;
    pub const INVALID_OPCODE: usize = 6;
    pub const NO_MATH_COPROCESSOR: usize = 7;
    pub const DOUBLE_FAULT: usize = 8;
    pub const COPROCESSOR_SEGMENT_OVERRUN: usize = 9;
    pub const INVALID_TSS: usize = 10;
    pub const SEGMENT_NOT_PRESENT: usize = 11;
    pub const STACK_SEGMENT_FAULT: usize = 12;
    pub const GENERAL_PROTECTION: usize = 13;
    pub const PAGE_FAULT: usize = 14;
    pub const FP_ERROR: usize = 16;
    pub const ALIGNMENT_CHECK: usize = 17;
    pub const MACHINE_CHECK: usize = 18;
    pub const SIMD_ERROR: usize = 19;
    pub const VIRTUALIZATION_EXCEPTION: usize = 20;
    pub const CONTROL_PROTECTION_EXCEPTION: usize = 21;
}

pub const HANDLERS: [unsafe fn(); 256] = [
    int_handler_entry_0,
    int_handler_entry_1,
    int_handler_entry_2,
    int_handler_entry_3,
    int_handler_entry_4,
    int_handler_entry_5,
    int_handler_entry_6,
    int_handler_entry_7,
    int_handler_entry_8,
    int_handler_entry_9,
    int_handler_entry_10,
    int_handler_entry_11,
    int_handler_entry_12,
    int_handler_entry_13,
    int_handler_entry_14,
    halt,
    int_handler_entry_16,
    int_handler_entry_17,
    int_handler_entry_18,
    int_handler_entry_19,
    int_handler_entry_20,
    int_handler_entry_21,
    halt,
    halt,
    halt,
    halt,
    halt,
    halt,
    halt,
    halt,
    halt,
    halt,
    int_handler_entry_32,
    int_handler_entry_33,
    int_handler_entry_34,
    int_handler_entry_35,
    int_handler_entry_36,
    int_handler_entry_37,
    int_handler_entry_38,
    int_handler_entry_39,
    int_handler_entry_40,
    int_handler_entry_41,
    int_handler_entry_42,
    int_handler_entry_43,
    int_handler_entry_44,
    int_handler_entry_45,
    int_handler_entry_46,
    int_handler_entry_47,
    int_handler_entry_48,
    int_handler_entry_49,
    int_handler_entry_50,
    int_handler_entry_51,
    int_handler_entry_52,
    int_handler_entry_53,
    int_handler_entry_54,
    int_handler_entry_55,
    int_handler_entry_56,
    int_handler_entry_57,
    int_handler_entry_58,
    int_handler_entry_59,
    int_handler_entry_60,
    int_handler_entry_61,
    int_handler_entry_62,
    int_handler_entry_63,
    int_handler_entry_64,
    int_handler_entry_65,
    int_handler_entry_66,
    int_handler_entry_67,
    int_handler_entry_68,
    int_handler_entry_69,
    int_handler_entry_70,
    int_handler_entry_71,
    int_handler_entry_72,
    int_handler_entry_73,
    int_handler_entry_74,
    int_handler_entry_75,
    int_handler_entry_76,
    int_handler_entry_77,
    int_handler_entry_78,
    int_handler_entry_79,
    int_handler_entry_80,
    int_handler_entry_81,
    int_handler_entry_82,
    int_handler_entry_83,
    int_handler_entry_84,
    int_handler_entry_85,
    int_handler_entry_86,
    int_handler_entry_87,
    int_handler_entry_88,
    int_handler_entry_89,
    int_handler_entry_90,
    int_handler_entry_91,
    int_handler_entry_92,
    int_handler_entry_93,
    int_handler_entry_94,
    int_handler_entry_95,
    int_handler_entry_96,
    int_handler_entry_97,
    int_handler_entry_98,
    int_handler_entry_99,
    int_handler_entry_100,
    int_handler_entry_101,
    int_handler_entry_102,
    int_handler_entry_103,
    int_handler_entry_104,
    int_handler_entry_105,
    int_handler_entry_106,
    int_handler_entry_107,
    int_handler_entry_108,
    int_handler_entry_109,
    int_handler_entry_110,
    int_handler_entry_111,
    int_handler_entry_112,
    int_handler_entry_113,
    int_handler_entry_114,
    int_handler_entry_115,
    int_handler_entry_116,
    int_handler_entry_117,
    int_handler_entry_118,
    int_handler_entry_119,
    int_handler_entry_120,
    int_handler_entry_121,
    int_handler_entry_122,
    int_handler_entry_123,
    int_handler_entry_124,
    int_handler_entry_125,
    int_handler_entry_126,
    int_handler_entry_127,
    int_handler_entry_128,
    int_handler_entry_129,
    int_handler_entry_130,
    int_handler_entry_131,
    int_handler_entry_132,
    int_handler_entry_133,
    int_handler_entry_134,
    int_handler_entry_135,
    int_handler_entry_136,
    int_handler_entry_137,
    int_handler_entry_138,
    int_handler_entry_139,
    int_handler_entry_140,
    int_handler_entry_141,
    int_handler_entry_142,
    int_handler_entry_143,
    int_handler_entry_144,
    int_handler_entry_145,
    int_handler_entry_146,
    int_handler_entry_147,
    int_handler_entry_148,
    int_handler_entry_149,
    int_handler_entry_150,
    int_handler_entry_151,
    int_handler_entry_152,
    int_handler_entry_153,
    int_handler_entry_154,
    int_handler_entry_155,
    int_handler_entry_156,
    int_handler_entry_157,
    int_handler_entry_158,
    int_handler_entry_159,
    int_handler_entry_160,
    int_handler_entry_161,
    int_handler_entry_162,
    int_handler_entry_163,
    int_handler_entry_164,
    int_handler_entry_165,
    int_handler_entry_166,
    int_handler_entry_167,
    int_handler_entry_168,
    int_handler_entry_169,
    int_handler_entry_170,
    int_handler_entry_171,
    int_handler_entry_172,
    int_handler_entry_173,
    int_handler_entry_174,
    int_handler_entry_175,
    int_handler_entry_176,
    int_handler_entry_177,
    int_handler_entry_178,
    int_handler_entry_179,
    int_handler_entry_180,
    int_handler_entry_181,
    int_handler_entry_182,
    int_handler_entry_183,
    int_handler_entry_184,
    int_handler_entry_185,
    int_handler_entry_186,
    int_handler_entry_187,
    int_handler_entry_188,
    int_handler_entry_189,
    int_handler_entry_190,
    int_handler_entry_191,
    int_handler_entry_192,
    int_handler_entry_193,
    int_handler_entry_194,
    int_handler_entry_195,
    int_handler_entry_196,
    int_handler_entry_197,
    int_handler_entry_198,
    int_handler_entry_199,
    int_handler_entry_200,
    int_handler_entry_201,
    int_handler_entry_202,
    int_handler_entry_203,
    int_handler_entry_204,
    int_handler_entry_205,
    int_handler_entry_206,
    int_handler_entry_207,
    int_handler_entry_208,
    int_handler_entry_209,
    int_handler_entry_210,
    int_handler_entry_211,
    int_handler_entry_212,
    int_handler_entry_213,
    int_handler_entry_214,
    int_handler_entry_215,
    int_handler_entry_216,
    int_handler_entry_217,
    int_handler_entry_218,
    int_handler_entry_219,
    int_handler_entry_220,
    int_handler_entry_221,
    int_handler_entry_222,
    int_handler_entry_223,
    int_handler_entry_224,
    int_handler_entry_225,
    int_handler_entry_226,
    int_handler_entry_227,
    int_handler_entry_228,
    int_handler_entry_229,
    int_handler_entry_230,
    int_handler_entry_231,
    int_handler_entry_232,
    int_handler_entry_233,
    int_handler_entry_234,
    int_handler_entry_235,
    int_handler_entry_236,
    int_handler_entry_237,
    int_handler_entry_238,
    int_handler_entry_239,
    int_handler_entry_240,
    int_handler_entry_241,
    int_handler_entry_242,
    int_handler_entry_243,
    int_handler_entry_244,
    int_handler_entry_245,
    int_handler_entry_246,
    int_handler_entry_247,
    int_handler_entry_248,
    int_handler_entry_249,
    int_handler_entry_250,
    int_handler_entry_251,
    int_handler_entry_252,
    int_handler_entry_253,
    int_handler_entry_254,
    int_handler_entry_255,
];

#[naked]
unsafe fn halt() {
    asm!("cli
hlt" :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_0() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$0, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_divide_error
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_1() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$1, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_debug_exception
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_2() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$2, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_nmi
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_3() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$3, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_breakpoint
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_4() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$4, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_overflow
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_5() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$5, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_bound_range_exceeded
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_6() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$6, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_invalid_opcode
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_7() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$7, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_no_math_coprocessor
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_8() {
    asm!("
        cmpq $$0x8, 0x10(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$8, %rdi
        mov 0x10(%rsp), %rsi
        mov (%rsp), %rdx
        call int_handler_double_fault
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x16(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_9() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$9, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_coprocessor_segment_overrun
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_10() {
    asm!("
        cmpq $$0x8, 0x10(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$10, %rdi
        mov 0x10(%rsp), %rsi
        mov (%rsp), %rdx
        call int_handler_invalid_tss
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x16(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_11() {
    asm!("
        cmpq $$0x8, 0x10(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$11, %rdi
        mov 0x10(%rsp), %rsi
        mov (%rsp), %rdx
        call int_handler_segment_not_present
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x16(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_12() {
    asm!("
        cmpq $$0x8, 0x10(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$12, %rdi
        mov 0x10(%rsp), %rsi
        mov (%rsp), %rdx
        call int_handler_stack_segment_fault
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x16(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_13() {
    asm!("
        cmpq $$0x8, 0x10(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$13, %rdi
        mov 0x10(%rsp), %rsi
        mov (%rsp), %rdx
        call int_handler_general_protection
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x16(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_14() {
    asm!("
        cmpq $$0x8, 0x10(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$14, %rdi
        mov 0x10(%rsp), %rsi
        mov (%rsp), %rdx
        call int_handler_page_fault
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x16(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_16() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$16, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_fp_error
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_17() {
    asm!("
        cmpq $$0x8, 0x10(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$17, %rdi
        mov 0x10(%rsp), %rsi
        mov (%rsp), %rdx
        call int_handler_alignment_check
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x16(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_18() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$18, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_machine_check
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_19() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$19, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_simd_error
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_20() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$20, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_virtualization_exception
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_21() {
    asm!("
        cmpq $$0x8, 0x10(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$21, %rdi
        mov 0x10(%rsp), %rsi
        mov (%rsp), %rdx
        call int_handler_control_protection_exception
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x16(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_32() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$32, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_33() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$33, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_34() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$34, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_35() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$35, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_36() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$36, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_37() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$37, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_38() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$38, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_39() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$39, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_40() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$40, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_41() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$41, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_42() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$42, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_43() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$43, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_44() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$44, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_45() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$45, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_46() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$46, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_47() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$47, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_48() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$48, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_49() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$49, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_50() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$50, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_51() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$51, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_52() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$52, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_53() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$53, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_54() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$54, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_55() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$55, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_56() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$56, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_57() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$57, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_58() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$58, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_59() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$59, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_60() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$60, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_61() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$61, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_62() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$62, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_63() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$63, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_64() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$64, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_65() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$65, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_66() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$66, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_67() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$67, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_68() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$68, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_69() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$69, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_70() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$70, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_71() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$71, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_72() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$72, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_73() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$73, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_74() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$74, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_75() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$75, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_76() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$76, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_77() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$77, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_78() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$78, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_79() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$79, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_80() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$80, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_81() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$81, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_82() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$82, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_83() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$83, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_84() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$84, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_85() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$85, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_86() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$86, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_87() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$87, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_88() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$88, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_89() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$89, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_90() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$90, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_91() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$91, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_92() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$92, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_93() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$93, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_94() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$94, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_95() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$95, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_96() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$96, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_97() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$97, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_98() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$98, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_99() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$99, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_100() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$100, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_101() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$101, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_102() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$102, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_103() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$103, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_104() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$104, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_105() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$105, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_106() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$106, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_107() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$107, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_108() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$108, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_109() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$109, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_110() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$110, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_111() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$111, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_112() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$112, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_113() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$113, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_114() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$114, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_115() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$115, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_116() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$116, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_117() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$117, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_118() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$118, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_119() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$119, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_120() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$120, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_121() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$121, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_122() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$122, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_123() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$123, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_124() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$124, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_125() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$125, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_126() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$126, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_127() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$127, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_128() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$128, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_129() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$129, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_130() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$130, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_131() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$131, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_132() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$132, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_133() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$133, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_134() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$134, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_135() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$135, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_136() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$136, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_137() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$137, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_138() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$138, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_139() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$139, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_140() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$140, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_141() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$141, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_142() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$142, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_143() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$143, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_144() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$144, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_145() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$145, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_146() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$146, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_147() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$147, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_148() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$148, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_149() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$149, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_150() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$150, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_151() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$151, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_152() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$152, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_153() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$153, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_154() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$154, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_155() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$155, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_156() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$156, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_157() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$157, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_158() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$158, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_159() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$159, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_160() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$160, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_161() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$161, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_162() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$162, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_163() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$163, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_164() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$164, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_165() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$165, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_166() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$166, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_167() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$167, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_168() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$168, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_169() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$169, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_170() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$170, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_171() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$171, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_172() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$172, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_173() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$173, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_174() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$174, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_175() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$175, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_176() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$176, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_177() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$177, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_178() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$178, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_179() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$179, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_180() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$180, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_181() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$181, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_182() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$182, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_183() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$183, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_184() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$184, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_185() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$185, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_186() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$186, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_187() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$187, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_188() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$188, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_189() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$189, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_190() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$190, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_191() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$191, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_192() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$192, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_193() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$193, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_194() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$194, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_195() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$195, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_196() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$196, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_197() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$197, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_198() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$198, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_199() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$199, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_200() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$200, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_201() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$201, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_202() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$202, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_203() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$203, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_204() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$204, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_205() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$205, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_206() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$206, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_207() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$207, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_208() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$208, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_209() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$209, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_210() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$210, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_211() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$211, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_212() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$212, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_213() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$213, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_214() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$214, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_215() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$215, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_216() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$216, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_217() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$217, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_218() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$218, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_219() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$219, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_220() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$220, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_221() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$221, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_222() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$222, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_223() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$223, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_224() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$224, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_225() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$225, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_226() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$226, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_227() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$227, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_228() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$228, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_229() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$229, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_230() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$230, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_231() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$231, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_232() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$232, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_233() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$233, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_234() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$234, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_235() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$235, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_236() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$236, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_237() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$237, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_238() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$238, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_239() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$239, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_240() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$240, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_241() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$241, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_242() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$242, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_243() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$243, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_244() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$244, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_245() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$245, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_246() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$246, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_247() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$247, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_248() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$248, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_249() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$249, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_250() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$250, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_251() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$251, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_252() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$252, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_253() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$253, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_254() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$254, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}


#[naked]
pub unsafe fn int_handler_entry_255() {
    asm!("
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        mov %rax, %gs:0x00
        mov %rcx, %gs:0x10
        mov %rdx, %gs:0x18
        mov %rdi, %gs:0x20
        mov %rsi, %gs:0x28
        mov  %r8, %gs:0x40
        mov  %r9, %gs:0x48
        mov %r10, %gs:0x50
        mov %r11, %gs:0x58
        
        mov $$255, %rdi
        mov 0x8(%rsp), %rsi
        sub $$0x8, %rsp
        call int_handler_any
        add $$0x8, %rsp
        
        mov %gs:0x58, %r11
        mov %gs:0x50, %r10
        mov %gs:0x48,  %r9
        mov %gs:0x40,  %r8
        mov %gs:0x28, %rsi
        mov %gs:0x20, %rdi
        mov %gs:0x18, %rdx
        mov %gs:0x10, %rcx
        mov %gs:0x00, %rax
        
        cmpq $$0x8, 0x8(%rsp)
        je 1f
        swapgs
        1:
        
        iretq
    " :::: "volatile");
}

