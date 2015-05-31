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

pub mod options;
mod state;
mod regex_patterns;
mod token;

pub use self::options::Options;
pub use environment::Environment;
//use self::token as token;
use self::token::Token;
use self::state::State;
use self::regex_patterns::RegexPatterns;
use template;
use error;

use std::rc::Rc;

// TODO: where does this belong?
const PUNCTUATION           : &'static str = "()[]{}?:.,|";

#[derive(Default)]
struct Lexer {
    env: Environment,
    options: Rc<Options>,
    patterns: RegexPatterns,
    template: Option<Rc<template::Raw>>, // TODO reference with lifetime?
    stream: Option<Rc<token::Stream>>,
    cursor: Option<Rc<template::raw::Cursor>>,
    position: Option<usize>,
    positions: Vec<usize>,
    state: State,
    states: Vec<State>,
    brackets: Vec<(&'static str/*TODO reduce lifetime*/, usize/*TODO LineNo*/)>,
    currentVarBlockLine: usize,
}

impl Lexer {
    pub fn new(env: Environment, options: Options) -> Lexer {
        let opt = Rc::new(options);
        Lexer {
            env: env,
            patterns: RegexPatterns::new(opt.clone()),
            options: opt,
            .. Default::default()
        }
    }
    
    pub fn tokenize(&mut self, template: Rc<template::Raw>) -> Result<Rc<token::Stream>, error::aliases::SyntaxError> {
        // TODO set/handle encoding (note: Twig-PHP assumes ASCII)
        
        unimplemented!();
        
        self.reset();
        self.template = Some(template.clone());
        let mut tokens = Vec::<Token>::new();
        
        // find all token starts in one go
        // TODO preg_match_all(self.patterns.tokens_start, self.code, matches, PREG_OFFSET_CAPTURE);
        // self.positions = matches;
        
        while !self.cursor.clone().expect("fatal").is_eof() {
            match self.state {
                State::Data => self.lex_data(),
                State::Block => self.lex_block(),
                State::Var => self.lex_var(),
                State::String => self.lex_string(),
                State::Interpolation => self.lex_interpolation(),
            }
        }
        
        tokens.push(Token::new(
            token::Type::Eof,
            token::Value("".to_string())/*TODO val*/,
            self.cursor.clone().expect("fatal").get_position() 
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
        
        let stream = Rc::new(token::Stream::new(tokens, template));
        self.stream = Some(stream.clone());
        
        Ok(stream)
    }
    
    fn reset(&mut self) {
        self.template = None;
        self.stream = None;
        self.cursor = None;
        self.position = None;
        self.positions.clear();
        self.states.clear();
        self.brackets.clear();
        self.state = State::Data;
    }

    fn lex_data(&self) {
        unimplemented!();
    }
    
    fn lex_block(&self) {
        unimplemented!();
    }
    
    fn lex_var(&self) {
        unimplemented!();
    }
    
    fn lex_string(&self) {
        unimplemented!();
    }
    
    fn lex_interpolation(&self) {
        unimplemented!();
    }
}
