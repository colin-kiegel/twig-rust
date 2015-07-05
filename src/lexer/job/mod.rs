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

use template;
use lexer::patterns::Extract;
use lexer::Patterns;
use lexer::token::Token;
use lexer::token;
use lexer::error::{LexerError, LexerErrorCode};
use self::state::TokenizeState;
use lexer::patterns::token_start;

/////////////
// exports //
/////////////

pub mod state;


// Finite State Machine loosely inspired by
// * http://www.huffingtonpost.com/damien-radtke/rustic-state-machines-for_b_4466566.html

#[allow(dead_code)]
pub struct Job<'a> {
    patterns: &'a Patterns,
    template: &'a template::Raw,
    current_var_block_line: usize,
    tokens: token::Stream<'a>,
    cursor: template::raw::Cursor<'a>,
    position: usize,
    token_start_iter: token_start::ExtractIter<'a, 'a>, // orig: positions
    brackets: Vec<(&'a str, usize/*TODO LineNo*/)>,
    states: Vec<&'static TokenizeState>,
}

#[allow(dead_code)]
impl<'a> Job<'a> {
    pub fn new(template: &'a template::Raw, patterns: &'a Patterns) -> Box<Job<'a>> {
        let token_start_iter = patterns.token_start.extract_iter(&template.code);
        let cursor = template::raw::Cursor::new(&template);
        let tokens = token::Stream::new(&template);

        Box::new(Job {
            patterns: patterns,
            template: template,
            tokens: tokens,
            cursor: cursor,
            token_start_iter: token_start_iter,
            position: 0,
            current_var_block_line: 0,
            brackets: Vec::default(),
            states: Vec::default(),
        })
    }


    pub fn tokenize(mut self: Job<'a>) -> Result<token::Stream<'a>, LexerError> {
        // The TokenizeStates call each other recursively to avoid dynamic dispatch
        // whenever possible. Dynamic dispatch is only needed after a previous state
        // is popped from the state stack. :-)
        //
        // NOTE: even that last dynamic dispatch is not needed anymore
        //       the general idea is to replace
        //          - `push_state(); return new_state().tokenize();`
        //       with
        //          - `return new_state().tokenize().and_then(|| self.tokenize());`
        //
        //       However the `pop_state()` calls are scattered accross curious places
        //       so this would be some quite risky refactoring. We also loose debugging
        //       information about the nesting of lexer states. It's not clear if it's
        //       worth it.
        //
        try!(state::Initial::instance().tokenize(&mut self)); // TODO wrap the error?

        Ok(self.tokens)
    }

    pub fn push_bracket(&mut self, bracket: (&'a str, usize)) {
        self.brackets.push(bracket)
    }

    pub fn pop_bracket(&mut self) -> Option<(&'a str, usize)> {
        self.brackets.pop()
    }

    // Only needed for the states of the job
    // - TODO: does it make sense to put it in a trait,
    //   only visible to the states, i.e. hiding it from clients?
    pub fn push_token(&mut self, token: token::Token) {
        // TODO sometime in the future:
        // * check if the template can be disassembled into string-objects without
        //   copying - i.e. without calling to_string(&str)

        let position = self.cursor.position();
        self.tokens.push(token, position);
    }

    /// Push previous state - to be able to
    pub fn push_state(&mut self, state: &'static TokenizeState) {
        self.states.push(state);
    }

    // Pop previous state
    pub fn pop_state(&mut self) -> Result<&'static TokenizeState, LexerError> {
        self.states.pop().ok_or(err!(LexerErrorCode::InvalidState, "No previous state!"))
    }
}
