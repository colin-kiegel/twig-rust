// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Final state of the lexer job.

use super::{TokenizeState, Code};
use lexer::error::LexerError;
use lexer::job::Job;

#[allow(dead_code)] // dummy
pub struct Final;

impl TokenizeState for Final {
    fn state() -> Code {
        Code::Final
    }

    fn tokenize<'a>(_job: &'a mut Job) -> Result<(),LexerError> {
        // #TODO:170 do some final checks like
        // - job.cursor.position() == job.cursor.end() <- implicit alternative?
        Ok(()) // means we are done.
        /*
        if (empty($this->brackets) && preg_match($this->regexes['lex_Final'], $this->code, $match, null, $this->cursor)) {
            $this->pushToken(Twig_Token::Final_END_TYPE);
            $this->moveCursor($match[0]);
            $this->popState();
        } else {
            $this->lexExpression();
        }
        */

                //println!("matcher {:?}", self.patterns.tokens_start);
        //println!("count {:?}", self.token_iter.unwrap().count());
        /*for slice in self.patterns.tokens_start.find_iter(&template.code) {
            let (start,end) = slice;
            let token = &template.code[start..end];

            println!("{:?}-{:?} = {:?}", start, end, token);
        }

        tokens.push(Token::new(
            token::Type::Eof,
            token::Value("".to_string()),
            cursor.get_position()
        ));

        if !self.brackets.is_empty() {
            //let (bracket, line) : (&str, usize) = self.brackets.pop().expect("fatal");

            //let a = error::syntax::Code::UnclosedBracket::new();

            //return a;
            //return Error::new(
              //  a,
                //format!("Unclosed {}", bracket),
                // ($line, $this->filename);
        //    );
        }*/
    }
}
