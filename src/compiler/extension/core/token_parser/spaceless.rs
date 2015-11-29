// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use compiler::extension::api::TokenParser;
use parser::{self, Job, ParserError};
use lexer::token::stream::Item;

#[derive(Debug, Default)]
pub struct Spaceless;

impl TokenParser for Spaceless {
    fn tag(&self) -> &'static str {
        "spaceless"
    }

    fn parse(&self, _job: &mut Job, _item: &Item) -> Result<Box<parser::Node>, ParserError> {
        unimplemented!()
    }
}
