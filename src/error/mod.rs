// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Twig generic error

use std::fmt::{self, Display};
use std::error;


#[macro_use]
pub mod macros;
// use std Error-trait to improve cross-crate compatibility
// don't mix it up with Err(X)
pub mod api;

// generic wrapper around some ErrorCode - adds location support
#[derive(Debug)]
pub struct Error<T>
    where T: api::ErrorCode
{
    // the exception codes are going to be enums
    // - i.e. Exception<MY_ENUM> implements std::error::Error without any boilerplate
    // to MY_ENUM. Hurray! :-)
    code: T,
    // I decided to call this field `code` instead of `error` to not confuse it with the Error trait
    location: Location,
    // chaining is required by std::error::Error
    cause: Option<Box<error::Error>>,
}

impl<T> Error<T>
    where T: api::ErrorCode
{
    pub fn new(code: T, location: Location) -> Error<T> {
        Error {
            code: code,
            location: location,
            cause: None
        }
    }

    #[allow(dead_code)] // only used by tests
    pub fn code(&self) -> &T {
        &self.code
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn caused_by<X: 'static + error::Error>(mut self, cause: X) -> Self {
        self.cause = Some(Box::new(cause));

        self
    }

    pub fn causes<X>(self, wrapper: Error<X>) -> Error<X> where
        X: api::ErrorCode
    {
        wrapper.caused_by(self)
    }

    // iterate along the error-chain.
    pub fn iter(&self) -> ErrorIter {
        ErrorIter {
            next: Some(self),
        }
    }
}

impl<T> error::Error for Error<T>
    where T: api::ErrorCode
{
    fn description(&self) -> &str {
        // delegate the error description to the ErrorCode
        &self.code.description()
    }

    fn cause<'a>(&'a self) -> Option<&'a error::Error> {
        // dereference from Option<Box<T>> to Option<&T>
        self.cause.as_ref().map(|x| &**x)
    }
}

pub struct ErrorIter<'a> {
    next: Option<&'a error::Error>
}

impl<'a> Iterator for ErrorIter<'a> {
    type Item = &'a error::Error;

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

impl<T> Display for Error<T>
    where T: api::ErrorCode
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(write!(f, "{error_code} at {location}\n",
            error_code = self.code,
            location = self.location));

        match self.cause {
            None => Ok(()),
            Some(ref cause) => write!(f, " - caused by: {}", cause),
        }
    }
}

#[derive(Debug)]
pub struct Location {
    // this might be a bit redundant - but we just store everything we can get.
    // we don't need to be super performant on exceptions - because we try to avoid them :-)
    //
    // note that the module_path is currently only displayed in Debug output due to this redundancy
    pub module_path : &'static str, // e.g. twig::lexer::job::state::shared_traits
    pub filename : &'static str,    // e.g. /src/lexer/job/state/shared_traits.rs
    pub line : u32,
    pub column : u32,
}

impl Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{filename}:{line}:{column}",
            filename = self.filename,
            line     = self.line,
            column   = self.column)
    }
}
