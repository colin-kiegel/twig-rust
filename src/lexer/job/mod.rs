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

use regex;
use template;
use std::iter::Peekable;
use lexer::patterns::Extract;
use lexer::Patterns;
use lexer::token::Token;
use lexer::token;
use lexer::SyntaxError;
use self::state::Tokenize;

/////////////
// exports //
/////////////

pub mod state;


// Finite State Machine inspired by http://www.huffingtonpost.com/damien-radtke/rustic-state-machines-for_b_4466566.html

#[allow(dead_code)]
pub struct Job<'a> {
    patterns: &'a Patterns,
    template: &'a template::Raw,
    current_var_block_line: usize,
    tokens: token::Stream<'a>,
    cursor: template::raw::Cursor<'a>,
    position: usize,
    token_iter: Peekable<regex::FindCaptures<'a ,'a>>, // orig: positions
    brackets: Vec<(&'a str, usize/*TODO LineNo*/)>,
    //states: Vec<State>, // or codes?
}

#[allow(dead_code)]
impl<'a> Job<'a> {
    pub fn new(template: &'a template::Raw, patterns: &'a Patterns) -> Box<Job<'a>> {
            // find all token starts in one go:
            let token_iter = patterns.token_start.regex().captures_iter(&template.code);
            let cursor = template::raw::Cursor::new(template);
            let tokens = token::Stream::new(template);
            println!("Starting with {:?}", cursor);

        Box::new(Job {
            patterns: patterns.clone(),
            template: template.clone(),
            tokens: tokens,
            cursor: cursor,
            token_iter: token_iter.peekable(),
            position: Default::default(),
            brackets: Default::default(),
            current_var_block_line: Default::default(),
            //states: Vec::default(),
        })
    }

    pub fn tokenize(mut self: Job<'a>) -> Result<token::Stream<'a>, SyntaxError> {
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

    /// Find the first token after the current cursor
    pub fn next_token_after_cursor(&mut self) -> Option<regex::Captures<'a>> {
        let position = self.cursor.get_position();

        self.token_iter.find(|x| {
            let start = x.pos(0).expect("capture group must have a position").0;
            start >= position
        })
    }
}
