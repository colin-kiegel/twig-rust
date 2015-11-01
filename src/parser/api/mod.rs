// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Twig library for rust
///
/// @author Colin Kiegel <kiegel@gmx.de>


/////////////
// imports //
/////////////

use super::error::NodeError;
use std::fmt::Debug;
use lexer::token::stream::Position;
use runtime::{Runtime, Job};

/////////////
// exports //
/////////////


pub trait Node : Debug {
    fn tag(&self) -> &str;
    fn position(&self) -> &Position;
    fn children(&self) -> &Vec<Box<Node>>;
    fn children_mut(&mut self) -> &mut Vec<Box<Node>>;
    fn has_attribute(&self, key: &str) -> bool;
    fn attribute(&self, key: &str) -> Result<&str, NodeError>;
    fn set_attribute(&mut self, key: &str, value: &str) -> Option<String>;
    fn rm_attribute(&mut self, key: &str) -> Option<String>;
    // NOTE: Can't use generic trait `runtime::api::DataProvider`
    //      because a generic function would not be object safe.
    //      Thus we restrict to HashMap first.
    fn run(&self, runtime: &Runtime, job: &mut Job);
}

// // TODO: check if this is really necessary??
// //  - looks *very* weird.
// //  - otherwise use deref
// #[allow(unused_variables)]
// impl<X> Node for Box<X> where
//     X: Node
// {
//     fn tag(&self) -> &str { unimplemented!() }
//     fn position(&self) -> &Position { unimplemented!() }
//     fn children(&self) -> &Vec<Box<Node>> { unimplemented!() }
//     fn children_mut(&mut self) -> &mut Vec<Box<Node>> { unimplemented!() }
//     fn has_attribute(&self, key: &str) -> bool { unimplemented!() }
//     fn attribute(&self, key: &str) -> Result<&str, NodeError> { unimplemented!() }
//     fn set_attribute(&mut self, key: &str, value: &str) -> Option<String> { unimplemented!() }
//     fn rm_attribute(&mut self, key: &str) -> Option<String> { unimplemented!() }
//     fn run(&self, runtime: &mut Runtime) { (**self).run(runtime) }
// }
