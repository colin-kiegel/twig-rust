/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Var state of the lexer.
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


#[allow(dead_code)] // TODO dummy
pub struct Var;

impl TokenizeState for Var {
    fn state() -> Code {
        Code::Var
    }

    fn tokenize<'a>(job: &'a mut Job) -> Result<(),LexerError> {
        if job.brackets.is_empty() {
            match job.patterns.var_end.extract(job.cursor.tail()) {
                Some(item) => {
                    job.cursor.move_by(item.position.1);
                    job.push_token(Token::VarEnd);

                    return Ok(());//try!(job.pop_state()).tokenize(job);
                },
                _ => {},
            }
        };

        return Self::lex_expression(job);
    }
}

impl LexExpression for Var {}
