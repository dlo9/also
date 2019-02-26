#![warn(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(test, deny(warnings))]

//! An approximation of Kotlin's chaining functions like [let](https://kotlinlang.org/api/latest/jvm/stdlib/kotlin/let.html) and [also](https://kotlinlang.org/api/latest/jvm/stdlib/kotlin/also.html)

/// Provides Kotlin-esque helper functions for all types via a blanket impl, enabling easier function chaining patterns
pub trait With {
    /// Calls a function with the receiver, and returns the result.
    /// Akin to Kotlin's [let](https://kotlinlang.org/api/latest/jvm/stdlib/kotlin/let.html)
    /// extension function
    /// # Examples
    /// ```
    /// # use with::*;
    /// let x = "Hello, world!".lets(|x| x.len());
    /// assert_eq!(13, x);
    /// ```
    #[inline(always)]
    fn lets<R>(mut self, f: impl FnOnce(&mut Self) -> R) -> R
    where
        Self: Sized,
    {
        f(&mut self)
    }

    /// Calls a function with the receiver, and returns the receiver.
    /// Akin to Kotlin's [also](https://kotlinlang.org/api/latest/jvm/stdlib/kotlin/also.html)
    /// extension function
    /// # Examples
    /// ```
    /// # use with::*;
    /// let x = "Hello".to_string().also(|x| x.push_str(", world!"));
    /// assert_eq!("Hello, world!", x);
    /// ```
    #[inline(always)]
    fn also(mut self, f: impl FnOnce(&mut Self)) -> Self
    where
        Self: Sized,
    {
        f(&mut self);
        self
    }
}

impl<T> With for T {}
