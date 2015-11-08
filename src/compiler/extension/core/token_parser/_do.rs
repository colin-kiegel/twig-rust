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
pub struct Do;

impl TokenParser for Do {
    fn tag(&self) -> &'static str {
        "do"
    }

    fn parse(&self, _job: &mut Job, _item: &Item) -> Result<Box<parser::Node>, ParserError> {
        // let expr = _parser.expression_parser().parse(); // #TODO:160 collapse to parse_expression()
        // _parser.stream().expect(Token::BlockEnd); // #TODO:110 check if it ends DO-block
        // let node = parser::node::Do::new(expr, token->line(), self.tag());
        //
        // return Box::new(node);

        unimplemented!()
    }
}
