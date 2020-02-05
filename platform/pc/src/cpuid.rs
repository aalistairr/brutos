use brutos_util_macros::{bitfield, ConvertInner};

pub fn cpuid(leaf: u32, subleaf: u32) -> [u32; 4] {
    unsafe {
        let eax: u32;
        let ebx: u32;
        let ecx: u32;
        let edx: u32;
        asm!("cpuid" : "={eax}" (eax), "={ebx}" (ebx), "={ecx}" (ecx), "={edx}" (edx) : "{eax}" (leaf), "{ecx}" (subleaf) : "memory" : "volatile");
        [eax, ebx, ecx, edx]
    }
}

pub mod leaf {
    macro_rules! leaf {
        ($name:ident: $t:ty = $leaf:expr => $subleaf:expr) => {
            pub enum $name {}

            impl $name {
                pub fn get() -> $t {
                    <$t>::from(super::cpuid($leaf, $subleaf))
                }
            }
        };
        ($name:ident: $t:ty = $leaf:expr) => {
            leaf!($name: $t = $leaf => 0);
        };
    }

    leaf!(CoreCrystalClock: super::CoreCrystalClock = 0x15);
    leaf!(InvariantTsc: super::InvariantTsc = 0x80000007);
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Debug, ConvertInner)]
    pub struct CoreCrystalClock([u32; 4]);

    #[ro] field tsc_ratio_den: usize => 0[0..32];
    #[ro] field tsc_ratio_num: usize => 1[0..32];
    #[ro] field ccc_freq_raw: usize => 2[0..32];
}

impl CoreCrystalClock {
    pub fn ccc_freq(&self) -> Option<usize> {
        let freq = self.ccc_freq_raw();
        if freq > 0 {
            Some(freq)
        } else {
            None
        }
    }

    pub fn tsc_freq(&self) -> Option<usize> {
        self.ccc_freq()
            .map(|ccc_freq| ccc_freq * self.tsc_ratio_num() / self.tsc_ratio_den())
    }
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Debug, ConvertInner)]
    pub struct InvariantTsc([u32; 4]);

    #[ro] pub field available: bool => 3[8];
}
