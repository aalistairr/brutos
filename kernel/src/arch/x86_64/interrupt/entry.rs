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

pub const ENTRY_FUNCTIONS: [unsafe extern "C" fn(); 256] = [
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
pub unsafe extern "C" fn interrupt_entry_halt() {
    asm!("
        cli
        hlt
    " :::: "volatile");
}


#[naked]
#[no_mangle]
pub unsafe extern "C" fn interrupt_entry_functions() {
    asm!("
    interrupt_entry_unswapped_gs_prefix_start:
    .global interrupt_0_entry
    interrupt_0_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$0, %rdi
        mov $$int_handler_divide_error, %rax
        jmp interrupt_x_entry
    .global interrupt_1_entry
    interrupt_1_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$1, %rdi
        mov $$int_handler_debug_exception, %rax
        jmp interrupt_x_entry
    .global interrupt_2_entry
    interrupt_2_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$2, %rdi
        mov $$int_handler_nmi, %rax
        jmp interrupt_nmi_entry
    .global interrupt_3_entry
    interrupt_3_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$3, %rdi
        mov $$int_handler_breakpoint, %rax
        jmp interrupt_x_entry
    .global interrupt_4_entry
    interrupt_4_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$4, %rdi
        mov $$int_handler_overflow, %rax
        jmp interrupt_x_entry
    .global interrupt_5_entry
    interrupt_5_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$5, %rdi
        mov $$int_handler_bound_range_exceeded, %rax
        jmp interrupt_x_entry
    .global interrupt_6_entry
    interrupt_6_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$6, %rdi
        mov $$int_handler_invalid_opcode, %rax
        jmp interrupt_x_entry
    .global interrupt_7_entry
    interrupt_7_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$7, %rdi
        mov $$int_handler_no_math_coprocessor, %rax
        jmp interrupt_x_entry
    .global interrupt_8_entry
    interrupt_8_entry:
        xchg (%rsp), %rdx
        push %rdi
        push %rax
        mov $$8, %rdi
        mov $$int_handler_double_fault, %rax
        jmp interrupt_x_entry
    .global interrupt_9_entry
    interrupt_9_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$9, %rdi
        mov $$int_handler_coprocessor_segment_overrun, %rax
        jmp interrupt_x_entry
    .global interrupt_10_entry
    interrupt_10_entry:
        xchg (%rsp), %rdx
        push %rdi
        push %rax
        mov $$10, %rdi
        mov $$int_handler_invalid_tss, %rax
        jmp interrupt_x_entry
    .global interrupt_11_entry
    interrupt_11_entry:
        xchg (%rsp), %rdx
        push %rdi
        push %rax
        mov $$11, %rdi
        mov $$int_handler_segment_not_present, %rax
        jmp interrupt_x_entry
    .global interrupt_12_entry
    interrupt_12_entry:
        xchg (%rsp), %rdx
        push %rdi
        push %rax
        mov $$12, %rdi
        mov $$int_handler_stack_segment_fault, %rax
        jmp interrupt_x_entry
    .global interrupt_13_entry
    interrupt_13_entry:
        xchg (%rsp), %rdx
        push %rdi
        push %rax
        mov $$13, %rdi
        mov $$int_handler_general_protection, %rax
        jmp interrupt_x_entry
    .global interrupt_14_entry
    interrupt_14_entry:
        xchg (%rsp), %rdx
        push %rdi
        push %rax
        mov $$14, %rdi
        mov $$int_handler_page_fault, %rax
        jmp interrupt_x_entry
    .global interrupt_16_entry
    interrupt_16_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$16, %rdi
        mov $$int_handler_fp_error, %rax
        jmp interrupt_x_entry
    .global interrupt_17_entry
    interrupt_17_entry:
        xchg (%rsp), %rdx
        push %rdi
        push %rax
        mov $$17, %rdi
        mov $$int_handler_alignment_check, %rax
        jmp interrupt_x_entry
    .global interrupt_18_entry
    interrupt_18_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$18, %rdi
        mov $$int_handler_machine_check, %rax
        jmp interrupt_x_entry
    .global interrupt_19_entry
    interrupt_19_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$19, %rdi
        mov $$int_handler_simd_error, %rax
        jmp interrupt_x_entry
    .global interrupt_20_entry
    interrupt_20_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$20, %rdi
        mov $$int_handler_virtualization_exception, %rax
        jmp interrupt_x_entry
    .global interrupt_21_entry
    interrupt_21_entry:
        xchg (%rsp), %rdx
        push %rdi
        push %rax
        mov $$21, %rdi
        mov $$int_handler_control_protection_exception, %rax
        jmp interrupt_x_entry
    .global interrupt_32_entry
    interrupt_32_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$32, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_33_entry
    interrupt_33_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$33, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_34_entry
    interrupt_34_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$34, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_35_entry
    interrupt_35_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$35, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_36_entry
    interrupt_36_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$36, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_37_entry
    interrupt_37_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$37, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_38_entry
    interrupt_38_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$38, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_39_entry
    interrupt_39_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$39, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_40_entry
    interrupt_40_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$40, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_41_entry
    interrupt_41_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$41, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_42_entry
    interrupt_42_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$42, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_43_entry
    interrupt_43_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$43, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_44_entry
    interrupt_44_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$44, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_45_entry
    interrupt_45_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$45, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_46_entry
    interrupt_46_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$46, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_47_entry
    interrupt_47_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$47, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_48_entry
    interrupt_48_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$48, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_49_entry
    interrupt_49_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$49, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_50_entry
    interrupt_50_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$50, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_51_entry
    interrupt_51_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$51, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_52_entry
    interrupt_52_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$52, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_53_entry
    interrupt_53_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$53, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_54_entry
    interrupt_54_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$54, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_55_entry
    interrupt_55_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$55, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_56_entry
    interrupt_56_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$56, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_57_entry
    interrupt_57_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$57, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_58_entry
    interrupt_58_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$58, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_59_entry
    interrupt_59_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$59, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_60_entry
    interrupt_60_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$60, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_61_entry
    interrupt_61_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$61, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_62_entry
    interrupt_62_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$62, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_63_entry
    interrupt_63_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$63, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_64_entry
    interrupt_64_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$64, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_65_entry
    interrupt_65_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$65, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_66_entry
    interrupt_66_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$66, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_67_entry
    interrupt_67_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$67, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_68_entry
    interrupt_68_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$68, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_69_entry
    interrupt_69_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$69, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_70_entry
    interrupt_70_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$70, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_71_entry
    interrupt_71_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$71, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_72_entry
    interrupt_72_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$72, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_73_entry
    interrupt_73_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$73, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_74_entry
    interrupt_74_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$74, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_75_entry
    interrupt_75_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$75, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_76_entry
    interrupt_76_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$76, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_77_entry
    interrupt_77_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$77, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_78_entry
    interrupt_78_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$78, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_79_entry
    interrupt_79_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$79, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_80_entry
    interrupt_80_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$80, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_81_entry
    interrupt_81_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$81, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_82_entry
    interrupt_82_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$82, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_83_entry
    interrupt_83_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$83, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_84_entry
    interrupt_84_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$84, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_85_entry
    interrupt_85_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$85, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_86_entry
    interrupt_86_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$86, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_87_entry
    interrupt_87_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$87, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_88_entry
    interrupt_88_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$88, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_89_entry
    interrupt_89_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$89, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_90_entry
    interrupt_90_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$90, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_91_entry
    interrupt_91_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$91, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_92_entry
    interrupt_92_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$92, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_93_entry
    interrupt_93_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$93, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_94_entry
    interrupt_94_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$94, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_95_entry
    interrupt_95_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$95, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_96_entry
    interrupt_96_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$96, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_97_entry
    interrupt_97_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$97, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_98_entry
    interrupt_98_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$98, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_99_entry
    interrupt_99_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$99, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_100_entry
    interrupt_100_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$100, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_101_entry
    interrupt_101_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$101, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_102_entry
    interrupt_102_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$102, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_103_entry
    interrupt_103_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$103, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_104_entry
    interrupt_104_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$104, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_105_entry
    interrupt_105_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$105, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_106_entry
    interrupt_106_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$106, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_107_entry
    interrupt_107_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$107, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_108_entry
    interrupt_108_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$108, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_109_entry
    interrupt_109_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$109, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_110_entry
    interrupt_110_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$110, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_111_entry
    interrupt_111_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$111, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_112_entry
    interrupt_112_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$112, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_113_entry
    interrupt_113_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$113, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_114_entry
    interrupt_114_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$114, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_115_entry
    interrupt_115_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$115, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_116_entry
    interrupt_116_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$116, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_117_entry
    interrupt_117_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$117, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_118_entry
    interrupt_118_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$118, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_119_entry
    interrupt_119_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$119, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_120_entry
    interrupt_120_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$120, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_121_entry
    interrupt_121_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$121, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_122_entry
    interrupt_122_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$122, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_123_entry
    interrupt_123_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$123, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_124_entry
    interrupt_124_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$124, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_125_entry
    interrupt_125_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$125, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_126_entry
    interrupt_126_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$126, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_127_entry
    interrupt_127_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$127, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_128_entry
    interrupt_128_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$128, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_129_entry
    interrupt_129_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$129, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_130_entry
    interrupt_130_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$130, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_131_entry
    interrupt_131_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$131, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_132_entry
    interrupt_132_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$132, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_133_entry
    interrupt_133_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$133, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_134_entry
    interrupt_134_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$134, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_135_entry
    interrupt_135_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$135, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_136_entry
    interrupt_136_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$136, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_137_entry
    interrupt_137_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$137, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_138_entry
    interrupt_138_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$138, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_139_entry
    interrupt_139_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$139, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_140_entry
    interrupt_140_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$140, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_141_entry
    interrupt_141_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$141, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_142_entry
    interrupt_142_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$142, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_143_entry
    interrupt_143_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$143, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_144_entry
    interrupt_144_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$144, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_145_entry
    interrupt_145_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$145, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_146_entry
    interrupt_146_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$146, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_147_entry
    interrupt_147_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$147, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_148_entry
    interrupt_148_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$148, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_149_entry
    interrupt_149_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$149, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_150_entry
    interrupt_150_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$150, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_151_entry
    interrupt_151_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$151, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_152_entry
    interrupt_152_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$152, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_153_entry
    interrupt_153_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$153, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_154_entry
    interrupt_154_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$154, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_155_entry
    interrupt_155_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$155, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_156_entry
    interrupt_156_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$156, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_157_entry
    interrupt_157_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$157, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_158_entry
    interrupt_158_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$158, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_159_entry
    interrupt_159_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$159, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_160_entry
    interrupt_160_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$160, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_161_entry
    interrupt_161_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$161, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_162_entry
    interrupt_162_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$162, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_163_entry
    interrupt_163_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$163, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_164_entry
    interrupt_164_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$164, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_165_entry
    interrupt_165_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$165, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_166_entry
    interrupt_166_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$166, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_167_entry
    interrupt_167_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$167, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_168_entry
    interrupt_168_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$168, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_169_entry
    interrupt_169_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$169, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_170_entry
    interrupt_170_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$170, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_171_entry
    interrupt_171_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$171, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_172_entry
    interrupt_172_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$172, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_173_entry
    interrupt_173_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$173, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_174_entry
    interrupt_174_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$174, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_175_entry
    interrupt_175_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$175, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_176_entry
    interrupt_176_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$176, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_177_entry
    interrupt_177_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$177, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_178_entry
    interrupt_178_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$178, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_179_entry
    interrupt_179_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$179, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_180_entry
    interrupt_180_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$180, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_181_entry
    interrupt_181_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$181, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_182_entry
    interrupt_182_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$182, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_183_entry
    interrupt_183_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$183, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_184_entry
    interrupt_184_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$184, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_185_entry
    interrupt_185_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$185, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_186_entry
    interrupt_186_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$186, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_187_entry
    interrupt_187_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$187, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_188_entry
    interrupt_188_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$188, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_189_entry
    interrupt_189_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$189, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_190_entry
    interrupt_190_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$190, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_191_entry
    interrupt_191_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$191, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_192_entry
    interrupt_192_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$192, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_193_entry
    interrupt_193_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$193, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_194_entry
    interrupt_194_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$194, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_195_entry
    interrupt_195_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$195, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_196_entry
    interrupt_196_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$196, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_197_entry
    interrupt_197_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$197, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_198_entry
    interrupt_198_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$198, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_199_entry
    interrupt_199_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$199, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_200_entry
    interrupt_200_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$200, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_201_entry
    interrupt_201_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$201, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_202_entry
    interrupt_202_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$202, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_203_entry
    interrupt_203_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$203, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_204_entry
    interrupt_204_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$204, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_205_entry
    interrupt_205_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$205, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_206_entry
    interrupt_206_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$206, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_207_entry
    interrupt_207_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$207, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_208_entry
    interrupt_208_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$208, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_209_entry
    interrupt_209_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$209, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_210_entry
    interrupt_210_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$210, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_211_entry
    interrupt_211_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$211, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_212_entry
    interrupt_212_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$212, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_213_entry
    interrupt_213_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$213, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_214_entry
    interrupt_214_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$214, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_215_entry
    interrupt_215_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$215, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_216_entry
    interrupt_216_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$216, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_217_entry
    interrupt_217_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$217, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_218_entry
    interrupt_218_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$218, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_219_entry
    interrupt_219_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$219, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_220_entry
    interrupt_220_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$220, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_221_entry
    interrupt_221_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$221, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_222_entry
    interrupt_222_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$222, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_223_entry
    interrupt_223_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$223, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_224_entry
    interrupt_224_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$224, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_225_entry
    interrupt_225_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$225, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_226_entry
    interrupt_226_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$226, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_227_entry
    interrupt_227_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$227, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_228_entry
    interrupt_228_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$228, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_229_entry
    interrupt_229_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$229, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_230_entry
    interrupt_230_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$230, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_231_entry
    interrupt_231_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$231, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_232_entry
    interrupt_232_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$232, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_233_entry
    interrupt_233_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$233, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_234_entry
    interrupt_234_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$234, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_235_entry
    interrupt_235_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$235, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_236_entry
    interrupt_236_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$236, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_237_entry
    interrupt_237_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$237, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_238_entry
    interrupt_238_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$238, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_239_entry
    interrupt_239_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$239, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_240_entry
    interrupt_240_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$240, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_241_entry
    interrupt_241_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$241, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_242_entry
    interrupt_242_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$242, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_243_entry
    interrupt_243_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$243, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_244_entry
    interrupt_244_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$244, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_245_entry
    interrupt_245_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$245, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_246_entry
    interrupt_246_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$246, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_247_entry
    interrupt_247_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$247, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_248_entry
    interrupt_248_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$248, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_249_entry
    interrupt_249_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$249, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_250_entry
    interrupt_250_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$250, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_251_entry
    interrupt_251_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$251, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_252_entry
    interrupt_252_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$252, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_253_entry
    interrupt_253_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$253, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_254_entry
    interrupt_254_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$254, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    .global interrupt_255_entry
    interrupt_255_entry:
        push %rdx
        push %rdi
        push %rax
        mov $$255, %rdi
        mov $$int_handler_any, %rax
        jmp interrupt_x_entry
    interrupt_x_entry:
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
    interrupt_entry_unswapped_gs_prefix_end:

        call *%rax
        cli

        cmpq $$8, 0x50(%rsp)
        je 1f
        swapgs
    interrupt_entry_unswapped_gs_postfix_start:
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
    interrupt_entry_unswapped_gs_postfix_end:


    interrupt_nmi_entry:
        push %rcx
        push %rsi
        push %r8
        push %r9
        push %r10
        push %r11

        cmpq $$8, 0x50(%rsp)
        jne 1f
        cmpq $$interrupt_entry_unswapped_gs_prefix_start, 0x48(%rsp)
        jb 2f
        cmpq $$interrupt_entry_unswapped_gs_prefix_end, 0x48(%rsp)
        jb 1f
        cmpq $$interrupt_entry_unswapped_gs_postfix_start, 0x48(%rsp)
        jb 2f
        cmpq $$interrupt_entry_unswapped_gs_postfix_end, 0x48(%rsp)
        jnb 2f

    1:
        swapgs
        movb $$1, 0x53(%rsp)
    2:

        mov 0x50(%rsp), %rsi
        call *%rax

        testb $$1, 0x53(%rsp)
        movb $$0, 0x53(%rsp)
        jz 1f
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

extern "C" {
    pub fn interrupt_0_entry();
    pub fn interrupt_1_entry();
    pub fn interrupt_2_entry();
    pub fn interrupt_3_entry();
    pub fn interrupt_4_entry();
    pub fn interrupt_5_entry();
    pub fn interrupt_6_entry();
    pub fn interrupt_7_entry();
    pub fn interrupt_8_entry();
    pub fn interrupt_9_entry();
    pub fn interrupt_10_entry();
    pub fn interrupt_11_entry();
    pub fn interrupt_12_entry();
    pub fn interrupt_13_entry();
    pub fn interrupt_14_entry();
    pub fn interrupt_16_entry();
    pub fn interrupt_17_entry();
    pub fn interrupt_18_entry();
    pub fn interrupt_19_entry();
    pub fn interrupt_20_entry();
    pub fn interrupt_21_entry();
    pub fn interrupt_32_entry();
    pub fn interrupt_33_entry();
    pub fn interrupt_34_entry();
    pub fn interrupt_35_entry();
    pub fn interrupt_36_entry();
    pub fn interrupt_37_entry();
    pub fn interrupt_38_entry();
    pub fn interrupt_39_entry();
    pub fn interrupt_40_entry();
    pub fn interrupt_41_entry();
    pub fn interrupt_42_entry();
    pub fn interrupt_43_entry();
    pub fn interrupt_44_entry();
    pub fn interrupt_45_entry();
    pub fn interrupt_46_entry();
    pub fn interrupt_47_entry();
    pub fn interrupt_48_entry();
    pub fn interrupt_49_entry();
    pub fn interrupt_50_entry();
    pub fn interrupt_51_entry();
    pub fn interrupt_52_entry();
    pub fn interrupt_53_entry();
    pub fn interrupt_54_entry();
    pub fn interrupt_55_entry();
    pub fn interrupt_56_entry();
    pub fn interrupt_57_entry();
    pub fn interrupt_58_entry();
    pub fn interrupt_59_entry();
    pub fn interrupt_60_entry();
    pub fn interrupt_61_entry();
    pub fn interrupt_62_entry();
    pub fn interrupt_63_entry();
    pub fn interrupt_64_entry();
    pub fn interrupt_65_entry();
    pub fn interrupt_66_entry();
    pub fn interrupt_67_entry();
    pub fn interrupt_68_entry();
    pub fn interrupt_69_entry();
    pub fn interrupt_70_entry();
    pub fn interrupt_71_entry();
    pub fn interrupt_72_entry();
    pub fn interrupt_73_entry();
    pub fn interrupt_74_entry();
    pub fn interrupt_75_entry();
    pub fn interrupt_76_entry();
    pub fn interrupt_77_entry();
    pub fn interrupt_78_entry();
    pub fn interrupt_79_entry();
    pub fn interrupt_80_entry();
    pub fn interrupt_81_entry();
    pub fn interrupt_82_entry();
    pub fn interrupt_83_entry();
    pub fn interrupt_84_entry();
    pub fn interrupt_85_entry();
    pub fn interrupt_86_entry();
    pub fn interrupt_87_entry();
    pub fn interrupt_88_entry();
    pub fn interrupt_89_entry();
    pub fn interrupt_90_entry();
    pub fn interrupt_91_entry();
    pub fn interrupt_92_entry();
    pub fn interrupt_93_entry();
    pub fn interrupt_94_entry();
    pub fn interrupt_95_entry();
    pub fn interrupt_96_entry();
    pub fn interrupt_97_entry();
    pub fn interrupt_98_entry();
    pub fn interrupt_99_entry();
    pub fn interrupt_100_entry();
    pub fn interrupt_101_entry();
    pub fn interrupt_102_entry();
    pub fn interrupt_103_entry();
    pub fn interrupt_104_entry();
    pub fn interrupt_105_entry();
    pub fn interrupt_106_entry();
    pub fn interrupt_107_entry();
    pub fn interrupt_108_entry();
    pub fn interrupt_109_entry();
    pub fn interrupt_110_entry();
    pub fn interrupt_111_entry();
    pub fn interrupt_112_entry();
    pub fn interrupt_113_entry();
    pub fn interrupt_114_entry();
    pub fn interrupt_115_entry();
    pub fn interrupt_116_entry();
    pub fn interrupt_117_entry();
    pub fn interrupt_118_entry();
    pub fn interrupt_119_entry();
    pub fn interrupt_120_entry();
    pub fn interrupt_121_entry();
    pub fn interrupt_122_entry();
    pub fn interrupt_123_entry();
    pub fn interrupt_124_entry();
    pub fn interrupt_125_entry();
    pub fn interrupt_126_entry();
    pub fn interrupt_127_entry();
    pub fn interrupt_128_entry();
    pub fn interrupt_129_entry();
    pub fn interrupt_130_entry();
    pub fn interrupt_131_entry();
    pub fn interrupt_132_entry();
    pub fn interrupt_133_entry();
    pub fn interrupt_134_entry();
    pub fn interrupt_135_entry();
    pub fn interrupt_136_entry();
    pub fn interrupt_137_entry();
    pub fn interrupt_138_entry();
    pub fn interrupt_139_entry();
    pub fn interrupt_140_entry();
    pub fn interrupt_141_entry();
    pub fn interrupt_142_entry();
    pub fn interrupt_143_entry();
    pub fn interrupt_144_entry();
    pub fn interrupt_145_entry();
    pub fn interrupt_146_entry();
    pub fn interrupt_147_entry();
    pub fn interrupt_148_entry();
    pub fn interrupt_149_entry();
    pub fn interrupt_150_entry();
    pub fn interrupt_151_entry();
    pub fn interrupt_152_entry();
    pub fn interrupt_153_entry();
    pub fn interrupt_154_entry();
    pub fn interrupt_155_entry();
    pub fn interrupt_156_entry();
    pub fn interrupt_157_entry();
    pub fn interrupt_158_entry();
    pub fn interrupt_159_entry();
    pub fn interrupt_160_entry();
    pub fn interrupt_161_entry();
    pub fn interrupt_162_entry();
    pub fn interrupt_163_entry();
    pub fn interrupt_164_entry();
    pub fn interrupt_165_entry();
    pub fn interrupt_166_entry();
    pub fn interrupt_167_entry();
    pub fn interrupt_168_entry();
    pub fn interrupt_169_entry();
    pub fn interrupt_170_entry();
    pub fn interrupt_171_entry();
    pub fn interrupt_172_entry();
    pub fn interrupt_173_entry();
    pub fn interrupt_174_entry();
    pub fn interrupt_175_entry();
    pub fn interrupt_176_entry();
    pub fn interrupt_177_entry();
    pub fn interrupt_178_entry();
    pub fn interrupt_179_entry();
    pub fn interrupt_180_entry();
    pub fn interrupt_181_entry();
    pub fn interrupt_182_entry();
    pub fn interrupt_183_entry();
    pub fn interrupt_184_entry();
    pub fn interrupt_185_entry();
    pub fn interrupt_186_entry();
    pub fn interrupt_187_entry();
    pub fn interrupt_188_entry();
    pub fn interrupt_189_entry();
    pub fn interrupt_190_entry();
    pub fn interrupt_191_entry();
    pub fn interrupt_192_entry();
    pub fn interrupt_193_entry();
    pub fn interrupt_194_entry();
    pub fn interrupt_195_entry();
    pub fn interrupt_196_entry();
    pub fn interrupt_197_entry();
    pub fn interrupt_198_entry();
    pub fn interrupt_199_entry();
    pub fn interrupt_200_entry();
    pub fn interrupt_201_entry();
    pub fn interrupt_202_entry();
    pub fn interrupt_203_entry();
    pub fn interrupt_204_entry();
    pub fn interrupt_205_entry();
    pub fn interrupt_206_entry();
    pub fn interrupt_207_entry();
    pub fn interrupt_208_entry();
    pub fn interrupt_209_entry();
    pub fn interrupt_210_entry();
    pub fn interrupt_211_entry();
    pub fn interrupt_212_entry();
    pub fn interrupt_213_entry();
    pub fn interrupt_214_entry();
    pub fn interrupt_215_entry();
    pub fn interrupt_216_entry();
    pub fn interrupt_217_entry();
    pub fn interrupt_218_entry();
    pub fn interrupt_219_entry();
    pub fn interrupt_220_entry();
    pub fn interrupt_221_entry();
    pub fn interrupt_222_entry();
    pub fn interrupt_223_entry();
    pub fn interrupt_224_entry();
    pub fn interrupt_225_entry();
    pub fn interrupt_226_entry();
    pub fn interrupt_227_entry();
    pub fn interrupt_228_entry();
    pub fn interrupt_229_entry();
    pub fn interrupt_230_entry();
    pub fn interrupt_231_entry();
    pub fn interrupt_232_entry();
    pub fn interrupt_233_entry();
    pub fn interrupt_234_entry();
    pub fn interrupt_235_entry();
    pub fn interrupt_236_entry();
    pub fn interrupt_237_entry();
    pub fn interrupt_238_entry();
    pub fn interrupt_239_entry();
    pub fn interrupt_240_entry();
    pub fn interrupt_241_entry();
    pub fn interrupt_242_entry();
    pub fn interrupt_243_entry();
    pub fn interrupt_244_entry();
    pub fn interrupt_245_entry();
    pub fn interrupt_246_entry();
    pub fn interrupt_247_entry();
    pub fn interrupt_248_entry();
    pub fn interrupt_249_entry();
    pub fn interrupt_250_entry();
    pub fn interrupt_251_entry();
    pub fn interrupt_252_entry();
    pub fn interrupt_253_entry();
    pub fn interrupt_254_entry();
    pub fn interrupt_255_entry();
}
