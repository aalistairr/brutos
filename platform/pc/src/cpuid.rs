use brutos_util_macros::bitfield;

pub struct Cpuid {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

pub fn cpuid(leaf: u32, subleaf: u32) -> Cpuid {
    unsafe {
        let eax: u32;
        let ebx: u32;
        let ecx: u32;
        let edx: u32;
        asm!("cpuid" : "={eax}" (eax), "={ebx}" (ebx), "={ecx}" (ecx), "={edx}" (edx) : "{eax}" (leaf), "{ecx}" (subleaf) : "memory" : "volatile");
        Cpuid { eax, ebx, ecx, edx }
    }
}

macro_rules! leaf_value {
    ($name:ident { $($fields:tt)* }) => {
        bitfield! {
            #[derive(Copy, Clone, PartialEq, Eq, Debug)]
            pub struct $name {
                eax: u32,
                ebx: u32,
                ecx: u32,
                edx: u32,
            }

            $($fields)*
        }

        impl From<Cpuid> for $name {
            fn from(cpuid: Cpuid) -> $name {
                $name {
                    eax: cpuid.eax,
                    ebx: cpuid.ebx,
                    ecx: cpuid.ecx,
                    edx: cpuid.edx,
                }
            }
        }
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

leaf_value!(CoreCrystalClock {
    #[ro] field tsc_ratio_den: usize = eax[0..32];
    #[ro] field tsc_ratio_num: usize = ebx[0..32];
    #[ro] field ccc_freq_raw: usize = ecx[0..32];
});

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

leaf_value!(InvariantTsc {
    #[ro] pub field available: bool = edx[8];
});
