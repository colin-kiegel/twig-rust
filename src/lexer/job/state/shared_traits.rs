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
    // #TODO:290 move this up or find another way to share (like trait)
    //     - because it is shared by block and variable state!
    fn lex_expression<'a>(job: &'a mut Job) -> Result<(), LexerError> {
        // whitespace
        let whitespace = job.cursor.tail().len() - job.cursor.tail().trim_left().len();
        job.cursor.move_by(whitespace); // #TODO:270 move this into cursor (trim_left?)

        if job.cursor.is_eof() {
            let error_code = match Self::state() {
                state::Code::Block => SyntaxErrorCode::UnclosedBlock,
                state::Code::Expression => SyntaxErrorCode::UnclosedVariable,
                _ => SyntaxErrorCode::Unknown, // should be unreachable
            };

            return err!(error_code)
                .explain(format!("at {cursor}", cursor = job.cursor))
                .causes(err!(LexerErrorCode::SyntaxError))
                .into();
        }

        // operators
        if let Some(x) = job.patterns.operator.extract(job.cursor.tail()) {
            // #TODO:330 overwrite extract() in operator pattern: preg_replace('/\s+/', ' ', x)
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
            let x = try!(x);
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
        }

        // strings
        if let Some(x) = job.patterns.string.extract(job.cursor.tail()) {
            job.push_token(Token::String(x.unescape_string()));
            job.cursor.move_by(x.position.1);
            return Self::tokenize(job);
        }

        // opening double quoted string
        // #TODO:630 switch to more simple pattern?
        //      Alternatively get all data generically from the match
        if let Some(_) = job.patterns.string_dq_delim.extract(job.cursor.tail()) {
            let bracket = (BracketType::DoubleQuote, job.cursor.line());
            job.push_bracket(bracket);
            job.cursor.move_by(1);
            try!(state::String::tokenize(job));

            return Self::tokenize(job);
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
