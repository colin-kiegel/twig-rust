// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! A lexer job - modeled as a FSM (Finite State Machine).

use std::fmt;
use template;
use engine::parser::lexer::patterns::{token_start, Extract};
use engine::parser::lexer::Patterns;
use engine::parser::token::{self, Token, BracketType};
use engine::parser::lexer::error::LexerError;
use self::state::TokenizeState;
use api::error::Traced;

pub mod state;
pub mod cursor;
pub use self::cursor::Cursor;

// Finite State Machine loosely inspired by
// * http://www.huffingtonpost.com/damien-radtke/rustic-state-machines-for_b_4466566.html

pub struct Job<'c, 't> {
    patterns: &'c Patterns,
    _template: &'t template::Raw,
    current_exp_block_line: usize,
    tokens: token::Stream<'t>,
    cursor: Cursor<'t>,
    _position: usize,
    token_start_iter: token_start::ExtractIter<'c, 't>, // orig: positions
    brackets: Vec<(BracketType, usize /* TODO LineNo */)>,
}

impl<'c, 't> Job<'c, 't> {
    #[allow(dead_code)] // TODO: testcase
    pub fn new(template: &'t template::Raw, patterns: &'c Patterns) -> Box<Job<'c, 't>> {
        let token_start_iter = patterns.token_start.extract_iter(&template.code);
        let cursor = Cursor::new(&template);
        let tokens = token::Stream::new(&template);

        Box::new(Job {
            patterns: patterns,
            _template: template,
            tokens: tokens,
            cursor: cursor,
            token_start_iter: token_start_iter,
            _position: 0,
            current_exp_block_line: 0,
            brackets: Vec::default(),
        })
    }

    #[allow(dead_code)] // TODO: testcase
    pub fn tokenize(mut self) -> Result<token::Stream<'t>, Traced<LexerError>> {
        // The TokenizeStates call each other *recursively* to avoid dynamic dispatch
        // for better performance. However, we loose debugging information about the
        // nesting of lexer states.
        try_traced!(state::Initial::tokenize(&mut self));
        // TODO: check whether we returned from *final* state

        Ok(self.tokens)
    }

    pub fn push_bracket(&mut self, bracket: (BracketType, usize)) {
        self.brackets.push(bracket)
    }

    pub fn pop_bracket(&mut self) -> Option<(BracketType, usize)> {
        self.brackets.pop()
    }

    // Only needed for the states of the job
    // - #TODO:180 does it make sense to put `push_token` in a trait,
    //   only visible to the states, i.e. hiding it from clients?
    pub fn push_token(&mut self, token: token::Token) {
        // TODO: sometime in the future: cow<_>
        // * check if the template can be disassembled into string-objects without
        //   copying - i.e. without calling to_string(&str)

        self.tokens.push(token, &self.cursor);
    }

    pub fn cursor(&self) -> &Cursor<'t> {
        &self.cursor
    }
}

// TODO: switch to Debug-Builder once stable
impl<'c, 't> fmt::Debug for Job<'c, 't> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f,
               "[\nCursor: {cursor}\nTokenstream: {tokens:?}\nBrackets: {brackets:?}\n]",
               cursor = self.cursor,
               tokens = self.tokens,
               brackets = self.brackets)
    }
}
