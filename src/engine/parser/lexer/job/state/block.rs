// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Block state of the lexer.

use super::{TokenizeState, Code};
use engine::parser::lexer::job::Job;
use engine::parser::token::Token;
use engine::parser::lexer::patterns::Extract;
use engine::parser::lexer::LexerError;
use super::shared_traits::LexExpression;
use api::error::Traced;

#[allow(dead_code)] // dummy
pub struct Block;

impl TokenizeState for Block {
    fn state() -> Code {
        Code::Block
    }

    fn tokenize<'a>(job: &'a mut Job) -> Result<(), Traced<LexerError>> {
        if job.brackets.is_empty() {
            match job.patterns.block_end.extract(job.cursor.tail()) {
                Some(item) => {
                    job.cursor.move_by(item.position.1);
                    job.push_token(Token::BlockEnd);

                    return Ok(());//try_traced!(job.pop_state()).tokenize(job);
                }
                _ => {}
            }
        };

        return Self::lex_expression(job);
    }
}

impl LexExpression for Block {}
