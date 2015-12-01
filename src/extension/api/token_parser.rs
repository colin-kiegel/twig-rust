// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Extension `token parser` definition.

use std::fmt::Debug;

use engine::Node;
use engine::parser::{self, Job, ParserError};
use engine::parser::token::stream::Item;

pub trait TokenParser : Debug {
    fn tag(&self) -> &'static str;

    fn parse(&self, job: &mut Job, item: &Item) -> Result<Box<Node>, ParserError>;
}

// TODO: move {Test, TestResult} to engine::extension::api::test?
pub type Test = Fn(&Item) -> TestResult;

#[derive(Debug)]
pub enum TestResult {
    Continue,  // orig: no_match
    KeepToken, // orig: is_match + dropNeedle == false
    DropToken, // orig: is_match + dropNeedle == true
    //Error(ParserError) // *unstable* - not clear whether we need this
}
