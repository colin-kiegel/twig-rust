// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Twig macros for error handling

/// Creates a traced error for the location from which it was invoked.
///
/// For internal use within the twig library and extensions.
///
/// * argument type `T` must implement `std::error::Error`
/// * expanded expression has type `Result<_,Traced<T: Error>>`
///
/// # Examples
///
/// ```rust,macro_test
/// # #[macro_use] extern crate twig;
/// # fn main() {
/// use twig::api::error::Traced;
/// use std::env::VarError;
///
/// let result: Result<(), Traced<VarError>> = traced_err!(VarError::NotPresent);
/// if let Err(error) = result {
///     assert_eq!(error.to_string(), "environment variable not found at <anon>:6:43\n");
/// }
/// # }
/// ```
#[macro_export]
macro_rules! traced_err {
    ( $error:expr ) => ({
        Err($crate::api::error::Traced::new($error, loc!()))
    });
}
/// Expands to the location from which it was invoked.
///
/// For internal use within the twig library and extensions.
///
/// * expanded expression has type `twig::api::error::Location`
/// * the returned location is not the invocation of the `loc!()` macro itself,
///   but rather the first macro invocation leading up to the invocation of the `loc!()` macro.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate twig;
/// # fn main() {
/// use twig::api::error;
/// use twig::api::error::Location;
///
/// let here: Location = loc!();
/// println!("called from: {:?}", here);
/// # }
/// ```
#[macro_export]
macro_rules! loc {
    () => ({
        $crate::api::error::Location {
            filename : file!(),
            line : line!(),
            column : column!(),
        }
    });
}

/// Equivalent to `try!` for traced errors.
///
/// For internal use within the twig library and extensions.
///
/// `try_traced!` pushes the current code location to the backtrace of `Traced<T: Error>`.
/// In Twig library and extensions this (or something equivalent) should be used everywhere,
/// where a function is supposed to return early in case of some traced error.
///
/// If the internal error type `T` implements `Into<NewError>`, then `try_traced!` will do implicit
/// conversions according to the expected result type of the current function (similar to `try!`).
///
/// # Pseudo-Code
///
/// ```ignore
/// fn foo() -> Result<(), Error<CODE_A>> {
///    let x: Result<(), Error<CODE_B>> = ...;
///
///    // `CODE_B` must implement `twig::error::GeneralizeTo<CODE_A>`
///    try_traced!(x); // try! would fail here, due to incompatible error types
/// }
/// ```
/// # Examples
///
/// ```rust,macro_test
/// #[macro_use] extern crate twig;
///
/// use twig::api::error::{Traced, ErrorExt, Location};
/// use std::env::VarError;
///
/// fn foo() -> Result<(), Traced<VarError>> {
///     let traced = VarError::NotPresent.at(loc!());
///
///     // The trace contains one code location.
///     assert_eq!(traced.backtrace().len(), 1);
///
///     try_traced!(Err(traced))
/// }
///
///
/// fn main() {
///   let traced = foo().unwrap_err();
///   assert_eq!(traced.backtrace().len(), 2);
/// }
/// ```
#[macro_export]
macro_rules! try_traced {
    ( $result:expr ) => ({
        use $crate::api::error::Traced;

        match $result {
            Ok(value) => value,
            Err(traced) => return Err(Traced::trace(traced, loc!())),
        }
    })
}

#[macro_export]
macro_rules! try_untraced {
    ( $result:expr ) => ({
        match $result {
            Ok(value) => value,
            Err(untraced) => return traced_err!(untraced),
        }
    })
}
