// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Typisation of runtime errors.

use std::fmt::{self, Display};
use error::Error;
use error::api::ErrorCode;

pub type RuntimeError = Error<RuntimeErrorCode>;

#[derive(Debug)]
pub enum RuntimeErrorCode {
    Unreachable {
        reason: String
    }
}

impl ErrorCode for RuntimeErrorCode {
    fn description(&self) -> &str {
        match *self {
            RuntimeErrorCode::Unreachable{..} => "Unexptected runtime error (please report as bug with details).",
        }
    }
}

impl Display for RuntimeErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.description()));

        match *self {
            RuntimeErrorCode::Unreachable {
                ref reason
            } => {
                write!(f, " {}.", reason)
            }
        }
    }
}
