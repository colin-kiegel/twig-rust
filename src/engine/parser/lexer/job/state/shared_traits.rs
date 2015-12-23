// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! FSM model of the lexer.

use super::TokenizeState;
use engine::parser::lexer::job::state;
use engine::parser::lexer::job::Job;
use engine::parser::token::{Token, Punctuation, BracketType};
use engine::parser::lexer::patterns::{number, Extract};
use engine::parser::lexer::{LexerError, SyntaxError};
use api::error::{Traced, Dump};

pub trait LexExpression where
    Self: Sized + TokenizeState
{
    // TODO: move this up or find another way to share (like trait)
    //     - because it is shared by block and variable state!
    fn lex_expression<'a>(job: &'a mut Job) -> Result<(), Traced<LexerError>> {
        // whitespace
        let whitespace = job.cursor.tail().len() - job.cursor.tail().trim_left().len();
        job.cursor.move_by(whitespace); // TODO: move this into cursor (trim_left?)

        if job.cursor.is_eof() {
            let error_code = match Self::state() {
                state::Code::Block => SyntaxError::UnclosedBlock { cursor: job.cursor.dump() },
                state::Code::Expression => {
                    SyntaxError::UnclosedVariable { cursor: job.cursor.dump() }
                }
                _ => {
                    SyntaxError::Unreachable {
                        reason: "End of template while lexing expression".to_string(),
                        cursor: job.cursor.dump(),
                    }
                }
            };

            try_traced!(traced_err!(error_code))
        }

        // operators
        if let Some(x) = job.patterns.operator.extract(job.cursor.tail()) {
            // TODO: overwrite extract() in operator pattern: preg_replace('/\s+/', ' ', x)
            job.push_token(Token::Operator(x.operator.to_string()));
            job.cursor.move_by(x.position.1);
            return Self::tokenize(job);
        }

        // names
        if let Some(x) = job.patterns.name.extract(job.cursor.tail()) {
            job.push_token(Token::Name(x.name.to_string()));
            job.cursor.move_by(x.position.1);
            return Self::tokenize(job);
        };

        // numbers
        if let Some(x) = job.patterns.number.extract(job.cursor.tail()) {
            let x = try_traced!(x);
            let token = match x.number {
                number::Number::Integer(u) => Token::IntegerNumber(u),
                number::Number::Floating(f) => Token::FloatingNumber(f),
            };
            job.push_token(token);
            job.cursor.move_by(x.position.1);
            return Self::tokenize(job);
        };

        // punctuation
        if let Some(punctuation) = job.patterns.punctuation.extract(job.cursor.tail()) {
            match punctuation { // check brackets ..
                Punctuation::ClosingBracket(ref b) => {
                    match job.pop_bracket() {
                        None => {
                            return try_traced!(traced_err!(SyntaxError::UnexpectedBracket {
                                bracket: b.clone(),
                                cursor: job.cursor.dump(),
                            }))
                        }
                        Some((b_expected, line)) => {
                            if *b != b_expected {
                                return try_traced!(traced_err!(SyntaxError::UnclosedBracket {
                                    bracket_before: b_expected,
                                    line_before: line,
                                    bracket: b.clone(),
                                    cursor: job.cursor.dump(),
                                }));
                            }

                            let bracket = (b.clone(), job.cursor.line());
                            job.push_bracket(bracket);
                        }
                    }
                }
                Punctuation::OpeningBracket(ref b) => {
                    let bracket = (b.clone(), job.cursor.line());
                    job.push_bracket(bracket);
                }
                _ => {}
            };

            // .. then ..
            job.push_token(Token::Punctuation(punctuation));
            job.cursor.move_by(1);
            return Self::tokenize(job);
        }

        // strings
        if let Some(x) = job.patterns.string.extract(job.cursor.tail()) {
            job.push_token(Token::String(x.unescape_string()));
            job.cursor.move_by(x.position.1);
            return Self::tokenize(job);
        }

        // opening double quoted string
        // TODO: switch to more simple pattern?
        //      Alternatively get all data generically from the match
        if let Some(_) = job.patterns.string_dq_delim.extract(job.cursor.tail()) {
            let bracket = (BracketType::DoubleQuote, job.cursor.line());
            job.push_bracket(bracket);
            job.cursor.move_by(1);
            try_traced!(state::String::tokenize(job));

            return Self::tokenize(job);
        }

        // unlexable
        println!("Current Job Status: {:?}", job); // DEBUG INFO

        let syntax_error = match job.cursor.tail().chars().next() {
            Some(c) => {
                traced_err!(SyntaxError::UnexpectedCharacter {
                    character: c,
                    cursor: job.cursor.dump(),
                })
            }
            None => {
                traced_err!(SyntaxError::UnexpectedEof {
                    reason: "Unclosed expression",
                    cursor: job.cursor.dump(),
                })
            }
        };

        try_traced!(syntax_error)
    }
}
