/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * A parser job
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use std::fmt;
use lexer::token;
use parser::error::*;
use parser::Parser;

/////////////
// exports //
/////////////


pub struct Job<'a> {
    // NOT-STACK:
    _parser: &'a Parser,    // orig: env
    // stack,
    // handlers,
    // visitors,
    // expressionParser,

    // STACK:
    tokens: &'a token::Stream<'a>, // orig: stream // really stacked???
    // parent,
    // blocks,
    // blockStack,
    // macros,
    // reservedMacroNames,
    // importedSymbols,
    // traits,
    // embeddedTemplates,
}

impl<'a> Job<'a> {
    #[allow(dead_code)] // TODO testcase
    pub fn new(tokens: &'a token::Stream, parser: &'a Parser) -> Job<'a> {
        Job {
            tokens: tokens,
            _parser: parser,
        }
    }

    #[allow(unused_mut)]
    #[allow(dead_code)] // TODO testcase
    pub fn parse(mut self: Job<'a>) -> Result<(), ParserError> {
        unimplemented!()
    }
}

// TODO switch to Debug-Builder once stable
impl<'a> fmt::Debug for Job<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "[\n\
            Tokenstream: {tokens}\n\
            ]",
            tokens = self.tokens
        )
    }
}
