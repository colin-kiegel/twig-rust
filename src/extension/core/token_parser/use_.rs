// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use extension::api::TokenParser;
use engine::Node;
use engine::parser::{Job, ParserError};
use engine::parser::token::stream::Item;

#[derive(Debug, Default)]
pub struct Use;

impl TokenParser for Use {
    fn tag(&self) -> &'static str {
        "use"
    }

    fn parse(&self, _job: &mut Job, _item: &Item) -> Result<Box<Node>, ParserError> {
        unimplemented!()
    }
}
