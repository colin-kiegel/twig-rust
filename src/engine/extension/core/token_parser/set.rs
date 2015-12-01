// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use engine::extension::api::TokenParser;
use parser::{self, Job, ParserError};
use lexer::token::stream::Item;

#[derive(Debug, Default)]
pub struct Set;

impl TokenParser for Set {
    fn tag(&self) -> &'static str {
        "set"
    }

    fn parse(&self, _job: &mut Job, _item: &Item) -> Result<Box<parser::Node>, ParserError> {
        unimplemented!()
    }
}