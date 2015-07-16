/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Initial state of the lexer job.
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

#[allow(dead_code)]
pub struct Initial;

impl TokenizeState for Initial {
    fn state() -> Code {
        Code::Initial
    }

    fn tokenize<'a>(job: &'a mut Job) -> Result<(),LexerError> {
        // TODO some pre-checks, like len>0?
        return Data::tokenize(job)
    }
}
