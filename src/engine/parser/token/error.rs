// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Typisation of syntax errors.

use std::fmt::{self, Display};
use error::Error;
use error::ErrorCode;
use engine::parser::token;

pub type TokenError = Error<TokenErrorCode>;

#[derive(Debug)]
pub enum TokenErrorCode {
    _NoValue,
    UnexpectedTokenAtItem {
        reason: Option<&'static str>,
        expected: token::PatternDump,
        found: token::stream::Item,
    }
}

impl ErrorCode for TokenErrorCode {
    fn description(&self) -> &str {
        match *self {
            TokenErrorCode::_NoValue => "No value.",
            TokenErrorCode::UnexpectedTokenAtItem{..} => "Unexpected token.",
        }
    }
}

impl Display for TokenErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.description()));

        match *self {
            TokenErrorCode::_NoValue => Ok(()),
            TokenErrorCode::UnexpectedTokenAtItem {
                reason, ref expected, ref found
            } => {
                try!(write!(f, " Expected token matching {x:?} but found item {t:?} at {p:?}",
                    x = expected, t = found.token(), p = found.position()));

                if let Some(reason) = reason {
                    try!(write!(f, " {}", reason))
                }

                Ok(())
            }
        }
    }
}
