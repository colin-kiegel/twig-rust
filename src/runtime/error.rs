// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Typisation of runtime errors.

use std::fmt::{self, Display};
use std::error::Error;

#[derive(Debug)]
pub enum RuntimeError {
    Unreachable {
        reason: String,
    },
}

impl Error for RuntimeError {
    fn description(&self) -> &str {
        match *self {
            RuntimeError::Unreachable{..} => {
                "Unexptected runtime error (please report as bug with details)."
            }
        }
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.description()));

        match *self {
            RuntimeError::Unreachable {
                ref reason
            } => write!(f, " {}.", reason),
        }
    }
}
