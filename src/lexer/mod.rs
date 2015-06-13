/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Lexes a template string.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

// exports
pub mod state;
pub mod options;
pub use self::options::Options;

// imports
#[cfg(test)]
mod test;
mod regex_patterns;
mod token;
use self::token::Token;
use self::state::State;
use self::regex_patterns::RegexPatterns;
use std::rc::Rc;
use template;
use environment::Environment;
use regex;

// TODO: where does this belong?
//const PUNCTUATION           : &'static str = "()[]{}?:.,|";

#[allow(dead_code)]
//#[derive(Default)]
struct Lexer<'t> {
    env: Rc<Environment>,
    options: Rc<Options>,
    patterns: RegexPatterns,
    template: Option<Rc<template::Raw>>,
    stream: Option<Rc<token::Stream>>,
    cursor: Option<Rc<template::raw::Cursor>>,
    position: Option<usize>,
    token_iter: Option<regex::FindMatches<'t ,'t >>, // orig: positions
    state: Option<Box<State>>,
    //states: Vec<State>,
    brackets: Vec<(&'static str/*TODO reduce lifetime*/, usize/*TODO LineNo*/)>,
    current_var_block_line: usize,
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl<'t, 'r> Lexer<'t> {
    pub fn new(env: Environment, opt: Options) -> Lexer<'static> {
        let env = Rc::new(env);
        let opt = Rc::new(opt);
        let patterns = RegexPatterns::new(env.clone(), opt.clone())
        .unwrap(); // TODO Error-Handling
        
        Lexer {
            env: env,
            options: opt,
            patterns: patterns,
            template: None,
            stream: None,
            cursor: None,
            position: None,
            token_iter: None,
            state: None,//State::default(),
            //states: Vec::default(),
            brackets: Vec::default(),
            current_var_block_line: 0,
        }
    }
    
    // inspired by http://www.huffingtonpost.com/damien-radtke/rustic-state-machines-for_b_4466566.html
    pub fn lex(&mut self, state: Box<State>) -> Result<Box<State>,state::SyntaxError> {
        Ok(match try!(state.lex()) {
            Some(new_state) => new_state,
            None => state
        })
    }
    
    pub fn tokenize(&'t mut self, template: &'r Rc<template::Raw>) -> Result<Rc<token::Stream>, state::SyntaxError>
        where 'r: 't // the template must outlive the Lexer
    {
        // TODO set/handle encoding (note: Twig-PHP assumes ASCII)
        
        let cursor = Rc::new(template::raw::Cursor::new(template.clone()));
        let mut tokens = Vec::<Token>::new();
        
        self.reset();
        self.template = Some(template.clone());
        self.cursor = Some(cursor.clone());
        
        println!("Starting with {:?}", cursor);
        
        // find all token starts in one go            
        self.token_iter = Some(self.patterns.tokens_start.find_iter(&template.code));
        // orig: self.positions = preg_match_all(self.patterns.tokens_start, self.code, matches, PREG_OFFSET_CAPTURE);
        
        // TODO switch to state machine?
        // i.e. promote the state to enum + Lex-trait [with internal data like Cursor, etc]
        /*while !cursor.is_eof() {
            match self.state {
                State::Data => self.lex_data(),
                State::Block => self.lex_block(),
                State::Var => self.lex_var(),
                State::String => self.lex_string(),
                State::Interpolation => self.lex_interpolation(),
            }
        }*/
        
        //println!("matcher {:?}", self.patterns.tokens_start);
        //println!("count {:?}", self.token_iter.unwrap().count());
        /*for slice in self.patterns.tokens_start.find_iter(&template.code) {
            let (start,end) = slice;
            let token = &template.code[start..end];
            
            println!("{:?}-{:?} = {:?}", start, end, token);
        }*/
        
        tokens.push(Token::new(
            token::Type::Eof,
            token::Value("".to_string())/*TODO val*/,
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
        }
        
        let stream = Rc::new(token::Stream::new(tokens, template.clone()));
        self.stream = Some(stream.clone());
        
        Ok(stream)
    }
    
    fn reset(&mut self) {
        self.template = None;
        self.stream = None;
        self.cursor = None;
        self.position = None;
        self.token_iter = None;
        //self.states.clear();
        self.brackets.clear();
        self.state = None;//State::Data;
    }
   
    
    fn lex_expression(&self) {
    /*
        // whitespace
        if (preg_match('/\s+/A', $this->code, $match, null, $this->cursor)) {
            $this->moveCursor($match[0]);

            if ($this->cursor >= $this->end) {
                throw new Twig_Error_Syntax(sprintf('Unclosed "%s"', $this->state === self::STATE_BLOCK ? 'block' : 'variable'), $this->currentVarBlockLine, $this->filename);
            }
        }

        // operators
        if (preg_match($this->regexes['operator'], $this->code, $match, null, $this->cursor)) {
            $this->pushToken(Twig_Token::OPERATOR_TYPE, preg_replace('/\s+/', ' ', $match[0]));
            $this->moveCursor($match[0]);
        }
        // names
        elseif (preg_match(self::REGEX_NAME, $this->code, $match, null, $this->cursor)) {
            $this->pushToken(Twig_Token::NAME_TYPE, $match[0]);
            $this->moveCursor($match[0]);
        }
        // numbers
        elseif (preg_match(self::REGEX_NUMBER, $this->code, $match, null, $this->cursor)) {
            $number = (float) $match[0];  // floats
            if (ctype_digit($match[0]) && $number <= PHP_INT_MAX) {
                $number = (int) $match[0]; // integers lower than the maximum
            }
            $this->pushToken(Twig_Token::NUMBER_TYPE, $number);
            $this->moveCursor($match[0]);
        }
        // punctuation
        elseif (false !== strpos(self::PUNCTUATION, $this->code[$this->cursor])) {
            // opening bracket
            if (false !== strpos('([{', $this->code[$this->cursor])) {
                $this->brackets[] = array($this->code[$this->cursor], $this->lineno);
            }
            // closing bracket
            elseif (false !== strpos(')]}', $this->code[$this->cursor])) {
                if (empty($this->brackets)) {
                    throw new Twig_Error_Syntax(sprintf('Unexpected "%s"', $this->code[$this->cursor]), $this->lineno, $this->filename);
                }

                list($expect, $lineno) = array_pop($this->brackets);
                if ($this->code[$this->cursor] != strtr($expect, '([{', ')]}')) {
                    throw new Twig_Error_Syntax(sprintf('Unclosed "%s"', $expect), $lineno, $this->filename);
                }
            }

            $this->pushToken(Twig_Token::PUNCTUATION_TYPE, $this->code[$this->cursor]);
            ++$this->cursor;
        }
        // strings
        elseif (preg_match(self::REGEX_STRING, $this->code, $match, null, $this->cursor)) {
            $this->pushToken(Twig_Token::STRING_TYPE, stripcslashes(substr($match[0], 1, -1)));
            $this->moveCursor($match[0]);
        }
        // opening double quoted string
        elseif (preg_match(self::REGEX_DQ_STRING_DELIM, $this->code, $match, null, $this->cursor)) {
            $this->brackets[] = array('"', $this->lineno);
            $this->pushState(self::STATE_STRING);
            $this->moveCursor($match[0]);
        }
        // unlexable
        else {
            throw new Twig_Error_Syntax(sprintf('Unexpected character "%s"', $this->code[$this->cursor]), $this->lineno, $this->filename);
        }
    */    
    }
    
    fn lex_raw_data(&self) {
    /*
        if (!preg_match(str_replace('%s', $tag, $this->regexes['lex_raw_data']), $this->code, $match, PREG_OFFSET_CAPTURE, $this->cursor)) {
            throw new Twig_Error_Syntax(sprintf('Unexpected end of file: Unclosed "%s" block', $tag), $this->lineno, $this->filename);
        }

        $text = substr($this->code, $this->cursor, $match[0][1] - $this->cursor);
        $this->moveCursor($text.$match[0][0]);

        if (false !== strpos($match[1][0], $this->options['whitespace_trim'])) {
            $text = rtrim($text);
        }

        $this->pushToken(Twig_Token::TEXT_TYPE, $text);
    */    
    }
    
    fn lex_comment(&self) {
    /*
        if (!preg_match($this->regexes['lex_comment'], $this->code, $match, PREG_OFFSET_CAPTURE, $this->cursor)) {
            throw new Twig_Error_Syntax('Unclosed comment', $this->lineno, $this->filename);
        }

        $this->moveCursor(substr($this->code, $this->cursor, $match[0][1] - $this->cursor).$match[0][0]);
    }

    protected function lexString()
    {
        if (preg_match($this->regexes['interpolation_start'], $this->code, $match, null, $this->cursor)) {
            $this->brackets[] = array($this->options['interpolation'][0], $this->lineno);
            $this->pushToken(Twig_Token::INTERPOLATION_START_TYPE);
            $this->moveCursor($match[0]);
            $this->pushState(self::STATE_INTERPOLATION);
        } elseif (preg_match(self::REGEX_DQ_STRING_PART, $this->code, $match, null, $this->cursor) && strlen($match[0]) > 0) {
            $this->pushToken(Twig_Token::STRING_TYPE, stripcslashes($match[0]));
            $this->moveCursor($match[0]);
        } elseif (preg_match(self::REGEX_DQ_STRING_DELIM, $this->code, $match, null, $this->cursor)) {
            list($expect, $lineno) = array_pop($this->brackets);
            if ($this->code[$this->cursor] != '"') {
                throw new Twig_Error_Syntax(sprintf('Unclosed "%s"', $expect), $lineno, $this->filename);
            }

            $this->popState();
            ++$this->cursor;
        }
    */
    }
}
