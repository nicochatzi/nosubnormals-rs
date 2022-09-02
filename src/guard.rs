/// RAII guard against subnormal values
/// within a given scope.
///
use crate::fpcr;

pub struct SubnormalGuard(u32);

impl Default for SubnormalGuard {
    #[inline]
    fn default() -> Self {
        Self(fpcr::reg::read())
    }
}

impl SubnormalGuard {
    /// Add the flags to flush subnormal values to zero
    #[inline]
    pub fn without_subnormal(mut self) -> Self {
        self.0 |= fpcr::FLUSH_SUBNORMAL_TO_ZERO_MASK;
        self
    }

    /// Add the flags to set the IEEE-754 floating-point rounding mode
    #[inline]
    pub fn with_rounding(mut self, rounding: fpcr::RoundingMode) -> Self {
        self.0 |= rounding.mask();
        self
    }

    /// Set the configured bits to the floating-point control register
    #[inline]
    pub fn set(self) -> Self {
        fpcr::reg::write(self.0);
        self
    }
}

impl Drop for SubnormalGuard {
    #[inline]
    fn drop(&mut self) {
        fpcr::reg::write(self.0);
    }
}

///
#[macro_export]
macro_rules! guard {
    () => {
        let _scoped_subnormal_removal_guard =
            $crate::SubnormalGuard::default().without_subnormal().set();
    };

    ($flags: expr) => {
        let _scoped_subnormal_removal_guard = {
            use $crate::fpcr::RoundingMode;
            $crate::SubnormalGuard::default()
                .without_subnormal()
                .with_rounding($flags)
                .set()
        };
    };
}

#[cfg(test)]
mod tests {
    use crate::{assert_not_subnormal, assert_subnormal, fpcr::tests::{Float, SUBNORMALS}};

    #[test]
    fn can_reach_subnormals_without_the_guard() {
        for f in SUBNORMALS {
            assert!(f.is_subnormal());
            match f {
                Float::F32(f) => assert_not_subnormal!(f),
                Float::F64(f) => assert_not_subnormal!(f),
            }
        }
    }

    #[test]
    fn cannot_reach_subnormals_with_guard() {
        crate::guard! {}

        for f in SUBNORMALS {
            assert!(f.is_subnormal());
            match f {
                Float::F32(f) => assert_subnormal!(f),
                Float::F64(f) => assert_subnormal!(f),
            }
        }
    }

    #[test]
    fn can_set_rounding_mode_through_guard() {
        crate::guard! { RoundingMode::ToZero }

        for f in SUBNORMALS {
            assert!(f.is_subnormal());
            match f {
                Float::F32(f) => assert_not_subnormal!(f),
                Float::F64(f) => assert_not_subnormal!(f),
            }
        }
    }
}
