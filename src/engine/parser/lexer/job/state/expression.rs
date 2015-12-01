// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Var state of the lexer.

use super::{TokenizeState, Code};
use engine::parser::lexer::job::Job;
use engine::parser::token::Token;
use engine::parser::lexer::patterns::{Extract};
use engine::parser::lexer::{LexerError};
use super::shared_traits::LexExpression;

#[allow(dead_code)] // #TODO:190 dummy
pub struct Expression;

impl TokenizeState for Expression {
    fn state() -> Code {
        Code::Expression
    }

    fn tokenize<'a>(job: &'a mut Job) -> Result<(),LexerError> {
        if job.brackets.is_empty() {
            match job.patterns.expression_end.extract(job.cursor.tail()) {
                Some(item) => {
                    job.cursor.move_by(item.position.1);
                    job.push_token(Token::ExpressionEnd);

                    return Ok(());//try!(job.pop_state()).tokenize(job);
                },
                _ => {},
            }
        };

        return Self::lex_expression(job);
    }
}

impl LexExpression for Expression {}
