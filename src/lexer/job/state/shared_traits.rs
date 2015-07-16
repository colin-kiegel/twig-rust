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
use lexer::token::{Token, Punctuation, BracketType};
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
    fn lex_expression<'a>(job: &'a mut Job) -> Result<(), LexerError> {
        // whitespace
        let whitespace = job.cursor.tail().len() - job.cursor.tail().trim_left().len();
        job.cursor.move_by(whitespace); // TODO move this into cursor (trim_left?)

        if job.cursor.is_eof() {
            let error_code = match Self::state() {
                state::Code::Block => SyntaxErrorCode::UnclosedBlock,
                state::Code::Var => SyntaxErrorCode::UnclosedVariable,
                _ => SyntaxErrorCode::Unknown, // should be unreachable
            };

            return err!(error_code)
                .explain(format!("at {cursor}", cursor = job.cursor))
                .causes(err!(LexerErrorCode::SyntaxError))
                .into();
        }

        // operators - TODO
        //
        // unimplemented!()

        // names
        match job.patterns.name.extract(job.cursor.tail()) {
            Some(x) => {
                job.push_token(Token::Name(x.name.to_string()));
                job.cursor.move_by(x.position.1);
                return Self::tokenize(job);
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
                return Self::tokenize(job);
            },
            None => {},
        };

        // punctuation
        match job.patterns.punctuation.extract(job.cursor.tail()) {
            Some(punctuation) => {
                match punctuation { // check brackets ..
                    Punctuation::ClosingBracket(ref b) => match job.pop_bracket() {
                        None => {
                            return err!(SyntaxErrorCode::UnexpectedBracket)
                                .explain(format!("Unexpected {b:?} at {cursor}",
                                    b = b,
                                    cursor = job.cursor))
                                .causes(err!(LexerErrorCode::SyntaxError))
                                .into();
                        },
                        Some((b_expected, line)) => {
                            if *b != b_expected {
                                return err!(SyntaxErrorCode::UnclosedBracket)
                                    .explain(format!("Unclosed {b_before:?} from line\
                                                    {line_before} but found {b:?} at {cursor}",
                                        b_before = b_expected,
                                        line_before = line,
                                        b = b,
                                        cursor = job.cursor))
                                    .causes(err!(LexerErrorCode::SyntaxError))
                                    .into();
                            }

                            let bracket = (b.clone(), job.cursor.line());
                            job.push_bracket(bracket);
                        },
                    },
                    Punctuation::OpeningBracket(ref b) => {
                        let bracket = (b.clone(), job.cursor.line());
                        job.push_bracket(bracket);
                    },
                    _ => {},
                };

                // .. then ..
                job.push_token(Token::Punctuation(punctuation));
                job.cursor.move_by(1);
                return Self::tokenize(job);
            },
            None => {},
        }

        // strings
        match job.patterns.string.extract(job.cursor.tail()) {
            Some(x) => {
                job.push_token(Token::String(x.unescape_string()));
                job.cursor.move_by(x.position.1);
                return Self::tokenize(job);
            },
            None => {},
        }

        // opening double quoted string
        // TODO switch to more simple pattern?
        //      Alternatively get all data generically from the match
        match job.patterns.string_dq_delim.extract(job.cursor.tail()) {
            Some(_) => {
                let bracket = (BracketType::DoubleQuote, job.cursor.line());
                job.push_bracket(bracket);
                job.cursor.move_by(1);
                try!(state::String::tokenize(job));

                return Self::tokenize(job);
            },
            None => {},
        }

        // unlexable
        println!("Current Job Status: {:?}", job); // DEBUG INFO

        let syntax_error = match job.cursor.tail().chars().next() {
            Some(c) => err!(SyntaxErrorCode::UnexpectedCharacter)
                .explain(format!("'{c}' at {cursor}",
                    c = c,
                    cursor = job.cursor)),
            None => err!(SyntaxErrorCode::UnexpectedEof)
                .explain(format!("at {cursor}", cursor = job.cursor)),
        };

        return syntax_error.causes(err!(LexerErrorCode::SyntaxError)).into();
    }
}
