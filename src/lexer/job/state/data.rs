// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Data state of the lexer.

use super::{TokenizeState, Code};
use lexer::job::state;
use lexer::job::Job;
use lexer::token::Token;
use lexer::patterns::{token_start, verbatim_start, Extract};
use lexer::error::{LexerError, SyntaxError, SyntaxErrorCode};
use error::api::Dump;

pub struct Data;

#[allow(unused_variables)]
impl TokenizeState for Data {
    fn state() -> Code {
        Code::Data
    }

    fn tokenize<'a>(job: &'a mut Job) -> Result<(),LexerError> {
        let capture = match Self::next_token_start_after_cursor(job) {
            Some(capture) => capture,
            None => {
                let slice = job.cursor.slice_to_end();
                job.push_token(Token::Text(slice.to_string()));

                return state::Final::tokenize(job)
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
                try_chain!(Self::lex_comment(job));

                return Self::tokenize(job);
            },
            token_start::Tag::Block => {
                // #TODO:300 nested matches don't look nice - alternative??
                // raw data?
                match job.patterns.verbatim_start.extract(job.cursor.tail()) {
                    Some(verbatim_start) => {
                        job.cursor.move_by(verbatim_start.position.1);
                        try_chain!(Self::lex_verbatim_data(job, verbatim_start.tag));

                        return Self::tokenize(job);
                    },
                    _ => {
                        // {% line \d+ %}
                        match job.patterns.block_line.extract(job.cursor.tail()) {
                            Some(block_line) => {
                                let block_line = try!(block_line);

                                job.cursor.move_by(block_line.position.1);
                                job.cursor.set_line(block_line.line);

                                return Self::tokenize(job);
                            },
                            _ => {
                                job.current_exp_block_line = job.cursor.line();
                                job.push_token(Token::BlockStart);
                                try!(state::Block::tokenize(job));

                                return Self::tokenize(job);
                            },
                        }
                    },
                }
            },
            token_start::Tag::Expression => {
                job.current_exp_block_line = job.cursor.line();
                job.push_token(Token::ExpressionStart);
                try!(state::Expression::tokenize(job));

                return Self::tokenize(job);
            }
        }
    }
}

impl<'a> Data {
    fn lex_comment(job: &'a mut Job) -> Result<(), SyntaxError> {
        match job.patterns.comment_end.find(job.cursor.tail()) {
            None => return err!(SyntaxErrorCode::UnclosedComment {
                cursor: job.cursor.dump()
            }),
            Some(position) => job.cursor.move_by(position.1),
        }
        Ok(())
    }

    fn lex_verbatim_data(job: &'a mut Job, tag: verbatim_start::Tag) -> Result<(), SyntaxError> {
        let capture = match job.patterns.verbatim_end.extract_iter(job.cursor.tail())
                                                 .find(|capture| capture.tag == tag) {
            Some(capture) => capture,
            _ => return err!(SyntaxErrorCode::UnexpectedEof {
                    reason: "Unclosed (raw|verbatim) block",
                    cursor: job.cursor().dump()
                }),
        };

        let mut slice = job.cursor.slice_by(capture.position.0);
        job.cursor.move_by(capture.position.1 - capture.position.0);

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
    use lexer::test::tokenize_err;
    use lexer::test::assert_tokenize;
    use lexer::token::Token;
    use std::error::Error;
    use lexer::SyntaxErrorCode;

    #[test]
    pub fn no_more_tokens() {
        assert_tokenize(
            "only data no tokens",
            vec![
                Token::Text("only data no tokens".to_string())
            ]
        )
    }

    #[test]
    pub fn comment() {
        assert_tokenize(
            " Hello \n {#- World -#} !",
            vec![
                Token::Text(" Hello".to_string()),
                Token::Text("!".to_string()),
            ]);
    }

    #[test]
    pub fn unclosed_comment() {
        let cursor_dump = "`test-example` line 1 column 19".to_string();
        let expect = SyntaxErrorCode::UnclosedComment {
            cursor: cursor_dump
        }.to_string();

        let err = tokenize_err(" lost in space {#-").cause().unwrap().to_string();
        assert!(err.starts_with(&expect), "error {} should start with {}", err, expect);
    }

    #[test]
    pub fn block_verbatim() {
        assert_tokenize(
            "{% verbatim %}<ul>{% for x in list %}<li>{{ x }}</li>{% endfor %}</ul>{% endverbatim %}",
            vec![
                Token::Text("".to_string()),
                Token::Text("<ul>{% for x in list %}<li>{{ x }}</li>{% endfor %}</ul>".to_string()),
                Token::Text("".to_string()),
            ]);
    }

    #[test]
    pub fn block_line() {
        let cursor_dump = "`test-example` line 100 column 9".to_string();
        let expect = SyntaxErrorCode::UnclosedBlock {
            cursor: cursor_dump
        }.to_string();

        let err = tokenize_err("line1 {% line 99 %}\nline 2{%").cause().unwrap().to_string();
        assert!(err.starts_with(&expect), "error {} should start with {}", err, expect);
    }

    #[test]
    pub fn block() {
        assert_tokenize(
            " To \n be   {%- something -%}  ",
            vec![
                Token::Text(" To \n be".to_string()),
                Token::BlockStart,
                Token::Name("something".to_string()),
                Token::BlockEnd,
                Token::Text("".to_string()),
            ]);
    }

    #[test]
    pub fn unclosed_block() {
        let cursor_dump = "`test-example` line 2 column 10".to_string();
        let expect = SyntaxErrorCode::UnclosedBlock {
            cursor: cursor_dump
        }.to_string();

        let err = tokenize_err(" To \n be   {%-").cause().unwrap().to_string();
        assert!(err.starts_with(&expect), "error {} should start with {}", err, expect);
    }

    #[test]
    pub fn var() {
        assert_tokenize(
            " foo bar  {{ x }} baz",
            vec![
                Token::Text(" foo bar  ".to_string()),
                Token::ExpressionStart,
                Token::Name("x".to_string()),
                Token::ExpressionEnd,
                Token::Text(" baz".to_string()),
            ]);
    }

    #[test]
    pub fn unclosed_var() {
        let cursor_dump = "`test-example` line 2 column 13".to_string();
        let expect = SyntaxErrorCode::UnclosedVariable {
            cursor: cursor_dump
        }.to_string();

        let err = tokenize_err(" or not \n to be !  {{").cause().unwrap().to_string();
        assert!(err.starts_with(&expect), "error {} should start with {}", err, expect);
    }
}
