/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * A lexer job - modeled as a FSM (Finite State Machine).
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

// ## exports ##
pub mod state;

// ## imports ##
use regex;
use template;
use std::rc::Rc;
use lexer::regex_patterns::RegexPatterns;
use lexer::token::Token;
use lexer::token;
use lexer::SyntaxError;
use self::state::Tokenize;

// Finite State Machine inspired by http://www.huffingtonpost.com/damien-radtke/rustic-state-machines-for_b_4466566.html
    
#[allow(dead_code)]
pub struct Job<'a> {
    current_var_block_line: usize,
    patterns: Rc<RegexPatterns>,
    template: Rc<template::Raw>,
    tokens: token::Stream,
    cursor: Rc<template::raw::Cursor>,
    position: usize,
    token_iter: regex::FindMatches<'a ,'a >, // orig: positions
    brackets: Vec<(&'a str, usize/*TODO LineNo*/)>,
    //states: Vec<State>, // or codes?
}

#[allow(dead_code)]
impl<'a> Job<'a> {
    pub fn new(template: &'a Rc<template::Raw>, patterns: &'a Rc<RegexPatterns>) -> Box<Job<'a>> {
            // find all token starts in one go:
            let token_iter = patterns.tokens_start.find_iter(&template.code);
            let cursor = Rc::new(template::raw::Cursor::new(template.clone()));
            let tokens = token::Stream::new(template.clone());
            println!("Starting with {:?}", cursor);

        Box::new(Job {
            patterns: patterns.clone(),
            template: template.clone(),
            tokens: tokens,
            cursor: cursor,
            token_iter: token_iter,
            position: Default::default(),
            brackets: Default::default(),
            current_var_block_line: Default::default(),
            //states: Vec::default(),
        })
    }
    
    pub fn tokenize(mut self) -> Result<token::Stream, SyntaxError> {
        let mut tokenizer : Box<Tokenize> = state::Initial::new();

        while !tokenizer.is_finished() {
            match tokenizer.step(&mut self) {
                Ok(new_state) => tokenizer = new_state,
                Err(e) => {
                    return Err(e); // TODO wrap the error?
                }
            }
        }
        
        Ok(self.tokens)
    }
    
    pub fn push_token(&mut self, token: token::Token) {
        // TODO sometime in the future:
        // * check if the template can be disassembled into string-objects without
        //   copying - i.e. without calling to_string(&str)

        let position = self.cursor.get_position();
        self.tokens.push(token, position);
    }
}