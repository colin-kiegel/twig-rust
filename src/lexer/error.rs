// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Typisation of syntax errors.

use std::fmt::{self, Display};
use error::Error;
use error::{GeneralizeTo, ErrorCode};

use lexer::token;
use lexer::job::cursor;

pub type SyntaxError = Error<SyntaxErrorCode>;
pub type TokenError = Error<TokenErrorCode>;
pub type LexerError = Error<LexerErrorCode>;

impl GeneralizeTo<LexerErrorCode> for SyntaxErrorCode {
    fn generalize(&self) -> LexerErrorCode { LexerErrorCode::SyntaxError }
}

#[derive(Debug)]
pub enum SyntaxErrorCode {
    Unreachable {
        reason: String,
        cursor: cursor::CursorDump,
    },
    UnexpectedCharacter {
        character: char,
        cursor: cursor::CursorDump,
    },
    UnexpectedBracket {
        bracket: token::BracketType,
        cursor: cursor::CursorDump,
    },
    UnexpectedEof {
        reason: &'static str,
        cursor: cursor::CursorDump,
    },
    UnclosedBracket {
        bracket: token::BracketType,
        bracket_before: token::BracketType,
        line_before: usize,
        cursor: cursor::CursorDump,
    },
    UnclosedComment {
        cursor: cursor::CursorDump,
    },
    UnclosedBlock {
        cursor: cursor::CursorDump,
    },
    UnclosedVariable {
        cursor: cursor::CursorDump
    },
}

impl ErrorCode for SyntaxErrorCode {
    fn description(&self) -> &str {
        match *self {
            SyntaxErrorCode::Unreachable{..} => "Unexptected syntax error (please report as bug with details).",
            SyntaxErrorCode::UnexpectedCharacter{..} => "Unexpected character.",
            SyntaxErrorCode::UnexpectedBracket{..} => "Unexpected bracket.",
            SyntaxErrorCode::UnexpectedEof{..} => "Unexpected end of template.",
            SyntaxErrorCode::UnclosedBracket{..} => "Unclosed bracket.",
            SyntaxErrorCode::UnclosedComment{..} => "Unclosed comment.",
            SyntaxErrorCode::UnclosedBlock{..} => "Unclosed block.",
            SyntaxErrorCode::UnclosedVariable{..} => "Unclosed variable.",
        }
    }
}

impl Display for SyntaxErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.description()));

        match *self {
            SyntaxErrorCode::Unreachable {
                reason: ref r,
                cursor: ref c
            } => {
                write!(f, " {reason} at {cursor}.",
                    reason = r,
                    cursor = c)
            },
            SyntaxErrorCode::UnexpectedCharacter {
                character, ref cursor
            } => {
                write!(f, " found '{c}' at {cursor}.",
                    c = character, cursor = cursor)
            },
            SyntaxErrorCode::UnexpectedBracket {
                ref cursor, ref bracket
            } => {
                write!(f, " Unexpected {bracket:?} at {cursor}.",
                    cursor = cursor, bracket = bracket)
            },
            SyntaxErrorCode::UnexpectedEof {
                reason: ref r,
                cursor: ref c
            } => {
                write!(f, " {reason} at {cursor}.",
                    reason = r,
                    cursor = c)
            },
            SyntaxErrorCode::UnclosedBracket {
                ref cursor, ref bracket, ref bracket_before, line_before
            } => {
                write!(f, " Unclosed {b_before:?} from line\
                                {line_before} but found {b:?} at {cursor}.",
                    cursor = cursor,
                    b = bracket,
                    b_before = bracket_before,
                    line_before = line_before)
            },
            SyntaxErrorCode::UnclosedComment {
                ref cursor
            } => {
                write!(f, " At {cursor}.",
                    cursor = cursor)
            },
            SyntaxErrorCode::UnclosedBlock {
                ref cursor
            } => {
                write!(f, " At {cursor}.",
                    cursor = cursor)
            },
            SyntaxErrorCode::UnclosedVariable {
                ref cursor
            } => {
                write!(f, " At {cursor}.",
                    cursor = cursor)
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LexerErrorCode {
    Unreachable {
        reason: String
    },
    MissingExtensions,
    PatternRegexError,
    _InvalidPatternMatch,
    InvalidValue {
        value: String
    },
    _InvalidState,
    SyntaxError,
}

impl ErrorCode for LexerErrorCode {
    fn description(&self) -> &str {
        match *self {
            LexerErrorCode::Unreachable{..} => "Unexptected lexer error (please report as bug with details).",
            LexerErrorCode::MissingExtensions => "Could not initialize lexer due to missing engine extensions.",
            LexerErrorCode::PatternRegexError => "Could not initialize lexer due to invalid regular expression.",
            LexerErrorCode::_InvalidPatternMatch => "Invalid pattern match.",
            LexerErrorCode::InvalidValue{..} => "Invalid value.",
            LexerErrorCode::_InvalidState => "Invalid state.",
            LexerErrorCode::SyntaxError => "Syntax error.",
        }
    }
}

impl Display for LexerErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.description()));

        match *self {
            LexerErrorCode::Unreachable {
                ref reason
            } => {
                write!(f, " {}.", reason)
            },
            LexerErrorCode::MissingExtensions => Ok(()),
            LexerErrorCode::PatternRegexError => Ok(()),
            LexerErrorCode::_InvalidPatternMatch => Ok(()),
            LexerErrorCode::InvalidValue {
                ref value
            } => {
                write!(f, " Found value {}", value)
            },
            LexerErrorCode::_InvalidState => Ok(()),
            LexerErrorCode::SyntaxError => Ok(()),
        }
    }
}

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
