#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(feature = "nightly", feature = "std"), feature(try_trait))]
#![warn(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(test, deny(warnings))]

//! <style>
//! /* Provides formatting for the feature list */
//! .desc {
//!     padding-left: 1em;
//!     margin-bottom: 1em;
//!     display: block;
//! }
//! .icon {
//!   width: 1em;
//!   height: 1em;
//!   display: inline-flex;
//!   top: .125em;
//!   position: relative;
//! }
//! .check-mark {
//!   background: url("data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='3.8 5 16.8 16.8' width='100%' height='100%'><path d='M9 16.2l-3.5-3.5c-.39-.39-1.01-.39-1.4 0-.39.39-.39 1.01 0 1.4l4.19 4.19c.39.39 1.02.39 1.41 0L20.3 7.7c.39-.39.39-1.01 0-1.4-.39-.39-1.01-.39-1.4 0L9 16.2' fill='green' /></svg>") no-repeat;
//! }
//! .cross-mark {
//!   background: url("data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='3.5 3.5 16.8 16.8' width='100%' height='100%'><path d='M18.3 5.71c-.39-.39-1.02-.39-1.41 0L12 10.59 7.11 5.7c-.39-.39-1.02-.39-1.41 0-.39.39-.39 1.02 0 1.41L10.59 12 5.7 16.89c-.39.39-.39 1.02 0 1.41.39.39 1.02.39 1.41 0L12 13.41l4.89 4.89c.39.39 1.02.39 1.41 0 .39-.39.39-1.02 0-1.41L13.41 12l4.89-4.89c.38-.38.38-1.02 0-1.4z' fill='red' /></svg>");
//! }
//! </style>
//!
//! An approximation of Kotlin's chaining functions like [let](https://kotlinlang.org/api/latest/jvm/stdlib/kotlin/let.html) and [also](https://kotlinlang.org/api/latest/jvm/stdlib/kotlin/also.html)
//! # Features
//! - nightly
#![cfg_attr(feature = "nightly", doc = r#"<i class="check-mark icon" /></i>"#)]
#![cfg_attr(not(feature = "nightly"), doc = r#"<i class="cross-mark icon" /></i>"#)]
//! <span class="desc">Additional features requiring the nightly compiler</span>
//! - std
#![cfg_attr(feature = "std", doc = r#"<i class="check-mark icon" /></i>"#)]
#![cfg_attr(not(feature = "std"), doc = r#"<i class="cross-mark icon" /></i>"#)]
//! <span class="desc">Additional features requiring the Rust Standard Library</span>

#[cfg(all(feature = "nightly", feature = "std"))]
use std::ops::Try;

/// Provides Kotlin-esque helper functions for all types via a blanket impl, enabling easier function chaining patterns
/// ```
/// # use also::*;
/// let not_empty = ().lets(|_| "Hello, world!");
/// assert_eq!("Hello, world!", not_empty);
/// ```
pub trait Also {
    /// Calls a function with the receiver, and returns the result.
    /// Akin to Kotlin's [let](https://kotlinlang.org/api/latest/jvm/stdlib/kotlin/let.html)
    /// extension function
    /// # Examples
    /// ```
    /// # use also::*;
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
    /// # use also::*;
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
    /// # use also::*;
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
    /// # use also::*;
    /// let x: Result<String, ()> = Ok("Hello".to_string()).and_run(|s| s.push('!'));
    /// assert_eq!(Ok("Hello!".to_string()), x);
    /// ```
    #[inline(always)]
    #[cfg(all(feature = "nightly", feature = "std"))]
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
    /// # use also::*;
    /// let x: Result<(), String> = Err("Hello".to_string()).or_run(|s| s.push('!'));
    /// assert_eq!(Err("Hello!".to_string()), x);
    /// ```
    #[inline(always)]
    #[cfg(all(feature = "nightly", feature = "std"))]
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

impl<T> Also for T {}
