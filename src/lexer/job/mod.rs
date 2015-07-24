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

/////////////
// imports //
/////////////

use std::fmt;
use template;
use lexer::patterns::{token_start, Extract};
use lexer::Patterns;
use lexer::token::{self, Token, BracketType};
use lexer::error::LexerError;
use self::state::TokenizeState;

/////////////
// exports //
/////////////

pub mod state;


// Finite State Machine loosely inspired by
// * http://www.huffingtonpost.com/damien-radtke/rustic-state-machines-for_b_4466566.html

pub struct Job<'a> {
    patterns: &'a Patterns,
    _template: &'a template::Raw,
    current_exp_block_line: usize,
    tokens: token::Stream<'a>,
    cursor: template::raw::Cursor<'a>,
    _position: usize,
    token_start_iter: token_start::ExtractIter<'a, 'a>, // orig: positions
    brackets: Vec<(BracketType, usize/*TODO LineNo*/)>,
}

impl<'a> Job<'a> {
    #[allow(dead_code)] // TODO testcase
    pub fn new(template: &'a template::Raw, patterns: &'a Patterns) -> Box<Job<'a>> {
        let token_start_iter = patterns.token_start.extract_iter(&template.code);
        let cursor = template::raw::Cursor::new(&template);
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

    #[allow(dead_code)] // TODO testcase
    pub fn tokenize(mut self: Job<'a>) -> Result<token::Stream<'a>, LexerError> {
        // The TokenizeStates call each other *recursively* to avoid dynamic dispatch
        // for better performance. However, we loose debugging information about the
        // nesting of lexer states.
        try!(state::Initial::tokenize(&mut self));
        // TODO check whether we returned from *final* state

        Ok(self.tokens)
    }

    pub fn push_bracket(&mut self, bracket: (BracketType, usize)) {
        self.brackets.push(bracket)
    }

    pub fn pop_bracket(&mut self) -> Option<(BracketType, usize)> {
        self.brackets.pop()
    }

    // Only needed for the states of the job
    // - TODO: does it make sense to put `push_token` in a trait,
    //   only visible to the states, i.e. hiding it from clients?
    pub fn push_token(&mut self, token: token::Token) {
        // TODO sometime in the future: cow<_>
        // * check if the template can be disassembled into string-objects without
        //   copying - i.e. without calling to_string(&str)

        let position = self.cursor.position();
        self.tokens.push(token, position);
    }
}

// TODO switch to Debug-Builder once stable
impl<'a> fmt::Debug for Job<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "[\n\
            Cursor: {cursor}\n\
            Tokenstream: {tokens}\n\
            Brackets: {brackets:?}\n\
            ]",
            cursor = self.cursor,
            tokens = self.tokens,
            brackets = self.brackets)
    }
}
