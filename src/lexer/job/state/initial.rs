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

use super::*;
use lexer::SyntaxError;
use lexer::job::Job;
use super::data::Data;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Initial;

impl Tokenize for Initial {
    fn new() -> Box<Self> {
        Box::new(Initial)
    }

    fn get_type(&self) -> Code {
        Code::Initial
    }

    fn step<'a> (&self, _job: &mut Job<'a>) -> Result<Box<Tokenize>,SyntaxError> {
        // TODO some pre-checks, like len>0?
        Ok(Data::new())
    }
}
