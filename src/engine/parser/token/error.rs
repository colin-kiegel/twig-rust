// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Typisation of syntax errors.

use std::fmt::{self, Display};
use std::error::Error;
use engine::parser::token;

#[derive(Debug)]
pub enum TokenError {
    _NoValue,
    UnexpectedTokenAtItem {
        reason: Option<&'static str>,
        expected: token::PatternDump,
        found: token::stream::Item,
    },
}

impl Error for TokenError {
    fn description(&self) -> &str {
        match *self {
            TokenError::_NoValue => "No value.",
            TokenError::UnexpectedTokenAtItem{..} => "Unexpected token.",
        }
    }
}

impl Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.description()));

        match *self {
            TokenError::_NoValue => Ok(()),
            TokenError::UnexpectedTokenAtItem {
                reason, ref expected, ref found
            } => {
                try!(write!(f,
                            " Expected token matching {x:?} but found item {t:?} at {p:?}",
                            x = expected,
                            t = found.token(),
                            p = found.position()));

                if let Some(reason) = reason {
                    try!(write!(f, " {}", reason))
                }

                Ok(())
            }
        }
    }
}
