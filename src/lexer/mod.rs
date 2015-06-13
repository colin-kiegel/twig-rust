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

// outer
use std::rc::Rc;
use template;
use environment::Environment;
use error;

// exports
pub mod syntax_error;
pub mod options;
pub use self::options::Options;
pub use self::syntax_error::Code as SyntaxErrorCode;
pub type SyntaxError = error::Error<SyntaxErrorCode>;

// inner
#[cfg(test)]
mod test;
mod state;
mod regex_patterns;
mod token;
use self::token::Token;
use self::state::State;
use self::regex_patterns::RegexPatterns;

// TODO: where does this belong?
//const PUNCTUATION           : &'static str = "()[]{}?:.,|";

#[allow(dead_code)]
//#[derive(Default)]
struct Lexer {
    env: Rc<Environment>,
    options: Rc<Options>,
    patterns: RegexPatterns,
    template: Option<Rc<template::Raw>>,
    stream: Option<Rc<token::Stream>>,
    cursor: Option<Rc<template::raw::Cursor>>,
    position: Option<usize>,
    positions: Vec<usize>,
    state: State,
    states: Vec<State>,
    brackets: Vec<(&'static str/*TODO reduce lifetime*/, usize/*TODO LineNo*/)>,
    current_var_block_line: usize,
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl Lexer {
    pub fn new(env: Environment, opt: Options) -> Lexer {
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
            positions: Vec::default(),
            state: State::default(),
            states: Vec::default(),
            brackets: Vec::default(),
            current_var_block_line: 0,
        }
    }
    
    pub fn tokenize(&mut self, template: Rc<template::Raw>) -> Result<Rc<token::Stream>, SyntaxError> {
        // TODO set/handle encoding (note: Twig-PHP assumes ASCII)
        
        let cursor = Rc::new(template::raw::Cursor::new(template.clone()));
        let mut tokens = Vec::<Token>::new();
        
        self.reset();
        self.template = Some(template.clone());
        self.cursor = Some(cursor.clone());
        
        println!("Starting with {:?}", cursor);
        
        // find all token starts in one go
        // let positions = self.patterns.tokens_start.find_iter(&template.code);
        
        // PHP: preg_match_all(self.patterns.tokens_start, self.code, matches, PREG_OFFSET_CAPTURE);
        // TODO self.positions = positions;
        
        while !cursor.is_eof() {
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
