use std::fmt::Debug;

pub trait DebugUnwrap: Sized {
    type Output;

    #[cfg(debug_assertions)]
    fn debug_unwrap(self) -> Self::Output;

    #[cfg(debug_assertions)]
    #[cfg(feature="out")]
    fn out(self) -> Self::Output {
        self.debug_unwrap()
    }
    #[cfg(debug_assertions)]
    #[cfg(feature="o")]
    fn o(self) -> Self::Output {
        self.debug_unwrap()
    }
    #[cfg(debug_assertions)]
    #[cfg(feature="peel")]
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
