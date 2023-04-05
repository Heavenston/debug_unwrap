#![doc = include_str!("../README.md")]
use std::fmt::Debug;

/// Extension trait for the Result and Option types, see crate-level
/// documentation for more information.
pub trait DebugUnwrap: Sized {
    type Output;

    /// The same as [Option::unwrap](Option::unwrap) on Options or
    /// [Result::unwrap](Result::unwrap) on Results but only exists in Debug mode.
    ///
    /// See crate-level documentation for more information.
    #[cfg(debug_assertions)]
    fn debug_unwrap(self) -> Self::Output;

    /// Alias for [debug_unwrap](DebugUnwrap::debug_unwrap).
    /// Enabled by the feature `out` (enabled by default).
    ///
    /// See crate-level documentation for more information.
    #[cfg(debug_assertions)]
    #[cfg(feature="out")]
    #[cfg_attr(docsrs, doc(cfg(feature = "out")))]
    fn out(self) -> Self::Output {
        self.debug_unwrap()
    }

    /// Alias for [debug_unwrap](DebugUnwrap::debug_unwrap).
    /// Enabled by the feature `o`.
    ///
    /// See crate-level documentation for more information.
    #[cfg(debug_assertions)]
    #[cfg(feature="o")]
    #[cfg_attr(docsrs, doc(cfg(feature = "o")))]
    fn o(self) -> Self::Output {
        self.debug_unwrap()
    }

    /// Alias for [debug_unwrap](DebugUnwrap::debug_unwrap).
    /// Enabled by the feature `peel`.
    ///
    /// See crate-level documentation for more information.
    #[cfg(debug_assertions)]
    #[cfg(feature="peel")]
    #[cfg_attr(docsrs, doc(cfg(feature = "peel")))]
    fn peel(self) -> Self::Output {
        self.debug_unwrap()
    }
}

impl<T: Sized> DebugUnwrap for Option<T> {
    type Output = T;

    #[cfg(debug_assertions)]
    fn debug_unwrap(self) -> T {
        self.unwrap()
    }
}

impl<T: Sized, E: Debug> DebugUnwrap for Result<T, E> {
    type Output = T;

    #[cfg(debug_assertions)]
    fn debug_unwrap(self) -> T {
        self.unwrap()
    }
}
