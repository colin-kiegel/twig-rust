// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Typisation of syntax errors.
///
/// @author Colin Kiegel <kiegel@gmx.de>


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

#[derive(Debug, PartialEq)]
pub enum LexerErrorCode {
    Logic,
    InvalidPattern,
    _InvalidPatternMatch,
    InvalidValue,
    _InvalidState,
    SyntaxError,
}

#[derive(Debug, PartialEq)]
pub enum TokenErrorCode {
    _NoValue,
    UnexpectedToken
}

impl_convert_exception!(SyntaxErrorCode, LexerErrorCode, LexerErrorCode::SyntaxError);

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
            LexerErrorCode::InvalidPattern => "InvalidPattern",
            LexerErrorCode::_InvalidPatternMatch => "InvalidPatternMatch",
            LexerErrorCode::InvalidValue => "InvalidValue",
            LexerErrorCode::_InvalidState => "InvalidState",
            LexerErrorCode::SyntaxError => "SyntaxError",
        }.to_string()
    }
}
impl ToString for TokenErrorCode {
    fn to_string(&self) -> String {
        match *self {
            TokenErrorCode::_NoValue => "NoValue",
            TokenErrorCode::UnexpectedToken => "UnexpectedToken",
        }.to_string()
    }
}
