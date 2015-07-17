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


#[allow(dead_code)] // dummy
pub struct String;

impl TokenizeState for String {
    fn state() -> Code {
        Code::String
    }

    fn tokenize<'a>(_job: &'a mut Job) -> Result<(),LexerError> {
        unimplemented!()
    }
}
