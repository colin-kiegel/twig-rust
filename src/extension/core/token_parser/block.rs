// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use extension::api::TokenParser;
use engine::Node;
use engine::parser::{self, Job, ParserError};
use engine::parser::token::stream::Item;

#[derive(Debug, Default)]
pub struct Block;

impl TokenParser for Block {
    fn tag(&self) -> &'static str {
        "block"
    }

    fn parse(&self, _job: &mut Job, _item: &Item) -> Result<Box<Node>, ParserError> {
        unimplemented!()
    }
}
