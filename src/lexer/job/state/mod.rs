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

/////////////
// imports //
/////////////

use lexer::error::LexerError;
use lexer::job::Job;

/////////////
// exports //
/////////////

pub mod initial;
pub mod data;
pub mod block;
pub mod var;
pub mod string;
pub mod interpolation;
pub mod _final;
pub use self::initial::Initial;


pub trait TokenizeState {
    fn new() -> Box<Self> where
        Self: Sized;

    fn step<'a>(self: Box<Self>, job: &'a mut Job) -> Result<Box<TokenizeState>,LexerError>;

    fn state(&self) -> Code;

    fn is_state(&self, code: Code) -> bool {
        self.state() == code
    }

    fn is_finished(&self) -> bool{
        false
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Code {
    Data            = 0,
    Block           = 1,
    Var             = 2,
    String          = 3,
    Interpolation   = 4,
    Initial         = -1, // orig: implicit sub-state of Data
    Final           = -2, // orig: implicit sub-state
}

#[allow(dead_code)]
impl Default for Code {
    fn default() -> Code {
        Code::Initial
    }
}
