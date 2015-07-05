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
use lexer::job::state;
use lexer::job::Job;
use lexer::token::Token;
use lexer::patterns::{token_start, block_raw, Extract};
use lexer::error::{LexerError, SyntaxError, SyntaxErrorCode};

/////////////
// exports //
/////////////

#[allow(dead_code)]
pub struct Data;

#[allow(unused_variables)]
impl TokenizeState for Data {
    fn instance() -> &'static Self {
        static INSTANCE : &'static Data = &Data;

        INSTANCE
    }

    fn state(&self) -> Code {
        Code::Data
    }

    fn tokenize<'a>(self: &'static Self, job: &'a mut Job) -> Result<(),LexerError> {
        let capture = match Self::next_token_start_after_cursor(job) {
            Some(capture) => capture,
            None => {
                let slice = job.cursor.slice_to_end();
                job.push_token(Token::Text(slice.to_string()));

                return state::Final::instance().tokenize(job)
            },
        };

        let mut slice = job.cursor.slice_to(capture.position.0);

        if capture.whitespace_trim {
            slice = slice.trim_right();
        }

        job.cursor.move_to(capture.position.1);
        job.push_token(Token::Text(slice.to_string()));

        match capture.tag {
            token_start::Tag::Comment => {
                try!(Self::lex_comment(job));
                return self.tokenize(job);
            },
            token_start::Tag::Block => {
                // TODO nested matches don't look nice - alternative??
                // raw data?
                match job.patterns.block_raw.extract(job.cursor.tail()) {
                    Some(block_raw) => {
                        job.cursor.move_by(block_raw.position.1);
                        try!(Self::lex_raw_data(job, block_raw.tag));

                        return self.tokenize(job);
                    },
                    _ => {
                        // {% line \d+ %}
                        match job.patterns.block_line.extract(job.cursor.tail()) {
                            Some(block_line) => {
                                let block_line = try!(block_line);

                                job.cursor.move_by(block_line.position.1);
                                job.cursor.set_line(block_line.line);

                                return self.tokenize(job);
                            },
                            _ => {
                                job.current_var_block_line = job.cursor.line();
                                job.push_state(self);
                                job.push_token(Token::BlockStart);

                                return state::Block::instance().tokenize(job);
                            },
                        }
                    },
                }
            },
            token_start::Tag::Variable => {
                job.current_var_block_line = job.cursor.line();
                job.push_state(self);
                job.push_token(Token::VarStart);

                return state::Var::instance().tokenize(job);
            }
        }
    }
}

impl<'a> Data {
    fn lex_comment(job: &'a mut Job) -> Result<(), SyntaxError> {
        match job.patterns.comment_end.find(job.cursor.tail()) {
            None => return err!(SyntaxErrorCode::UnclosedComment).into(),
            Some((_, rel_end)) => job.cursor.move_by(rel_end),
        }
        Ok(())
    }

    fn lex_raw_data(job: &'a mut Job, tag: block_raw::Tag) -> Result<(), SyntaxError> {
        let capture = match job.patterns.raw_data.extract_iter(job.cursor.tail())
                                                 .find(|capture| capture.tag == tag) {
            Some(capture) => capture,
            _ => return err!(SyntaxErrorCode::UnexpectedEof, "Unclosed (raw|verbatim) block").into(),
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
    fn next_token_start_after_cursor(job: &'a mut Job) -> Option<token_start::ItemData> {
        let position = job.cursor.position();

        job.token_start_iter.find(|x| {
            x.position.0 >= position
        })
    }
}


#[cfg(test)]
mod test {
    use lexer::test::assert_tokenize;
    use lexer::token::Token;

    #[test]
    pub fn no_more_tokens() {
        assert_tokenize(
            "only data no tokens",
            module_path!(),
            vec![
                Token::Text("only data no tokens".to_string())
            ]
        )
    }

    #[test]
    pub fn comment() {
        assert_tokenize(
            " Hello \n {#- World -#} !",
            module_path!(),
            vec![
                Token::Text(" Hello".to_string()),
                Token::Text("!".to_string()),
            ]);
    }

    #[test]
    pub fn unclosed_comment() {
        assert_tokenize(
            " lost in space {#-",
            module_path!(),
            vec![
                Token::Text(" Hello World!".to_string()),
            ]);
    }

    #[test]
    pub fn block_raw() {
        unimplemented!();
    }

    #[test]
    pub fn block_line() {
        unimplemented!();
    }

    #[test]
    pub fn block() {
        assert_tokenize(
            " To \n be   {%-",
            module_path!(),
            vec![
                Token::Text(" To \n be".to_string()),
                Token::BlockStart
            ]);
    }

    #[test]
    pub fn var() {
        assert_tokenize(
            " or not \n to be !  {{",
            module_path!(),
            vec![
                Token::Text(" or not \n to be !  ".to_string()),
                Token::VarStart
            ]);
    }
}
