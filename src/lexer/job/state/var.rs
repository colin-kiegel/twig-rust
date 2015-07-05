/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Var state of the lexer.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use super::{TokenizeState, Code};
use lexer::error::LexerError;
use lexer::job::Job;

pub struct Var;

impl TokenizeState for Var {
    fn instance() -> &'static Self {
        static INSTANCE : &'static Var = &Var;

        INSTANCE
    }

    fn state(&self) -> Code {
        Code::Var
    }

    fn tokenize<'a>(self: &'static Self, _job: &'a mut Job) -> Result<(),LexerError> {
        unimplemented!()
    }
}
