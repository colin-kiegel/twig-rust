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

use super::*;
use lexer::SyntaxError;
use lexer::job::Job;
use super::_final::Final;
use lexer::token::Token;
use lexer::patterns::token_start::CaptureData;
use lexer::patterns::Extract;

/////////////
// exports //
/////////////

#[derive(Debug)]
#[allow(dead_code)]
pub struct Data;

#[allow(unused_variables)]
impl Tokenize for Data {
    fn new() -> Box<Self> {
        Box::new(Data)
    }

    fn get_type(&self) -> Code {
        Code::Data
    }

    fn step<'a>(&self, job: &mut Job<'a>) -> Result<Box<Tokenize>,SyntaxError> {
        // if no matches are left we return the rest of the template as simple text token
        if job.token_iter.peek().is_none() {
            let slice = job.cursor.slice_to_end().to_string();
            job.push_token(Token::Text(slice));

            return Ok(Final::new())
        }

        // Find the first token after the current cursor
        let ref captures = job.next_token_after_cursor().expect("no token matches left - this should not happen");
        let data : CaptureData = job.patterns.token_start.extract(captures);

        let whitespace_trim : bool = match job.token_iter.peek() {
            None             => false,
            Some(captures)   => {
                let next_token = job.patterns.token_start.extract(captures);

                next_token.whitespace_trim
            },
        };


        //let (start, end) = match job.next_token_after_cursor() {
          //  Some(x) => x,

            // TODO if the panic does *not* occur, we might safely merge this with above 'if no matches left'
            // *otherwise* check first, how orig engine behaves in this case
     //   };

/*
        // push the template text first
        $text = $textContent = substr($this->code, $this->cursor, $position[1] - $this->cursor);
        if (isset($this->positions[2][$this->position][0])) {
            $text = rtrim($text);
        }
        $this->pushToken(Twig_Token::TEXT_TYPE, $text);
        $this->moveCursor($textContent.$position[0]);

        switch ($this->positions[1][$this->position][0]) {
            case $this->options['tag_comment'][0]:
                $this->lexComment();
                break;

            case $this->options['tag_block'][0]:
                // raw data?
                if (preg_match($this->regexes['lex_block_raw'], $this->code, $match, null, $this->cursor)) {
                    $this->moveCursor($match[0]);
                    $this->lexRawData($match[1]);
                // {% line \d+ %}
                } elseif (preg_match($this->regexes['lex_block_line'], $this->code, $match, null, $this->cursor)) {
                    $this->moveCursor($match[0]);
                    $this->lineno = (int) $match[1];
                } else {
                    $this->pushToken(Twig_Token::BLOCK_START_TYPE);
                    $this->pushState(self::STATE_BLOCK);
                    $this->currentVarBlockLine = $this->lineno;
                }
                break;

            case $this->options['tag_variable'][0]:
                $this->pushToken(Twig_Token::VAR_START_TYPE);
                $this->pushState(self::STATE_VAR);
                $this->currentVarBlockLine = $this->lineno;
                break;
        }
        */
        unimplemented!();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::rc::Rc;
    use lexer::job::Job;
    use template::raw::Raw as Template;
    use lexer::Patterns;
    use lexer::job::state::{Tokenize, Code};
    use lexer::token::Token;

    #[test]
    pub fn no_more_tokens() {
        const CODE : &'static str = "only data no tokens";
        const FILENAME : &'static str = "only data";

        let ref template = Rc::new(Template::new(CODE, FILENAME));
        let ref patterns = Rc::new(Patterns::default());
        let mut job = Job::new(template, patterns);

        let state = Data::new().step(&mut job).unwrap();

        if !state.is_type(Code::Final) {
            panic!("not final state");
        }

        println!("tokens are: {}", job.tokens.to_string());

        assert_eq!(job.tokens.len(), 1);

        let t_x = Token::Text(CODE.to_string());
        let t_o : Token = job.tokens.into_iter().last().unwrap().into();

        assert_eq!(t_o, t_x);
    }
}
