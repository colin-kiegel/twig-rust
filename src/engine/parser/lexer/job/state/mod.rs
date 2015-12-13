// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! FSM model of the lexer.

use engine::parser::lexer::LexerError;
use engine::parser::lexer::job::Job;
use api::error::Traced;

pub mod shared_traits;
pub mod initial;
pub mod data;
pub mod block;
pub mod expression;
pub mod string;
pub mod interpolation;
pub mod final_;
pub use self::initial::Initial;
pub use self::data::Data;
pub use self::block::Block;
pub use self::expression::Expression;
pub use self::string::String;
pub use self::interpolation::Interpolation;
pub use self::final_::Final;

pub trait TokenizeState {
    /// tokenize recursively
    fn tokenize<'a>(_job: &'a mut Job) -> Result<(), Traced<LexerError>> where
        Self: Sized;

    fn state() -> Code where
        Self: Sized;

    fn is_state(code: Code) -> bool where
        Self: Sized
    {
        Self::state() == code
    }
}

#[derive(Debug, PartialEq)]
pub enum Code {
    Data            = 0,
    Block           = 1,
    Expression      = 2, // orig: var
    String          = 3,
    Interpolation   = 4,
    Initial         = -1, // orig: implicit sub-state of Data
    Final           = -2, // orig: implicit sub-state
}
