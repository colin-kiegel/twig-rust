/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Data state of the lexer.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use super::{TokenizeState, Code};
use super::_final::Final;
use super::var::Var;
use super::block::Block;
use lexer::error::{LexerError, LexerErrorCode, SyntaxError, SyntaxErrorCode};
use lexer::job::Job;
use lexer::token::Token;
use lexer::patterns::{token_start, block_raw, Extract};

/////////////
// exports //
/////////////

#[allow(dead_code)]
pub struct Data;

#[allow(unused_variables)]
impl TokenizeState for Data {
    fn new() -> Box<Data> {
        Box::new(Data)
    }

    fn state(&self) -> Code {
        Code::Data
    }

    fn step<'a>(self: Box<Self>, job: &'a mut Job) -> Result<Box<TokenizeState>,LexerError> {
        let capture = match Self::next_token_start_after_cursor(job) {
            Some(capture) => capture,
            None => {
                let slice = job.cursor.slice_to_end();
                job.push_token(Token::Text(slice.to_string()));

                return Ok(Final::new())
            },
        };

        let mut slice = job.cursor.slice_to(capture.position.0);

        // trim whitespace according to the next token
        match job.token_start_iter.peek() {
            Some(next) if next.whitespace_trim => {
                slice = slice.trim_right();
            },
            _ => {},
        };

        job.cursor.move_to(capture.position.1);
        job.push_token(Token::Text(slice.to_string()));

        // process and return new Box<Tokenize> (i.e. new state)
        match capture.tag {
            token_start::Tag::Comment => {
                try!(Self::lex_comment(job));
                self.step(job) // direct recursion is faster than dynamic dispatch (return self)
            },
            token_start::Tag::Block => {
                // TODO nested matches don't look nice - alternative??
                // raw data?
                match job.patterns.block_raw.extract(job.cursor.tail()) {
                    Some(block_raw) => {
                        job.cursor.move_by(block_raw.position.1);
                        try!(Self::lex_raw_data(job, block_raw.tag));

                        return Ok(self);
                    },
                    _ => {
                        // {% line \d+ %}
                        match job.patterns.block_line.extract(job.cursor.tail()) {
                            Some(block_line) => {
                                let block_line = try!(block_line);

                                job.cursor.move_by(block_line.position.1);
                                job.cursor.set_line(block_line.line);

                                return Ok(self);
                            },
                            _ => {
                                job.current_var_block_line = job.cursor.line();
                                job.push_token(Token::BlockStart);
                                job.push_state(self);

                                Ok(Block::new())
                            },
                        }
                    },
                }
            },
            token_start::Tag::Variable => {
                job.current_var_block_line = job.cursor.line();
                job.push_token(Token::VarStart);
                job.push_state(self);

                Ok(Var::new())
            }
        }
    }
}

impl<'a> Data {
    fn lex_comment(job: &'a mut Job) -> Result<(), SyntaxError> {
        match job.patterns.comment_end.find(job.cursor.tail()) {
            None => return err!(SyntaxErrorCode::UnclosedComment),
            Some((_, rel_end)) => job.cursor.move_by(rel_end),
        }
        Ok(())
    }

    fn lex_raw_data(job: &'a mut Job, tag: block_raw::Tag) -> Result<(), SyntaxError> {
        let capture = match job.patterns.raw_data.extract_iter(job.cursor.tail())
                                                 .find(|capture| capture.tag == tag) {
            Some(capture) => capture,
            _ => return err!(SyntaxErrorCode::UnexpectedEof, "Unclosed (raw|verbatim) block"),
        };

        let mut slice = job.cursor.slice_by(capture.position.0);
        job.cursor.move_by(capture.position.1);

        if capture.whitespace_trim {
            slice = slice.trim_right();
        }

        job.push_token(Token::Text(slice.to_string()));
        Ok(())
    }

    /// Find the first token after the current cursor
    fn next_token_start_after_cursor(job: &'a mut Job) -> Option<token_start::CaptureData> {
        let position = job.cursor.position();

        job.token_start_iter.find(|x| {
            x.position.0 >= position
        })
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::rc::Rc;
    use lexer::job::Job;
    use template::raw::Raw as Template;
    use lexer::Patterns;
    use lexer::job::state::{TokenizeState, Code};
    use lexer::token::Token;

    #[test]
    pub fn no_more_tokens() {
        const CODE : &'static str = "only data no tokens";
        const FILENAME : &'static str = "only data";

        let ref template = Rc::new(Template::new(CODE, FILENAME));
        let ref patterns = Rc::new(Patterns::default());
        let mut job = Job::new(template, patterns);

        let state = Data::new().step(&mut job).unwrap();

        if !state.is_state(Code::Final) {
            panic!("not final state");
        }

        println!("tokens are: {}", job.tokens.to_string());

        assert_eq!(job.tokens.len(), 1);

        let t_x = Token::Text(CODE.to_string());
        let t_o : Token = job.tokens.into_iter().last().unwrap().into();

        assert_eq!(t_o, t_x);
    }
}
