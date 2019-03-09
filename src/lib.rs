#![feature(try_trait)]
#![warn(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(test, deny(warnings))]

//! An approximation of Kotlin's chaining functions like [let](https://kotlinlang.org/api/latest/jvm/stdlib/kotlin/let.html) and [also](https://kotlinlang.org/api/latest/jvm/stdlib/kotlin/also.html)

use std::ops::Try;

/// Provides Kotlin-esque helper functions for all types via a blanket impl, enabling easier function chaining patterns
/// ```
/// # use with::*;
/// let not_empty = ().lets(|_| "Hello, world!");
/// assert_eq!("Hello, world!", not_empty);
/// ```
pub trait With {
    /// Calls a function with the receiver, and returns the result.
    /// Akin to Kotlin's [let](https://kotlinlang.org/api/latest/jvm/stdlib/kotlin/let.html)
    /// extension function
    /// # Examples
    /// ```
    /// # use with::*;
    /// let x = "Hello, world!".lets(|x| x.len());
    /// assert_eq!(13, x);
    ///
    /// let x = "Hello, world!".to_string().lets(|mut x| {
    ///     x.push('g');
    ///     x.len()
    /// });
    /// assert_eq!(14, x);
    /// ```
    #[inline(always)]
    fn lets<R>(self, f: impl FnOnce(Self) -> R) -> R
    where
        Self: Sized,
    {
        f(self)
    }

    /// Returns the receiver if the given function returns `Ok`, else forwards the `Err`.
    /// Akin to Kotlin's [takeIf](https://kotlinlang.org/api/latest/jvm/stdlib/kotlin/take-if.html)
    /// extension function
    /// # Examples
    /// ```
    /// # use with::*;
    /// let x = "42".take_if(|x| u8::from_str_radix(x, 10));
    /// assert_eq!(Ok("42"), x);
    ///
    /// let x = "aa".take_if(|x| u8::from_str_radix(x, 10));
    /// assert!(x.is_err());
    /// ```
    #[inline(always)]
    fn take_if<R, E>(mut self, f: impl FnOnce(&mut Self) -> Result<R, E>) -> Result<Self, E>
    where
        Self: Sized,
    {
        f(&mut self).map(|_| self)
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
    fn also<R>(mut self, f: impl FnOnce(&mut Self) -> R) -> Self
    where
        Self: Sized,
    {
        f(&mut self);
        self
    }

    /// Calls a function with the `Ok` contained value and returns the `Result`.
    /// # Examples
    /// ```
    /// # use with::*;
    /// let x: Result<String, ()> = Ok("Hello".to_string()).and_run(|s| s.push('!'));
    /// assert_eq!(Ok("Hello!".to_string()), x);
    /// ```
    #[inline(always)]
    fn and_run<R, E, T>(self, f: impl FnOnce(&mut R) -> T) -> Self
    where
        Self: Try<Ok = R, Error = E> + Sized,
    {
        match Try::into_result(self) {
            Ok(mut r) => {
                f(&mut r);
                Try::from_ok(r)
            }
            Err(e) => Try::from_error(e),
        }
    }

    /// Calls a function with the `Err` contained value and returns the `Result`.
    /// # Examples
    /// ```
    /// # use with::*;
    /// let x: Result<(), String> = Err("Hello".to_string()).or_run(|s| s.push('!'));
    /// assert_eq!(Err("Hello!".to_string()), x);
    /// ```
    #[inline(always)]
    fn or_run<R, E, T>(self, f: impl FnOnce(&mut E) -> T) -> Self
    where
        Self: Try<Ok = R, Error = E> + Sized,
    {
        match Try::into_result(self) {
            Ok(r) => Try::from_ok(r),
            Err(mut e) => {
                f(&mut e);
                Try::from_error(e)
            }
        }
    }
}

impl<T> With for T {}
