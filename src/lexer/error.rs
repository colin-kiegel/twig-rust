/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Typisation of syntax errors.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use error;

/////////////
// exports //
/////////////

pub type SyntaxError = error::Exception<SyntaxErrorCode>;
pub type TokenError = error::Exception<TokenErrorCode>;
pub type LexerError = error::Exception<LexerErrorCode>;


#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum SyntaxErrorCode {
    Unknown,
    UnexpectedCharacter,
    UnexpectedBracket,
    UnexpectedEof,
    UnclosedBracket,
    UnclosedComment,
    UnclosedBlock,
    UnclosedVariable,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum LexerErrorCode {
    Logic,
    InvalidPatternMatch,
    InvalidValue,
    InvalidState,
    SyntaxError,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum TokenErrorCode {
    NoValue,
}

impl ::std::convert::From<SyntaxError> for LexerError {
    fn from(cause: SyntaxError) -> LexerError {
        let details = ::error::Details {
            message: None,
            .. *cause.details()
        };
        ::error::Exception::new(details, LexerErrorCode::SyntaxError)
            .caused_by(cause)
    }
}

impl ToString for SyntaxErrorCode {
    fn to_string(&self) -> String {
        match *self {
            SyntaxErrorCode::Unknown => "Unknown",
            SyntaxErrorCode::UnexpectedCharacter => "UnexpectedCharacter",
            SyntaxErrorCode::UnexpectedBracket => "UnexpectedBracket",
            SyntaxErrorCode::UnexpectedEof => "UnexpectedEof",
            SyntaxErrorCode::UnclosedBracket => "UnclosedBracket",
            SyntaxErrorCode::UnclosedComment => "UnclosedComment",
            SyntaxErrorCode::UnclosedBlock => "UnclosedBlock",
            SyntaxErrorCode::UnclosedVariable => "UnclosedVariable",
        }.to_string()
    }
}
impl ToString for LexerErrorCode {
    fn to_string(&self) -> String {
        match *self {
            LexerErrorCode::Logic => "Logic",
            LexerErrorCode::InvalidPatternMatch => "InvalidPatternMatch",
            LexerErrorCode::InvalidValue => "InvalidValue",
            LexerErrorCode::InvalidState => "InvalidState",
            LexerErrorCode::SyntaxError => "SyntaxError",
        }.to_string()
    }
}
impl ToString for TokenErrorCode {
    fn to_string(&self) -> String {
        match *self {
            TokenErrorCode::NoValue => "NoValue",
        }.to_string()
    }
}
