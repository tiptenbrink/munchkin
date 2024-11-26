#[cfg(all(not(test), not(feature = "debug-checks")))]
pub(crate) const ASSERT_LEVEL_DEFINITION: u8 = ASSERT_SIMPLE;

#[cfg(any(test, feature = "debug-checks"))]
pub(crate) const ASSERT_LEVEL_DEFINITION: u8 = ASSERT_EXTREME;

pub(crate) const ASSERT_SIMPLE: u8 = 1;
pub(crate) const ASSERT_MODERATE: u8 = 2;
pub(crate) const ASSERT_ADVANCED: u8 = 3;
pub(crate) const ASSERT_EXTREME: u8 = 4;

#[macro_export]
#[doc(hidden)]
macro_rules! munchkin_assert_simple {
    ($($arg:tt)*) => {
        if $crate::asserts::ASSERT_LEVEL_DEFINITION >= $crate::asserts::ASSERT_SIMPLE {
            assert!($($arg)*);
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! munchkin_assert_eq_simple {
    ($($arg:tt)*) => {
        if $crate::asserts::ASSERT_LEVEL_DEFINITION >= $crate::asserts::ASSERT_SIMPLE {
            assert_eq!($($arg)*);
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! munchkin_assert_ne_simple {
    ($($arg:tt)*) => {
        if $crate::asserts::ASSERT_LEVEL_DEFINITION >= $crate::asserts::ASSERT_SIMPLE {
            assert_ne!($($arg)*);
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! munchkin_assert_moderate {
    ($($arg:tt)*) => {
        if $crate::asserts::ASSERT_LEVEL_DEFINITION >= $crate::asserts::ASSERT_MODERATE {
            assert!($($arg)*);
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! munchkin_assert_ne_moderate {
    ($($arg:tt)*) => {
        if $crate::asserts::ASSERT_LEVEL_DEFINITION >= $crate::asserts::ASSERT_MODERATE {
            assert_ne!($($arg)*);
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! munchkin_assert_advanced {
    ($($arg:tt)*) => {
        if $crate::asserts::ASSERT_LEVEL_DEFINITION >= $crate::asserts::ASSERT_ADVANCED {
            assert!($($arg)*);
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! munchkin_assert_extreme {
    ($($arg:tt)*) => {
        if $crate::asserts::ASSERT_LEVEL_DEFINITION >= $crate::asserts::ASSERT_EXTREME {
            assert!($($arg)*);
        }
    };
}
