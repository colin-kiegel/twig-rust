/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Twig base error.
 *
 * This error class and its children must only be used when
 * an error occurs during the loading of a template, when a syntax error
 * is detected in a template, or when rendering a template. Other
 * errors must use regular error classes (like when the template
 * cache directory is not writable for instance).
 *
 * To help debugging template issues, this class tracks the original template
 * name and line where the error occurred.
 *
 * Whenever possible, you must set these information (original template name
 * and line number) yourself by passing them to the constructor. If some or all
 * these information are not available from where you throw the exception, then
 * this class will guess them automatically (when the line number is set to -1
 * and/or the filename is set to null). As this is a costly operation, this
 * can be disabled by passing false for both the filename and the line number
 * when creating a new instance of this class.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use std::fmt;
use std::error::Error as ErrorTrait;

/////////////
// exports //
/////////////

#[macro_use]
pub mod macros;

// TODO Read more about error handling in rust
//http://doc.rust-lang.org/std/error/index.html

#[allow(dead_code)]
#[derive(Debug)]
pub struct Error<T> {
    code: T,
    details: Details,
    description: String,
    cause: Option<Box<ErrorTrait>>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Details {
    pub message : &'static str, // TODO make this Option<&str> - requires additional macro logic
    pub module_path : &'static str,
    pub filename : &'static str,
    pub line : u32,
    pub column : u32,
}

#[allow(dead_code)]
impl<T> Error<T>
    where T: fmt::Debug {
    pub fn new(details: Details, code: T) -> Error<T> {
        let description = format!("[{code:?}]: {details}",
            code = code,
            details = details.to_string());

        Error {
            code : code,
            details : details,
            description: description,
            cause: None,
        }
    }

    pub fn chain(&mut self, cause: Box<ErrorTrait>) {
        self.cause = Some(cause)
    }
}

// error: use of unstable library feature 'core': requires RFC and more experience
//     where T: ::std::marker::Reflect + fmt::Debug {
//              ^~~~~~~~~~~~~~~~~~~~~~
// impl<T> ErrorTrait for Error<T>
//     where T: ::std::marker::Reflect + fmt::Debug {
//     fn description(&self) -> &str {
//         self.description.as_ref()
//     }
//
//     fn cause<'a>(&'a self) -> Option<&'a ErrorTrait> {
//         use std::borrow::Borrow;
//         // TODO is there a simpler way to go from Option<Box<T>> to Option<&T>? Ask this on SO...
//         match self.cause {
//             Some(ref cause) => Some(cause.borrow()),
//             None            => None
//         }
//     }
// }

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.description)
    }
}

impl ToString for Details {
    fn to_string(&self) -> String {
        format!("{message} in {path}/{filename}:{line}:{column}",
            message  = self.message,
            path     = self.module_path,
            filename = self.filename,
            line     = self.line,
            column   = self.column)
    }
}
