// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Twig Error API for error codes

use error::{Error, Location};
use std::fmt::{Debug, Display};
use std::any::Any;

/// Base functionality for error codes
/// - similar to std::error::Error, but without error-chaining!
///
///
pub trait ErrorCode: Debug + Display + Any {
    /// A short description of the error code.
    ///
    /// The description should not contain newlines or sentence-ending
    /// punctuation, to facilitate embedding in larger user-facing
    /// strings.
    fn description(&self) -> &str;

    /// Provide the location, where the error occured.
    fn at(self, location: Location) -> Error<Self> where
        Self: Sized
    {
        Error::new(self, location)
    }
}

// will be used to transform error codes!
// can't use Into-trait - because we only have references
pub trait GeneralizeTo<T> {
    fn generalize(&self) -> T;
}

// implement this trait for complex objects like lexer/parser Jobs
// to be able to embedd their most important data in ErrorCodes via `X::dump()`
pub trait Dump {
    type Data: Debug + Display;

    fn dump(&self) -> Self::Data;
}
