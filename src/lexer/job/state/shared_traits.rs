/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * FSM model of the lexer.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use super::TokenizeState;
use lexer::job::state;
use lexer::job::Job;
use lexer::token::Token;
use lexer::patterns::{number, Extract};
use lexer::error::{LexerError, LexerErrorCode, SyntaxErrorCode};

/////////////
// exports //
/////////////

pub trait LexExpression where
    Self: Sized + TokenizeState
{
    // TODO move this up or find another way to share (like trait)
    //     - because it is shared by block and variable state!
    fn lex_expression<'a>(&'static self, job: &'a mut Job) -> Result<(), LexerError> {
        // whitespace
        let whitespace = job.cursor.tail().len() - job.cursor.tail().trim_left().len();
        job.cursor.move_by(whitespace); // TODO move this into cursor (trim_left?)

        if job.cursor.is_eof() {
            let error_code = match self.state() {
                state::Code::Block => SyntaxErrorCode::UnclosedBlock,
                state::Code::Var => SyntaxErrorCode::UnclosedVariable,
                _ => unimplemented!(),
            };

            return err!(error_code)
                .explain(format!("in {file}:{line}",
                    file = job.template.filename(),
                    line = job.cursor.line()))
                .causes(err!(LexerErrorCode::SyntaxError))
                .into();
        }

        // operators
        //
        // unimplented!()
        // TODO !!!!

        // names
        match job.patterns.name.extract(job.cursor.tail()) {
            Some(x) => {
                job.push_token(Token::Name(x.name.to_string()));
                job.cursor.move_by(x.position.1);
                return self.tokenize(job);
            },
            None => {},
        };

        // numbers
        match job.patterns.number.extract(job.cursor.tail()) {
            Some(x) => {
                let x = try!(x);
                let token = match x.number {
                    number::Number::Integer(u) => Token::IntegerNumber(u),
                    number::Number::Floating(f) => Token::FloatingNumber(f),
                };
                job.push_token(token);
                job.cursor.move_by(x.position.1);
                return self.tokenize(job);
            },
            None => {},
        };

        // punctuation
        //
        // unimplented!()
        // TODO !!!!

        // strings
        match job.patterns.string.extract(job.cursor.tail()) {
            Some(x) => {
                job.push_token(Token::String(x.unescape_string()));
                job.cursor.move_by(x.position.1);
                return self.tokenize(job);
            },
            None => {},
        }

        // opening double quoted string
        // TODO switch to more simple pattern?
        //      Alternatively get all data generically from the match
        match job.patterns.dq_string_delim.extract(job.cursor.tail()) {
            Some(_) => {
                let bracket = ("\"", job.cursor.line()); // TODO introduce a bracket-object?
                job.push_bracket(bracket);
                job.cursor.move_by(1);
                job.push_state(self);
                return state::String::instance().tokenize(job);
            },
            None => {},
        }

        // unlexable
        let syntax_error = match job.cursor.tail().chars().next() {
            Some(c) => err!(SyntaxErrorCode::UnexpectedCharacter)
                .explain(format!("'{c}' in {filename}:{line}",
                    c = c,
                    line = job.cursor.line(),
                    filename = job.cursor.template().filename(),
                )),
            None => err!(SyntaxErrorCode::UnexpectedEof)
                .explain(format!("in {filename}:{line}",
                    line = job.cursor.line(),
                    filename = job.cursor.template().filename(),
                )),
        };

        return LexerError::from(syntax_error).into();
    }
}