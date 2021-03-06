// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Initial state of the lexer job.

use super::{TokenizeState, Code};
use engine::parser::lexer::LexerError;
use engine::parser::lexer::job::Job;
use super::data::Data;
use api::error::Traced;

#[allow(dead_code)] // dummy
pub struct Initial;

impl TokenizeState for Initial {
    fn state() -> Code {
        Code::Initial
    }

    fn tokenize<'a>(job: &'a mut Job) -> Result<(), Traced<LexerError>> {
        // TODO: some pre-checks, like len>0?
        return Data::tokenize(job);
    }
}
