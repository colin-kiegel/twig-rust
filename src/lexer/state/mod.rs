/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * FSM model of the lexer.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

// exports
pub mod syntax_error;
pub use self::syntax_error::Code as SyntaxErrorCode;
pub type SyntaxError = error::Error<SyntaxErrorCode>;
pub mod block;
pub mod data;
pub mod interpolation;
pub mod string;
pub mod var;

// imports
//use error::macros;
use error;

pub trait State {
    fn lex(&self) -> Result<Option<Box<State>>,SyntaxError> {
        err!("not implemented", SyntaxErrorCode::Unknown)
    }
}

#[allow(dead_code)]
pub enum Code {
    Data            = 0,
    Block           = 1,
    Var             = 2,
    String          = 3,
    Interpolation   = 4,
}

/*#[allow(dead_code)]
impl Default for State {
    fn default() -> State {
        State::Data
    }
}*/
