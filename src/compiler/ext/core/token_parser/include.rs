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

use compiler::ext::TokenParser;
use parser::{self, Parser};
use lexer;

/////////////
// exports //
/////////////

#[derive(Debug, Default)]
pub struct Include;

impl TokenParser for Include {
    fn tag(&self) -> &'static str {
        "include"
    }

    fn parse(&self, _parser: Parser, _token: lexer::Token) -> Box<parser::Node> {
        unimplemented!()
    }
}
