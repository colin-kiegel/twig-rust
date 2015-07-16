/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Twig base exception.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use std::fmt;
use std::any::Any;

/////////////
// exports //
/////////////

#[macro_use]
pub mod macros;
pub use std::error::Error;

// TODO Read more about error handling in rust
//http://doc.rust-lang.org/std/error/index.html

pub struct Exception<T> {
    code: T,
    details: Details,
    description: String,
    cause: Option<Box<Error>>,
}

#[derive(Debug)]
pub struct Details {
    pub message : Option<String>,
    pub module_path : &'static str, // e.g. twig::lexer::job::state::shared_traits
    pub filename : &'static str,    // e.g. /src/lexer/job/state/shared_traits.rs
    pub line : u32,
    pub column : u32,
}

// TODO read more about Any trait
impl<T> Exception<T>
    where T: Any + fmt::Debug {
    pub fn new(details: Details, code: T) -> Exception<T> {
        let description = Self::description_string(&code, &details);

        Exception {
            code : code,
            details : details,
            description: description,
            cause: None,
        }
    }

    fn description_string(code: &T, details: &Details) -> String {
        format!("[{code:?}]: {details}",
            code = code,
            details = details.to_string())
    }

    #[allow(dead_code)] // only used by tests
    pub fn code(&self) -> &T {
        &self.code
    }

    pub fn details(&self) -> &Details {
        &self.details
    }

    pub fn explain(mut self, message: String) -> Self {
        self.details.message = Some(message);
        self.description = Self::description_string(&self.code, &self.details);

        self
    }

    pub fn caused_by<X: 'static + Error>(mut self, cause: X) -> Self {
        self.cause = Some(Box::new(cause));

        self
    }

    pub fn causes<X>(self, wrapper: Exception<X>) -> Exception<X> where
        X: Any + fmt::Debug
    {
        wrapper.caused_by(self)
    }

    pub fn iter(&self) -> ErrorIter {
        ErrorIter {
            next: Some(self),
        }
    }
}

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

impl<T, V> ::std::convert::Into<Result<V, Exception<T>>> for Exception<T> {
    fn into (self) -> Result<V, Exception<T>> {
        Err(self)
    }
}
impl<T> Error for Exception<T>
    where T: Any + fmt::Debug
{
    fn description(&self) -> &str {
        self.description.as_ref()
    }

    fn cause<'a>(&'a self) -> Option<&'a Error> {
        use std::borrow::Borrow;
        // TODO is there a simpler way to go from Option<Box<T>> to Option<&T>? Ask this on SO...
        match self.cause {
            Some(ref cause) => Some(cause.borrow()),
            None            => None
        }
    }
}

impl<T> fmt::Display for Exception<T>
    where T: Any + fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.description())
    }
}

impl<T> fmt::Debug for Exception<T>
    where T: Any + fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let recursive_desc = self.iter().map(|e| e.description())
             .collect::<Vec<&str>>().connect(" caused by\n - ");
        write!(f, "\n - {}\n", recursive_desc)
    }
}

impl ToString for Details {
    fn to_string(&self) -> String {

        format!("{message}{in_}{filename}:{line}:{column}",
            message  = match self.message {
                    Some(ref msg) => msg.as_ref(),
                    None => "",
                },
            in_      = if self.message.is_some() { " in " } else { "" },
            filename = self.filename,
            line     = self.line,
            column   = self.column)
    }
}
