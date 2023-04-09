//! Adds the method debug_unwrap for when you just want to make it compile.
//! Not to be confused with the <https://crates.io/crates/debug_unwraps> crate.
//! 
//! This library adds the [DebugUnwrap](DebugUnwrap) extension trait that adds
//! the method [debug_unwrap](DebugUnwrap::debug_unwrap) to the
//! [Option](std::option::Option) and [Result](std::result::Result) types.
//! It does exactly the same thing as the normal unwrap methods, but won't exist
//! when compiling without debug_assertions enabled
//! (i.e. when not compiling in Debug mode).
//! 
//! There is also three other aliases that all have there respective
//! library features of the same name to enable them:
//!  - out (enabled by default)
//!  - o
//!  - peel
//!
//! You can also use the `deprecate` feature which makes the functions deprecated
//! on release mode instead of flat out not existing, so that your code will
//! compile but with warnings.
use std::fmt::Debug;

/// Extension trait for the Result and Option types, see crate-level
/// documentation for more information.
pub trait DebugUnwrap: Sized {
    type Output;

    /// The same as [Option::unwrap](Option::unwrap) on Options or
    /// [Result::unwrap](Result::unwrap) on Results but only exists in Debug mode.
    ///
    /// See crate-level documentation for more information.
    #[cfg(any(debug_assertions, feature="deprecate"))]
    #[cfg_attr(
        all(not(debug_assertions), feature = "deprecate"),
        deprecated = "Debug unwrap must not be used in release mode"
    )]
    fn debug_unwrap(self) -> Self::Output;

    /// Alias for [debug_unwrap](DebugUnwrap::debug_unwrap).
    /// Enabled by the feature `out` (enabled by default).
    ///
    /// See crate-level documentation for more information.
    #[cfg(all(any(debug_assertions, feature="deprecate"), feature="out"))]
    #[cfg_attr(
        all(not(debug_assertions), feature = "deprecate"),
        deprecated = "Debug unwrap must not be used in release mode"
    )]
    fn out(self) -> Self::Output {
        #[allow(deprecated)]
        self.debug_unwrap()
    }

    /// Alias for [debug_unwrap](DebugUnwrap::debug_unwrap).
    /// Enabled by the feature `o`.
    ///
    /// See crate-level documentation for more information.
    #[cfg(all(any(debug_assertions, feature="deprecate"), feature="o"))]
    #[cfg_attr(
        all(not(debug_assertions), feature = "deprecate"),
        deprecated = "Debug unwrap must not be used in release mode"
    )]
    fn o(self) -> Self::Output {
        #[allow(deprecated)]
        self.debug_unwrap()
    }

    /// Alias for [debug_unwrap](DebugUnwrap::debug_unwrap).
    /// Enabled by the feature `peel`.
    ///
    /// See crate-level documentation for more information.
    #[cfg(all(any(debug_assertions, feature="deprecate"), feature="peel"))]
    #[cfg_attr(
        all(not(debug_assertions), feature = "deprecate"),
        deprecated = "Debug unwrap must not be used in release mode"
    )]
    fn peel(self) -> Self::Output {
        #[allow(deprecated)]
        self.debug_unwrap()
    }
}

impl<T: Sized> DebugUnwrap for Option<T> {
    type Output = T;

    #[cfg(any(debug_assertions, feature="deprecate"))]
    fn debug_unwrap(self) -> T {
        self.unwrap()
    }
}

impl<T: Sized, E: Debug> DebugUnwrap for Result<T, E> {
    type Output = T;

    #[cfg(any(debug_assertions, feature="deprecate"))]
    fn debug_unwrap(self) -> T {
        self.unwrap()
    }
}
