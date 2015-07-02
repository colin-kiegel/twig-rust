/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Block state of the lexer.
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

pub struct Block;

impl TokenizeState for Block {
    fn new() -> Box<Block> {
        Box::new(Block)
    }

    fn state(&self) -> Code {
        Code::Block
    }

    fn step<'a>(self: Box<Self>, _job: &'a mut Job) -> Result<Box<TokenizeState>,LexerError> {
        unimplemented!()
    }
}
