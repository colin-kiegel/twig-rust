// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use extension::api::TokenParser;
use engine::Node;
use engine::parser::{Job, ParserError};
use engine::parser::token::stream::Item;
use api::error::Traced;

#[derive(Debug, Default)]
pub struct For;

impl TokenParser for For {
    fn tag(&self) -> &'static str {
        "for"
    }

    fn parse(&self, _job: &mut Job, _item: &Item) -> Result<Box<Node>, Traced<ParserError>> {
        unimplemented!()
    }
}
