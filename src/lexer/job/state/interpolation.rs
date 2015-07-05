/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Interpolation state of the lexer.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use super::{TokenizeState, Code};
use lexer::error::LexerError;
use lexer::job::Job;

pub struct Interpolation;

impl TokenizeState for Interpolation {
    fn instance() -> &'static Self {
        static INSTANCE : &'static Interpolation = &Interpolation;

        INSTANCE
    }

    fn state(&self) -> Code {
        Code::Interpolation
    }

    fn tokenize<'a>(self: &'static Self, _job: &'a mut Job) -> Result<(),LexerError> {
        unimplemented!()
    }
}
