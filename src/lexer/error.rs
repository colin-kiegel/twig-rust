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

// exports //
pub type SyntaxError = error::Error<SyntaxErrorCode>;
pub type TokenError = error::Error<TokenErrorCode>;
pub type LexerError = error::Error<LexerErrorCode>;

// imports //
use error;

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
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum TokenErrorCode {
    NoValue,
}
