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
use super::data::Data;

pub struct Interpolation;

impl TokenizeState for Interpolation {
    fn new() -> Box<Interpolation> {
        Box::new(Interpolation)
    }

    fn state(&self) -> Code {
        Code::Interpolation
    }

    fn step<'a>(self: Box<Self>, _job: &'a mut Job) -> Result<Box<TokenizeState>,LexerError> {
        unimplemented!()
    }
}
