// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Twig library for rust.

use super::error::NodeError;
use std::fmt::Debug;
use lexer::token::stream::Position;
use runtime::api::Execute;

pub trait Node : Debug + Execute {
    fn tag(&self) -> &str;
    fn position(&self) -> &Position;
    fn children(&self) -> &Vec<Box<Node>>;
    fn children_mut(&mut self) -> &mut Vec<Box<Node>>;
    fn has_attribute(&self, key: &str) -> bool;
    fn attribute(&self, key: &str) -> Result<&str, NodeError>;
    fn set_attribute(&mut self, key: &str, value: &str) -> Option<String>;
    fn rm_attribute(&mut self, key: &str) -> Option<String>;
}
