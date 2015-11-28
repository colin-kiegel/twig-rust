// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Extension `token parser` definition
///
/// @author Colin Kiegel <kiegel@gmx.de>

/////////////
// imports //
/////////////

use std::fmt::Debug;
use parser::{self, Job, ParserError};
use lexer::token::stream::Item;

/////////////
// exports //
/////////////

pub trait TokenParser : Debug {
    fn tag(&self) -> &'static str;

    fn parse(&self, job: &mut Job, item: &Item) -> Result<Box<parser::Node>, ParserError>;
}

// TODO: move {Test, TestResult} to compiler::extension::api::test?
pub type Test = Fn(&Item) -> TestResult;

#[derive(Debug)]
pub enum TestResult {
    Continue,  // orig: no_match
    KeepToken, // orig: is_match + dropNeedle == false
    DropToken, // orig: is_match + dropNeedle == true
    //Error(ParserError) // *unstable* - not clear whether we need this
}
