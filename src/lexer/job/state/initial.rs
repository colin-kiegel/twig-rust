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

pub struct Initial;

impl TokenizeState for Initial {
    fn new() -> Box<Initial> {
        Box::new(Initial)
    }

    fn state(&self) -> Code {
        Code::Initial
    }

    fn step<'a>(self: Box<Self>, _job: &'a mut Job) -> Result<Box<TokenizeState>,LexerError> {
        // TODO some pre-checks, like len>0?
        Ok(Data::new())
    }
}
