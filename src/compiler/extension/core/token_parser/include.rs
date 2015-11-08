// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

///
///
/// @author Colin Kiegel <kiegel@gmx.de>

/////////////
// imports //
/////////////

use compiler::extension::api::TokenParser;
use parser::{self, Job, ParserError};
use lexer::token::stream::Item;

/////////////
// exports //
/////////////

#[derive(Debug, Default)]
pub struct Include;

impl TokenParser for Include {
    fn tag(&self) -> &'static str {
        "include"
    }

    fn parse(&self, _job: &mut Job, _item: &Item) -> Result<Box<parser::Node>, ParserError> {
        unimplemented!()
    }
}
