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

pub type SyntaxError = error::Error<SyntaxErrorCode>;
pub type TokenError = error::Error<TokenErrorCode>;
pub type LexerError = error::Error<LexerErrorCode>;


#[allow(dead_code)]
#[derive(Debug)]
pub enum SyntaxErrorCode {
    Unknown,
    UnexpectedCharacter,
    UnexpectedBracket,
    UnexpectedEof,
    UnclosedBracket,
    UnclosedComment,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum LexerErrorCode {
    Logic,
    InvalidPatternMatch,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum TokenErrorCode {
    NoValue,
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
        }.to_string()
    }
}
impl ToString for LexerErrorCode {
    fn to_string(&self) -> String {
        match *self {
            LexerErrorCode::Logic => "Logic",
            LexerErrorCode::InvalidPatternMatch => "InvalidPatternMatch",
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
