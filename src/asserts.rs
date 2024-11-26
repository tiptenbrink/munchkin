#[cfg(all(not(test), not(feature = "debug-checks")))]
pub const munchkin_assert_LEVEL_DEFINITION: u8 = munchkin_assert_SIMPLE;

#[cfg(any(test, feature = "debug-checks"))]
pub const munchkin_assert_LEVEL_DEFINITION: u8 = munchkin_assert_EXTREME;

pub const munchkin_assert_SIMPLE: u8 = 1;
pub const munchkin_assert_MODERATE: u8 = 2;
pub const munchkin_assert_ADVANCED: u8 = 3;
pub const munchkin_assert_EXTREME: u8 = 4;

#[macro_export]
#[doc(hidden)]
macro_rules! print_munchkin_assert_warning_message {
    () => {
        if munchkin::asserts::munchkin_assert_LEVEL_DEFINITION >= munchkin::asserts::munchkin_assert_MODERATE {
            warn!("Potential performance degradation: the Pumpkin assert level is set to {}, meaning many debug asserts are active which may result in performance degradation.", munchkin::asserts::munchkin_assert_LEVEL_DEFINITION);
        };
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! munchkin_assert_simple {
    ($($arg:tt)*) => {
        if $crate::asserts::munchkin_assert_LEVEL_DEFINITION >= $crate::asserts::munchkin_assert_SIMPLE {
            assert!($($arg)*);
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! munchkin_assert_eq_simple {
    ($($arg:tt)*) => {
        if $crate::asserts::munchkin_assert_LEVEL_DEFINITION >= $crate::asserts::munchkin_assert_SIMPLE {
            assert_eq!($($arg)*);
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! munchkin_assert_ne_simple {
    ($($arg:tt)*) => {
        if $crate::asserts::munchkin_assert_LEVEL_DEFINITION >= $crate::asserts::munchkin_assert_SIMPLE {
            assert_ne!($($arg)*);
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! munchkin_assert_moderate {
    ($($arg:tt)*) => {
        if $crate::asserts::munchkin_assert_LEVEL_DEFINITION >= $crate::asserts::munchkin_assert_MODERATE {
            assert!($($arg)*);
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! munchkin_assert_ne_moderate {
    ($($arg:tt)*) => {
        if $crate::asserts::munchkin_assert_LEVEL_DEFINITION >= $crate::asserts::munchkin_assert_MODERATE {
            assert_ne!($($arg)*);
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! munchkin_assert_advanced {
    ($($arg:tt)*) => {
        if $crate::asserts::munchkin_assert_LEVEL_DEFINITION >= $crate::asserts::munchkin_assert_ADVANCED {
            assert!($($arg)*);
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! munchkin_assert_extreme {
    ($($arg:tt)*) => {
        if $crate::asserts::munchkin_assert_LEVEL_DEFINITION >= $crate::asserts::munchkin_assert_EXTREME {
            assert!($($arg)*);
        }
    };
}
