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

pub type TwigError = error::Exception<TwigErrorCode>;

#[derive(Debug, PartialEq)]
pub enum TwigErrorCode {
    Logic,
    Loader,
    Lexer,
    Parser,
    Runtime,
}

impl ToString for TwigErrorCode {
    fn to_string(&self) -> String {
        match *self {
            TwigErrorCode::Logic => "Logic",
            TwigErrorCode::Loader => "Loader",
            TwigErrorCode::Lexer => "Lexer",
            TwigErrorCode::Parser => "Parser",
            TwigErrorCode::Runtime => "Runtime",
        }.to_string()
    }
}
