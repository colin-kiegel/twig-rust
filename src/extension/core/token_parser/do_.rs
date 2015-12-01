// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use extension::api::TokenParser;
use engine::Node;
use engine::parser::{self, Job, ParserError};
use engine::parser::token::stream::Item;

#[derive(Debug, Default)]
pub struct Do;

impl TokenParser for Do {
    fn tag(&self) -> &'static str {
        "do"
    }

    fn parse(&self, _job: &mut Job, _item: &Item) -> Result<Box<Node>, ParserError> {
        // let expr = _parser.expression_parser().parse(); // #TODO:160 collapse to parse_expression()
        // _parser.stream().expect(Token::BlockEnd); // #TODO:110 check if it ends DO-block
        // let node = parser::node::Do::new(expr, token->line(), self.tag());
        //
        // return Box::new(node);

        unimplemented!()
    }
}
