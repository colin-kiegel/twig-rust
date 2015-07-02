/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * String state of the lexer.
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

pub struct String;

impl TokenizeState for String {
    fn new() -> Box<String> {
        Box::new(String)
    }

    fn state(&self) -> Code {
        Code::String
    }

    fn step<'a>(self: Box<Self>, _job: &'a mut Job) -> Result<Box<TokenizeState>,LexerError> {
        unimplemented!()
    }
}
