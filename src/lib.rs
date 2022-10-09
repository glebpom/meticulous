//! This crate provides extensions to [`Result`] type with additional
//! unwrapping methods, which have more meaning compared to standard
//! [`Result::expect`] and [`Result::unwrap`].
//!
//! See [`ResultExt`] doc for more info
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
    #[cfg(all(feature = "disallow-todo-on-release", debug_assertions = "false"))]
    fn todo(self) -> T {
        compile_error!(".todo() disallowed with debug_assertions turned off")
    }

    #[cfg(not(all(feature = "disallow-todo-on-release", debug_assertions = "false")))]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_todo() {
        u16::try_from(i32::MAX).todo();
    }

    #[test]
    #[should_panic]
    fn test_assured() {
        u16::try_from(i32::MAX).assured("always ok on linux");
    }

    #[test]
    #[should_panic]
    fn test_verified() {
        u16::try_from(i32::MAX).verified("boundaries already checked");
    }
}
