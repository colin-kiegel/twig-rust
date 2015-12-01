// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Twig macros for error handling

/// A macro which creates a error for the location from which it was invoked.
/// For internal use within the twig library.
///
/// * argument must implement `twig::error::ErrorCode`
/// * expanded expression has type `Result<_,twig::error::Error<T>>`
///
/// # Examples
///
/// ```rust,macro_test
/// # #[macro_use] extern crate twig;
/// # fn main() {
/// use twig::error::Error;
///
/// // `twig::error::ErrorCode` is implemented for `&'static str`
/// let result: Result<(), Error<&'static str>> = err!("critical error");
/// if let Err(error) = result {
///     assert_eq!(error.to_string(), "critical error at <anon>:6:46\n");
/// }
/// # }
/// ```
#[macro_export]
macro_rules! err {
    ( $code:expr ) => ({
        Err($crate::error::Error::new($code, loc!()))
    });
}
/// A macro which expands to the location from which it was invoked.
/// For internal use within the twig library.
///
/// * expanded expression has type `twig::error::Location`
/// * the returned location is not the invocation of the `loc!()` macro itself,
///   but rather the first macro invocation leading up to the invocation of the `loc!()` macro.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate twig;
/// # fn main() {
/// use twig::error;
///
/// let this_location = loc!();
/// println!("called from: {}", this_location);
/// # }
/// ```
#[macro_export]
macro_rules! loc {
    () => ({
        $crate::error::Location {
            module_path : module_path!(),
            filename : file!(),
            line : line!(),
            column : column!(),
        }
    });
}

/// A macro which will create an error-chain with location for each chaining-operation.
/// For internal use within the twig library.
///
/// `try_chain!` is supposed to be used, whenever errors cross a logic boundary
///
/// # Pseudo-Code
///
/// ```ignore
/// fn foo() -> Result<(), Error<CODE_A>> {
///    let x: Result<(), Error<CODE_B>> = ...;
///
///    // `CODE_B` must implement `twig::error::GeneralizeTo<CODE_A>`
///    try_chain!(x); // try! would fail here, due to incompatible error types
/// }
/// ```
#[macro_export]
macro_rules! try_chain {
    ( $result:expr ) => (
        match $result {
            Ok(value) => value,
            Err(cause) => {
                let code = $crate::error::GeneralizeTo::generalize(cause.code());

                return Err($crate::error::Error::new(code, loc!())
                    .caused_by(cause)
                    .into())
            }
        }
    )
}
