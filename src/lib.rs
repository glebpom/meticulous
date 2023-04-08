//! This crate provides extensions to [`Result`] and [`Option`] types with additional
//! unwrapping methods, which have more meaning compared to standard
//! [`Result::expect`]/[`Option::expect`] and [`Result::unwrap`]/[`Option::unwrap`].
//!
//! See [`ResultExt`] and [`OptionExt`] docs for more info
//!

use std::fmt::Debug;

/// The extension trait for `Result`
pub trait ResultExt<T, E>
where
    E: Debug,
{
    /// Returns the contained [`Ok`] value, consuming the `self` value.
    ///
    /// Use this method to indicate that it should be replaced with
    /// a better implementation later.
    ///
    /// If `disallow-todo-on-release` feature is used, then the compilation
    /// will fail if `debug_assertions` are turned off (typically on a release
    /// build).
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Err`], with message "not yet implemented",
    /// and the content of the [`Err`].
    ///
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```should_panic
    /// use meticulous::ResultExt;
    ///
    /// let x: Result<u32, &str> = Err("emergency failure");
    /// x.todo(); // panics with `not yet implemented: emergency failure`
    /// ```
    fn todo(self) -> T;

    /// Returns the contained [`Ok`] value, consuming the `self` value.
    ///
    /// Use this method to indicate that you don't expect the `Result`
    /// to be `Err` in any condition suitable for the program, e.g. `u64`
    /// will always convert to `usize` if you target only 64-bit arch.
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Err`], with message "the success was expected to be assured, but the error was returned",
    /// and the content of the [`Err`].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```should_panic
    /// use meticulous::ResultExt;
    ///
    /// let x: Result<u32, &str> = Err("emergency failure");
    /// x.assured("always true for 64-bit apps"); // panics with `the success was expected to be assured, but the error was returned: always true for 64-bit apps: emergency failure`
    /// ```
    fn assured(self, reason: &str) -> T;

    /// Returns the contained [`Ok`] value, consuming the `self` value.
    ///
    /// Use this method to indicate that conditions which may lead to `Err`
    /// result have already been checked in the code, so you never expect
    /// an error.
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Err`], with message "the success was expected to be verified in the code earlier, but the error was returned",
    /// and the content of the [`Err`].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```should_panic
    /// use meticulous::ResultExt;
    ///
    /// let x: Result<u32, &str> = Err("emergency failure");
    /// x.verified("boundaries already checked"); // panics with `the success was expected to be verified in the code earlier, but the error was returned: boundaries already checked: emergency failure`
    /// ```
    fn verified(self, reason: &str) -> T;
}

impl<T, E> ResultExt<T, E> for Result<T, E>
where
    E: Debug,
{
    #[inline]
    #[track_caller]
    #[cfg(not(all(feature = "disallow-todo-on-release", not(debug_assertions))))]
    fn todo(self) -> T {
        self.expect("not yet implemented")
    }

    #[inline]
    #[track_caller]
    fn assured(self, reason: &str) -> T {
        self.unwrap_or_else(|_| {
            panic!(
                "the success was expected to be assured, but the error was returned: {}",
                reason
            )
        })
    }

    #[inline]
    #[track_caller]
    fn verified(self, reason: &str) -> T {
        self.unwrap_or_else(|_| panic!("the success was expected to be verified in the code earlier, but the error was returned: {}", reason))
    }
}

/// The extension trait for `Option`
pub trait OptionExt<T> {
    /// Returns the contained [`Some`] value, consuming the `self` value.
    ///
    /// Use this method to indicate that it should be replaced with
    /// a better implementation later.
    ///
    /// If `disallow-todo-on-release` feature is used, then the compilation
    /// will fail if `debug_assertions` are turned off (typically on a release
    /// build).
    ///
    /// # Panics
    ///
    /// Panics if the value is [`None`], with message "not yet implemented".
    ///
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```should_panic
    /// use meticulous::OptionExt;
    ///
    /// let x: Option<u32> = None;
    /// x.todo(); // panics with `not yet implemented: None`
    /// ```
    fn todo(self) -> T;

    /// Returns the contained [`Some`] value, consuming the `self` value.
    ///
    /// Use this method to indicate that you don't expect the `Option`
    /// to be `None` in any condition suitable for the program.
    ///
    /// # Panics
    ///
    /// Panics if the value is [`None`], with message "the value was assured to exist but was None".
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```should_panic
    /// use meticulous::OptionExt;
    ///
    /// let x: Option<u32> = None;
    /// x.assured("always set"); // panics with `the value was assured to exist but was None: always set`
    /// ```
    fn assured(self, reason: &str) -> T;

    /// Returns the contained [`Some`] value, consuming the `self` value.
    ///
    /// Use this method to indicate that conditions which may lead to `None`
    /// result have already been checked in the code, so you always expect value to be present.
    ///
    /// # Panics
    ///
    /// Panics if the value is [`None`], with message "it was verified that value presents but None was returned",
    /// and the content of the [`Err`].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```should_panic
    /// use meticulous::OptionExt;
    ///
    /// let x: Option<u32> = None;
    /// x.verified("should always be Some"); // panics with `it was verified that value presents but None was returned: should always be Some`
    /// ```
    fn verified(self, reason: &str) -> T;
}

impl<T> OptionExt<T> for Option<T> {
    #[cfg(not(all(feature = "disallow-todo-on-release", not(debug_assertions))))]
    #[track_caller]
    fn todo(self) -> T {
        self.expect("not yet implemented")
    }

    #[inline]
    #[track_caller]
    fn assured(self, reason: &str) -> T {
        self.unwrap_or_else(|| panic!("the value was assured to exist but was None: {}", reason))
    }

    #[inline]
    #[track_caller]
    fn verified(self, reason: &str) -> T {
        self.unwrap_or_else(|| {
            panic!(
                "it was verified that value presents but None was returned: {}",
                reason
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_result_todo() {
        u16::try_from(i32::MAX).todo();
    }

    #[test]
    #[should_panic]
    fn test_result_assured() {
        u16::try_from(i32::MAX).assured("always ok on linux");
    }

    #[test]
    #[should_panic]
    fn test_result_verified() {
        u16::try_from(i32::MAX).verified("boundaries already checked");
    }

    #[test]
    #[should_panic]
    fn test_option_todo() {
        "hello".strip_prefix("a").todo();
    }

    #[test]
    #[should_panic]
    fn test_option_assured() {
        "hello"
            .strip_prefix("a")
            .assured("string always starts with a");
    }

    #[test]
    #[should_panic]
    fn test_option_verified() {
        "hello"
            .strip_prefix("a")
            .verified("string prefix was already checked");
    }
}
