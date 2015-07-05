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

pub mod shared_traits;
pub mod initial;
pub mod data;
pub mod block;
pub mod var;
pub mod string;
pub mod interpolation;
pub mod _final;
pub use self::initial::Initial;
pub use self::data::Data;
pub use self::block::Block;
pub use self::var::Var;
pub use self::string::String;
pub use self::interpolation::Interpolation;
pub use self::_final::Final;

// TODO check to remove initial and final state - after all states are implemented
pub trait TokenizeState {
    fn instance() -> &'static Self where // Flyweight- + Singleton-pattern
        Self: Sized;

    /// tokenize recursively
    fn tokenize<'a>(self: &'static Self, _job: &'a mut Job) -> Result<(), LexerError> {
        unimplemented!()
    }

    fn state(&self) -> Code;

    fn is_state(&self, code: Code) -> bool {
        self.state() == code
    }
}

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
