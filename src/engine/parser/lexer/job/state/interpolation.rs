// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Interpolation state of the lexer.

use super::{TokenizeState, Code};
use engine::parser::lexer::LexerError;
use engine::parser::lexer::job::Job;
use api::error::Traced;

#[allow(dead_code)] // dummy
pub struct Interpolation;

impl TokenizeState for Interpolation {
    fn state() -> Code {
        Code::Interpolation
    }

    fn tokenize<'a>(_job: &'a mut Job) -> Result<(), Traced<LexerError>> {
        unimplemented!()
    }
}
