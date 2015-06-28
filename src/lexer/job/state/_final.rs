/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Final state of the lexer job.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use super::*;
use lexer::SyntaxError;
use lexer::job::Job;

/////////////
// exports //
/////////////

#[derive(Debug)]
#[allow(dead_code)]
pub struct Final {
    is_finished: bool,
}

impl Tokenize for Final {
    fn new() -> Box<Self> {
        Box::new(Final{ is_finished : false })
    }

    fn get_type(&self) -> Code {
        Code::Final
    }

    fn is_finished(&self) -> bool {
        self.is_finished
    }

    // this one requires a read o
    // https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md
    // http://stackoverflow.com/questions/29985153/trait-object-is-not-object-safe-error
    //fn lex<T>(self: Box<Self>) -> Result<Box<Tokenize<'a>>,SyntaxError>;

    fn step<'a>(&self, _job: &mut Job<'a>) -> Result<Box<Tokenize>,SyntaxError> {
        Ok(Box::new(Final{ is_finished : true }))
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
            token::Value("".to_string()),//TODO val
            cursor.get_position()
        ));

        if !self.brackets.is_empty() {
            //let (bracket, lineno) : (&str, usize) = self.brackets.pop().expect("fatal");

            //let a = error::syntax::Code::UnclosedBracket::new();

            //return a;
            //return Error::new(
              //  a,
                //format!("Unclosed {}", bracket),
                // TODO ($lineno, $this->filename);
        //    );
        }*/
        //unimplemented!();
    }
}
