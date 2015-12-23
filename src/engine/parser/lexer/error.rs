// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Typisation of lexer and syntax errors.

use std::fmt::{self, Display};
use std::error::Error;
use std::num::{ParseFloatError, ParseIntError};
use engine::parser::token;
use engine::parser::lexer::job::cursor;
use regex;

#[derive(Debug)]
pub enum SyntaxError {
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
        cursor: cursor::CursorDump,
    },
}

impl Error for SyntaxError {
    fn description(&self) -> &str {
        match *self {
            SyntaxError::Unreachable{..} => {
                "Unexptected syntax error (please report as bug with details)."
            }
            SyntaxError::UnexpectedCharacter{..} => "Unexpected character.",
            SyntaxError::UnexpectedBracket{..} => "Unexpected bracket.",
            SyntaxError::UnexpectedEof{..} => "Unexpected end of template.",
            SyntaxError::UnclosedBracket{..} => "Unclosed bracket.",
            SyntaxError::UnclosedComment{..} => "Unclosed comment.",
            SyntaxError::UnclosedBlock{..} => "Unclosed block.",
            SyntaxError::UnclosedVariable{..} => "Unclosed variable.",
        }
    }
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.description()));

        match *self {
            SyntaxError::Unreachable {
                reason: ref r,
                cursor: ref c
            } => write!(f, " {reason} at {cursor}.", reason = r, cursor = c),
            SyntaxError::UnexpectedCharacter {
                character, ref cursor
            } => {
                write!(f,
                       " found '{c}' at {cursor}.",
                       c = character,
                       cursor = cursor)
            }
            SyntaxError::UnexpectedBracket {
                ref cursor, ref bracket
            } => {
                write!(f,
                       " Unexpected {bracket:?} at {cursor}.",
                       cursor = cursor,
                       bracket = bracket)
            }
            SyntaxError::UnexpectedEof {
                reason: ref r,
                cursor: ref c
            } => write!(f, " {reason} at {cursor}.", reason = r, cursor = c),
            SyntaxError::UnclosedBracket {
                ref cursor, ref bracket, ref bracket_before, line_before
            } => {
                write!(f,
                       " Unclosed {b_before:?} from line{line_before} but found {b:?} at {cursor}.",
                       cursor = cursor,
                       b = bracket,
                       b_before = bracket_before,
                       line_before = line_before)
            }
            SyntaxError::UnclosedComment {
                ref cursor
            } => write!(f, " At {cursor}.", cursor = cursor),
            SyntaxError::UnclosedBlock {
                ref cursor
            } => write!(f, " At {cursor}.", cursor = cursor),
            SyntaxError::UnclosedVariable {
                ref cursor
            } => write!(f, " At {cursor}.", cursor = cursor),
        }
    }
}

#[derive(Debug)]
pub enum LexerError {
    Unreachable {
        reason: String,
    },
    MissingExtensions,
    InvalidRegexPattern(regex::Error),
    _InvalidPatternMatch,
    InvalidFloat {
        value: String,
        parse_error: ParseFloatError,
    },
    InvalidInteger {
        value: String,
        parse_error: ParseIntError,
    },
    _InvalidState,
    Syntax(SyntaxError),
}

impl From<SyntaxError> for LexerError {
    fn from(err: SyntaxError) -> LexerError {
        LexerError::Syntax(err)
    }
}

impl From<regex::Error> for LexerError {
    fn from(err: regex::Error) -> LexerError {
        LexerError::InvalidRegexPattern(err)
    }
}

impl Error for LexerError {
    fn description(&self) -> &str {
        match *self {
            LexerError::Unreachable{..} => {
                "Unexptected lexer error (please report as bug with details)."
            }
            LexerError::MissingExtensions => {
                "Could not initialize lexer due to missing engine extensions."
            }
            LexerError::InvalidRegexPattern(..) => {
                "Could not initialize lexer due to invalid regular expression."
            }
            LexerError::_InvalidPatternMatch => "Invalid pattern match.",
            LexerError::InvalidFloat{..} => "Invalid float.",
            LexerError::InvalidInteger{..} => "Invalid integer.",
            LexerError::_InvalidState => "Invalid state.",
            LexerError::Syntax(..) => "Syntax error.",
        }
    }
}

impl Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{} ", self.description()));

        match *self {
            LexerError::Unreachable {
                ref reason
            } => write!(f, "{}.", reason),
            LexerError::MissingExtensions => Ok(()),
            LexerError::InvalidRegexPattern(..) => Ok(()),
            LexerError::_InvalidPatternMatch => Ok(()),
            LexerError::InvalidFloat {
                ref value, parse_error: _
            } => write!(f, "Found value {}.", value),
            LexerError::InvalidInteger {
                ref value, parse_error: _
            } => write!(f, "Found value {}.", value),
            LexerError::_InvalidState => Ok(()),
            LexerError::Syntax(ref e) => Display::fmt(e, f),
        }
    }
}
