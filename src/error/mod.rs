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

#[macro_use]
pub mod macros;

// TODO Read more about error handling in rust

#[allow(dead_code)]
#[derive(Debug)]
pub struct Error<T>
    /* TODO where T: Syntax, ... - restrict to real codes */ {
    code: T,
    details: Details,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Details {
    pub message : &'static str,
    pub module_path : &'static str,
    pub filename : &'static str,
    pub line : u32,
    pub column : u32,
}

#[allow(dead_code)]
impl<T> Error<T> {
    pub fn new(code: T, details: Details) -> Error<T> {
        Error {
            code : code,
            details : details,
        }
    }
}

impl ToString for Details {
    fn to_string(&self) -> String {
        format!("{}::{}:{}:{}",
            self.module_path,
            self.filename,
            self.line,
            self.column)
    }
}
