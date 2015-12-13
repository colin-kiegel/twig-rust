// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Twig Error Handling
//!
//! # Examples
//!
//! ```rust,macro_test
//! #[macro_use] extern crate twig; // Macros must be imported *explicitly*
//!
//! use std::fmt;
//! use twig::api::error::Traced;
//! use std::error::Error;
//!
//! // Implement a custom error code.
//! type MyTracedSimpleError = Traced<MySimpleError>;
//!
//! #[derive(Debug)]
//! enum MySimpleError {
//!     Critical,
//!     Recoverable
//! }
//!
//! impl Error for MySimpleError {
//!     fn description(&self) -> &str {
//!         match *self {
//!             MySimpleError::Critical => "Critical error.",
//!             MySimpleError::Recoverable => "Recoverable error."
//!         }
//!     }
//! }
//!
//! impl fmt::Display for MySimpleError {
//!     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//!         write!(f, "{} With human readable details", self.description())
//!     }
//! }
//!
//! fn main() {
//!     // Create a twig error, wrapping this error code + code location.
//!     let result: Result<(), Traced<MySimpleError>> = traced_err!(MySimpleError::Critical);
//!
//!     if let Err(error) = result {
//!         assert!(error.to_string().starts_with("Critical error. With human readable details at <anon>:"));
//!     }
//! }
//! ```

#[macro_use] mod macros;
use std::fmt::{self, Debug, Display};
use std::error::Error; // use std Error-trait to improve cross-crate compatibility
use std::ops::{Deref, DerefMut};

/// Extension trait for std::error::Error
///
/// adds conversion to `Traced` error
pub trait ErrorExt: Error {
    /// Returns generic twig error for this error code.
    /// You must provide the location, where the error occured.
    fn at(self, location: Location) -> Traced<Self>
        where Self: Sized
    {
        Traced::new(self, location)
    }
}

impl<T> ErrorExt for T where T: Error {}

/// Record current state of complex objects
///
/// The purpose of a dump is to be wrapped in error codes instead of complex objects.
/// This abstraction allows to
/// * keep error codes free of any lifetimes
/// * maintain the ability of receiver of error codes to decide about verbosity
///
/// The Dump::Data type may not contain lifetimes.
/// In practice this means cloning all referenced data into the dump.
///
/// For a type `X: Dump` you can reference the associated dump type via `<X as Dump>::Data`.
pub trait Dump {
    type Data: Debug + Display + 'static;

    fn dump(&self) -> Self::Data;
}

/// Generic error with backtrace.
///
/// Wrapper around some error type `T` implementing `std::error::Error`.
/// * Adds support for a backtrace.
/// * Automatically derefs to the inner error.
pub struct Traced<T>
    where T: Error
{
    error: T,
    trace: Trace,
}

impl<T> Traced<T> where
    T: Error
{
    /// Create a new twig error out of some generic error code.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate twig;
    /// # fn main() {
    /// use twig::api::error;
    /// use std::env::VarError;
    ///
    /// error::Traced::new(VarError::NotPresent, loc!()); // shorthand: `trace!(VarError::NotPresent)`
    /// # }
    /// ```
    pub fn new(error: T, location: Location) -> Traced<T> {
        Traced {
            error: error,
            trace: Trace(vec![location]),
        }
    }

    /// Return the first location the error occured.
    pub fn error(&self) -> &T {
        &self.error
    }

    /// Return the first location the error occured.
    pub fn location(&self) -> Option<&Location> {
        self.trace.first()
    }

    pub fn backtrace(&self) -> &[Location] {
        &self.trace
    }

    // should not name it `at`, because `Traced` derefs to `Error` which impl `ErrorExt::at()`
    pub fn trace<R>(self, loc: Location) -> Traced<R>
        where T: Into<R>,
              R: Error
    {
        let Traced { error, mut trace } = self;
        trace.push(loc);

        Traced {
            error: error.into(),
            trace: trace
        }
    }

    /// Creates an iterator to iterate along the error cause-chain.
    pub fn iter(&self) -> ErrorIter {
        ErrorIter {
            next: Some(&self.error),
        }
    }
}

impl<T> Deref for Traced<T> where
    T: Error
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.error
    }
}

/// Iterator to iterate along the error cause-chain.
pub struct ErrorIter<'a> {
    next: Option<&'a Error>
}

impl<'a> Iterator for ErrorIter<'a> {
    type Item = &'a Error;

    fn next(&mut self) -> Option<Self::Item> {
        return match self.next {
            Some(err) => {
                self.next = err.cause();
                Some(err)
            }
            None => None,
        }
    }
}

impl<T> Debug for Traced<T> where
    T: Error
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("Traced")
            .field("error", &self.error)
            .field("at", &self.trace)
            .finish()
    }
}

impl<T> Display for Traced<T> where
    T: Error
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{error} at {trace}\n",
            error = self.error,
            trace = &self.trace)
    }
}

/// Trace wraps `Vec<Location>`  with specialized `Display` and `Debug` impls.
///
/// For everything else it just derefs to `Vec<Location>`.
pub struct Trace(Vec<Location>);

impl Deref for Trace {
    type Target = Vec<Location>;

    fn deref(&self) -> &Self::Target {
        let Trace(ref trace) = *self;

        trace
    }
}

impl DerefMut for Trace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let Trace(ref mut trace) = *self;

        trace
    }
}

impl Debug for Trace {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.iter().enumerate().fold(
            &mut f.debug_struct("Trace"),
            |s, i| s.field(&format!("#{}", i.0), i.1)
        ).finish()
    }
}

impl Display for Trace {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.first() {
            Some(location) => write!(f, "{:?}", location),
            None =>  write!(f, "unknown (no backtrace)")
        }
    }
}

/// Location in rust source code
///
/// Debug::fmt() output is formatted in a compact way: `"{filename}:{line}:{column}"`.
#[derive(PartialEq)]
pub struct Location {
    pub filename : &'static str,    // e.g. /src/lexer/job/state/shared_traits.rs
    pub line : u32,
    pub column : u32,
}

impl Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{filename}:{line}:{column}",
            filename = self.filename,
            line     = self.line,
            column   = self.column)
    }
}
