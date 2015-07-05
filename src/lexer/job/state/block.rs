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
use lexer::job::Job;
use lexer::token::Token;
use lexer::patterns::{Extract};
use lexer::error::{LexerError};
use super::shared_traits::LexExpression;

pub struct Block;

impl TokenizeState for Block {
    fn instance() -> &'static Self {
        static INSTANCE : &'static Block = &Block;

        INSTANCE
    }

    fn state(&self) -> Code {
        Code::Block
    }

    fn tokenize<'a>(self: &'static Self, job: &'a mut Job) -> Result<(),LexerError> {
        if job.brackets.is_empty() {
            match job.patterns.block_end.extract(job.cursor.tail()) {
                Some(item) => {
                    job.cursor.move_by(item.position.1);
                    job.push_token(Token::BlockEnd);

                    return try!(job.pop_state()).tokenize(job);
                },
                _ => {},
            }
        };

        return self.lex_expression(job);
    }
}

impl LexExpression for Block {}
