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

pub const ENTRY_FUNCTIONS: [unsafe fn(); 256] = [
    interrupt_0_entry,
    interrupt_1_entry,
    interrupt_2_entry,
    interrupt_3_entry,
    interrupt_4_entry,
    interrupt_5_entry,
    interrupt_6_entry,
    interrupt_7_entry,
    interrupt_8_entry,
    interrupt_9_entry,
    interrupt_10_entry,
    interrupt_11_entry,
    interrupt_12_entry,
    interrupt_13_entry,
    interrupt_14_entry,
    interrupt_entry_halt,
    interrupt_16_entry,
    interrupt_17_entry,
    interrupt_18_entry,
    interrupt_19_entry,
    interrupt_20_entry,
    interrupt_21_entry,
    interrupt_entry_halt,
    interrupt_entry_halt,
    interrupt_entry_halt,
    interrupt_entry_halt,
    interrupt_entry_halt,
    interrupt_entry_halt,
    interrupt_entry_halt,
    interrupt_entry_halt,
    interrupt_entry_halt,
    interrupt_entry_halt,
    interrupt_32_entry,
    interrupt_33_entry,
    interrupt_34_entry,
    interrupt_35_entry,
    interrupt_36_entry,
    interrupt_37_entry,
    interrupt_38_entry,
    interrupt_39_entry,
    interrupt_40_entry,
    interrupt_41_entry,
    interrupt_42_entry,
    interrupt_43_entry,
    interrupt_44_entry,
    interrupt_45_entry,
    interrupt_46_entry,
    interrupt_47_entry,
    interrupt_48_entry,
    interrupt_49_entry,
    interrupt_50_entry,
    interrupt_51_entry,
    interrupt_52_entry,
    interrupt_53_entry,
    interrupt_54_entry,
    interrupt_55_entry,
    interrupt_56_entry,
    interrupt_57_entry,
    interrupt_58_entry,
    interrupt_59_entry,
    interrupt_60_entry,
    interrupt_61_entry,
    interrupt_62_entry,
    interrupt_63_entry,
    interrupt_64_entry,
    interrupt_65_entry,
    interrupt_66_entry,
    interrupt_67_entry,
    interrupt_68_entry,
    interrupt_69_entry,
    interrupt_70_entry,
    interrupt_71_entry,
    interrupt_72_entry,
    interrupt_73_entry,
    interrupt_74_entry,
    interrupt_75_entry,
    interrupt_76_entry,
    interrupt_77_entry,
    interrupt_78_entry,
    interrupt_79_entry,
    interrupt_80_entry,
    interrupt_81_entry,
    interrupt_82_entry,
    interrupt_83_entry,
    interrupt_84_entry,
    interrupt_85_entry,
    interrupt_86_entry,
    interrupt_87_entry,
    interrupt_88_entry,
    interrupt_89_entry,
    interrupt_90_entry,
    interrupt_91_entry,
    interrupt_92_entry,
    interrupt_93_entry,
    interrupt_94_entry,
    interrupt_95_entry,
    interrupt_96_entry,
    interrupt_97_entry,
    interrupt_98_entry,
    interrupt_99_entry,
    interrupt_100_entry,
    interrupt_101_entry,
    interrupt_102_entry,
    interrupt_103_entry,
    interrupt_104_entry,
    interrupt_105_entry,
    interrupt_106_entry,
    interrupt_107_entry,
    interrupt_108_entry,
    interrupt_109_entry,
    interrupt_110_entry,
    interrupt_111_entry,
    interrupt_112_entry,
    interrupt_113_entry,
    interrupt_114_entry,
    interrupt_115_entry,
    interrupt_116_entry,
    interrupt_117_entry,
    interrupt_118_entry,
    interrupt_119_entry,
    interrupt_120_entry,
    interrupt_121_entry,
    interrupt_122_entry,
    interrupt_123_entry,
    interrupt_124_entry,
    interrupt_125_entry,
    interrupt_126_entry,
    interrupt_127_entry,
    interrupt_128_entry,
    interrupt_129_entry,
    interrupt_130_entry,
    interrupt_131_entry,
    interrupt_132_entry,
    interrupt_133_entry,
    interrupt_134_entry,
    interrupt_135_entry,
    interrupt_136_entry,
    interrupt_137_entry,
    interrupt_138_entry,
    interrupt_139_entry,
    interrupt_140_entry,
    interrupt_141_entry,
    interrupt_142_entry,
    interrupt_143_entry,
    interrupt_144_entry,
    interrupt_145_entry,
    interrupt_146_entry,
    interrupt_147_entry,
    interrupt_148_entry,
    interrupt_149_entry,
    interrupt_150_entry,
    interrupt_151_entry,
    interrupt_152_entry,
    interrupt_153_entry,
    interrupt_154_entry,
    interrupt_155_entry,
    interrupt_156_entry,
    interrupt_157_entry,
    interrupt_158_entry,
    interrupt_159_entry,
    interrupt_160_entry,
    interrupt_161_entry,
    interrupt_162_entry,
    interrupt_163_entry,
    interrupt_164_entry,
    interrupt_165_entry,
    interrupt_166_entry,
    interrupt_167_entry,
    interrupt_168_entry,
    interrupt_169_entry,
    interrupt_170_entry,
    interrupt_171_entry,
    interrupt_172_entry,
    interrupt_173_entry,
    interrupt_174_entry,
    interrupt_175_entry,
    interrupt_176_entry,
    interrupt_177_entry,
    interrupt_178_entry,
    interrupt_179_entry,
    interrupt_180_entry,
    interrupt_181_entry,
    interrupt_182_entry,
    interrupt_183_entry,
    interrupt_184_entry,
    interrupt_185_entry,
    interrupt_186_entry,
    interrupt_187_entry,
    interrupt_188_entry,
    interrupt_189_entry,
    interrupt_190_entry,
    interrupt_191_entry,
    interrupt_192_entry,
    interrupt_193_entry,
    interrupt_194_entry,
    interrupt_195_entry,
    interrupt_196_entry,
    interrupt_197_entry,
    interrupt_198_entry,
    interrupt_199_entry,
    interrupt_200_entry,
    interrupt_201_entry,
    interrupt_202_entry,
    interrupt_203_entry,
    interrupt_204_entry,
    interrupt_205_entry,
    interrupt_206_entry,
    interrupt_207_entry,
    interrupt_208_entry,
    interrupt_209_entry,
    interrupt_210_entry,
    interrupt_211_entry,
    interrupt_212_entry,
    interrupt_213_entry,
    interrupt_214_entry,
    interrupt_215_entry,
    interrupt_216_entry,
    interrupt_217_entry,
    interrupt_218_entry,
    interrupt_219_entry,
    interrupt_220_entry,
    interrupt_221_entry,
    interrupt_222_entry,
    interrupt_223_entry,
    interrupt_224_entry,
    interrupt_225_entry,
    interrupt_226_entry,
    interrupt_227_entry,
    interrupt_228_entry,
    interrupt_229_entry,
    interrupt_230_entry,
    interrupt_231_entry,
    interrupt_232_entry,
    interrupt_233_entry,
    interrupt_234_entry,
    interrupt_235_entry,
    interrupt_236_entry,
    interrupt_237_entry,
    interrupt_238_entry,
    interrupt_239_entry,
    interrupt_240_entry,
    interrupt_241_entry,
    interrupt_242_entry,
    interrupt_243_entry,
    interrupt_244_entry,
    interrupt_245_entry,
    interrupt_246_entry,
    interrupt_247_entry,
    interrupt_248_entry,
    interrupt_249_entry,
    interrupt_250_entry,
    interrupt_251_entry,
    interrupt_252_entry,
    interrupt_253_entry,
    interrupt_254_entry,
    interrupt_255_entry,
];


#[naked]
#[no_mangle]
pub unsafe fn interrupt_entry_halt() {
    asm!("
        cli
        hlt
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_0_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$0, %rdi
        mov $$int_handler_divide_error, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_1_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$1, %rdi
        mov $$int_handler_debug_exception, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_2_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$2, %rdi
        mov $$int_handler_nmi, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_3_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$3, %rdi
        mov $$int_handler_breakpoint, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_4_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$4, %rdi
        mov $$int_handler_overflow, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_5_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$5, %rdi
        mov $$int_handler_bound_range_exceeded, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_6_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$6, %rdi
        mov $$int_handler_invalid_opcode, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_7_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$7, %rdi
        mov $$int_handler_no_math_coprocessor, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_8_entry() {
    asm!("
        xchg (%rsp), %rdx
        push %rdi
        push %rax
        mov $$8, %rdi
        mov $$int_handler_double_fault, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_9_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$9, %rdi
        mov $$int_handler_coprocessor_segment_overrun, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_10_entry() {
    asm!("
        xchg (%rsp), %rdx
        push %rdi
        push %rax
        mov $$10, %rdi
        mov $$int_handler_invalid_tss, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_11_entry() {
    asm!("
        xchg (%rsp), %rdx
        push %rdi
        push %rax
        mov $$11, %rdi
        mov $$int_handler_segment_not_present, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_12_entry() {
    asm!("
        xchg (%rsp), %rdx
        push %rdi
        push %rax
        mov $$12, %rdi
        mov $$int_handler_stack_segment_fault, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_13_entry() {
    asm!("
        xchg (%rsp), %rdx
        push %rdi
        push %rax
        mov $$13, %rdi
        mov $$int_handler_general_protection, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_14_entry() {
    asm!("
        xchg (%rsp), %rdx
        push %rdi
        push %rax
        mov $$14, %rdi
        mov $$int_handler_page_fault, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_16_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$16, %rdi
        mov $$int_handler_fp_error, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_17_entry() {
    asm!("
        xchg (%rsp), %rdx
        push %rdi
        push %rax
        mov $$17, %rdi
        mov $$int_handler_alignment_check, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_18_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$18, %rdi
        mov $$int_handler_machine_check, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_19_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$19, %rdi
        mov $$int_handler_simd_error, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_20_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$20, %rdi
        mov $$int_handler_virtualization_exception, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_21_entry() {
    asm!("
        xchg (%rsp), %rdx
        push %rdi
        push %rax
        mov $$21, %rdi
        mov $$int_handler_control_protection_exception, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_32_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$32, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_33_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$33, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_34_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$34, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_35_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$35, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_36_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$36, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_37_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$37, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_38_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$38, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_39_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$39, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_40_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$40, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_41_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$41, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_42_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$42, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_43_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$43, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_44_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$44, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_45_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$45, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_46_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$46, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_47_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$47, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_48_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$48, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_49_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$49, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_50_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$50, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_51_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$51, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_52_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$52, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_53_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$53, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_54_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$54, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_55_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$55, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_56_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$56, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_57_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$57, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_58_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$58, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_59_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$59, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_60_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$60, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_61_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$61, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_62_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$62, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_63_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$63, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_64_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$64, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_65_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$65, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_66_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$66, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_67_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$67, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_68_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$68, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_69_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$69, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_70_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$70, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_71_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$71, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_72_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$72, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_73_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$73, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_74_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$74, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_75_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$75, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_76_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$76, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_77_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$77, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_78_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$78, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_79_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$79, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_80_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$80, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_81_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$81, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_82_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$82, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_83_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$83, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_84_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$84, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_85_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$85, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_86_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$86, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_87_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$87, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_88_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$88, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_89_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$89, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_90_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$90, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_91_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$91, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_92_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$92, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_93_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$93, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_94_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$94, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_95_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$95, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_96_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$96, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_97_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$97, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_98_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$98, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_99_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$99, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_100_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$100, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_101_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$101, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_102_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$102, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_103_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$103, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_104_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$104, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_105_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$105, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_106_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$106, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_107_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$107, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_108_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$108, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_109_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$109, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_110_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$110, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_111_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$111, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_112_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$112, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_113_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$113, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_114_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$114, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_115_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$115, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_116_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$116, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_117_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$117, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_118_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$118, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_119_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$119, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_120_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$120, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_121_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$121, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_122_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$122, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_123_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$123, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_124_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$124, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_125_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$125, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_126_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$126, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_127_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$127, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_128_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$128, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_129_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$129, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_130_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$130, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_131_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$131, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_132_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$132, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_133_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$133, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_134_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$134, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_135_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$135, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_136_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$136, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_137_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$137, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_138_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$138, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_139_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$139, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_140_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$140, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_141_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$141, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_142_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$142, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_143_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$143, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_144_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$144, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_145_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$145, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_146_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$146, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_147_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$147, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_148_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$148, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_149_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$149, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_150_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$150, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_151_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$151, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_152_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$152, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_153_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$153, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_154_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$154, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_155_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$155, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_156_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$156, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_157_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$157, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_158_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$158, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_159_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$159, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_160_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$160, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_161_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$161, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_162_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$162, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_163_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$163, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_164_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$164, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_165_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$165, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_166_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$166, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_167_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$167, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_168_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$168, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_169_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$169, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_170_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$170, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_171_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$171, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_172_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$172, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_173_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$173, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_174_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$174, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_175_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$175, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_176_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$176, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_177_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$177, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_178_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$178, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_179_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$179, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_180_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$180, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_181_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$181, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_182_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$182, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_183_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$183, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_184_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$184, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_185_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$185, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_186_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$186, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_187_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$187, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_188_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$188, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_189_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$189, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_190_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$190, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_191_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$191, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_192_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$192, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_193_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$193, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_194_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$194, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_195_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$195, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_196_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$196, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_197_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$197, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_198_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$198, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_199_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$199, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_200_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$200, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_201_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$201, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_202_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$202, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_203_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$203, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_204_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$204, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_205_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$205, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_206_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$206, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_207_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$207, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_208_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$208, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_209_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$209, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_210_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$210, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_211_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$211, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_212_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$212, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_213_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$213, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_214_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$214, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_215_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$215, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_216_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$216, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_217_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$217, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_218_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$218, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_219_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$219, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_220_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$220, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_221_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$221, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_222_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$222, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_223_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$223, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_224_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$224, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_225_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$225, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_226_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$226, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_227_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$227, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_228_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$228, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_229_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$229, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_230_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$230, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_231_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$231, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_232_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$232, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_233_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$233, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_234_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$234, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_235_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$235, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_236_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$236, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_237_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$237, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_238_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$238, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_239_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$239, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_240_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$240, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_241_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$241, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_242_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$242, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_243_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$243, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_244_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$244, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_245_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$245, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_246_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$246, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_247_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$247, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_248_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$248, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_249_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$249, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_250_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$250, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_251_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$251, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_252_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$252, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_253_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$253, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_254_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$254, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_255_entry() {
    asm!("
        push %rdx
        push %rdi
        push %rax
        mov $$255, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe fn interrupt_x_entry() {
    asm!("
        push %rcx
        push %rsi
        push %r8
        push %r9
        push %r10
        push %r11

        mov 0x50(%rsp), %rsi

        cmpq $$8, %rsi
        je 1f
        swapgs
    1:

        call *%rax
        cli

        cmpq $$8, %rsi
        je 1f
        swapgs
    1:

        pop %r11
        pop %r10
        pop %r9
        pop %r8
        pop %rsi
        pop %rcx

        pop %rax
        pop %rdi
        pop %rdx

        iretq
    " :::: "volatile");
}


