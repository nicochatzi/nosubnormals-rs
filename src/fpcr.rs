//! Floating-Point Status Register controller
//!
//! flags:
//! x86_64 : https://help.totalview.io/previous_releases/2019/html/index.html#page/Reference_Guide/Intelx86MXSCRRegister.html
//! aarch64 : https://developer.arm.com/documentation/ddi0595/2021-06/AArch64-Registers/FPCR--Floating-point-Control-Register?lang=en

#[cfg(target_arch = "x86_64")]
pub(crate) mod reg {
    pub mod flags {
        pub const RN: u32 = 0b00 << 13;
        pub const RP: u32 = 0b10 << 13;
        pub const RM: u32 = 0b01 << 13;
        pub const RZ: u32 = 0b11 << 13;
        pub const FTZ: u32 = 1 << 15;
        pub const DAZ: u32 = 1 << 6;
        pub const AUX: u32 = 0;
    }
    #[inline]
    pub fn read() -> u32 {
        unsafe { core::arch::x86_64::_mm_getcsr() }
    }

    #[inline]
    pub fn write(val: u32) {
        unsafe { core::arch::x86_64::_mm_setcsr(val) };
    }
}

#[cfg(target_arch = "aarch64")]
pub(crate) mod reg {
    pub mod flags {
        pub const RN: u32 = 0b00 << 22;
        pub const RP: u32 = 0b01 << 22;
        pub const RM: u32 = 0b10 << 22;
        pub const RZ: u32 = 0b11 << 22;
        pub const FTZ: u32 = 1 << 0;
        pub const DAZ: u32 = (1 << 24) | (1 << 19); // f32/64 | f16
        pub const AUX: u32 = 1 << 1; // Alternate Handling
    }

    #[inline]
    pub fn read() -> u32 {
        let mut v: u32;
        unsafe { core::arch::asm!("mrs {:x}, fpcr", out(reg) v, options(nomem, nostack)) };
        v
    }

    #[inline]
    pub fn write(val: u32) {
        unsafe { core::arch::asm!("msr fpcr, {:x}", in(reg) val, options(nomem, nostack)) }
    }
}

pub(crate) const FLUSH_SUBNORMAL_TO_ZERO_MASK: u32 =
    reg::flags::FTZ | reg::flags::DAZ | reg::flags::AUX;

#[inline]
pub fn disable_subnormal() {
    reg::write(reg::read() | FLUSH_SUBNORMAL_TO_ZERO_MASK);
}

#[inline]
pub fn enable_subnormal() {
    reg::write(reg::read() & !FLUSH_SUBNORMAL_TO_ZERO_MASK);
}

pub enum RoundingMode {
    Nearest,
    PlusInf,
    MinusInf,
    ToZero,
}

impl RoundingMode {
    #[inline]
    pub fn set(self) {
        reg::write(reg::read() | self.mask());
    }

    #[inline]
    pub(crate) const fn mask(self) -> u32 {
        match self {
            Self::Nearest => reg::flags::RN,
            Self::PlusInf => reg::flags::RP,
            Self::MinusInf => reg::flags::RM,
            Self::ToZero => reg::flags::RZ,
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub enum Float {
        F32(f32),
        F64(f64),
    }

    pub const SUBNORMALS: [Float; 4] = [
        Float::F32(1.0e-40),
        Float::F32(-1.0e-40),
        Float::F64(1.0e-308),
        Float::F64(-1.0e-308),
    ];

    impl Float {
        pub fn is_subnormal(&self) -> bool {
            match self {
                Self::F32(f) => f.to_bits() & 0x7f800000 == 0,
                Self::F64(f) => f.to_bits() & 0x7ff0000000000000 == 0,
            }
        }
    }

    #[test]
    fn can_disable_and_enable_subnormal() {
        for f in SUBNORMALS {
            assert!(f.is_subnormal());

            disable_subnormal();

            match f {
                Float::F32(f) => assert_eq!(f, 0.),
                Float::F64(f) => assert_eq!(f, 0.),
            }

            enable_subnormal();

            match f {
                Float::F32(f) => assert_ne!(f, 0.),
                Float::F64(f) => assert_ne!(f, 0.),
            }
        }
    }
}
