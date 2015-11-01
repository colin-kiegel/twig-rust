// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Represents a node that executes an expression.
///
/// @author Colin Kiegel <kiegel@gmx.de>


//////////////
// imports  //
//////////////

use super::GenericNode;
use runtime::{Runtime, Execute, Job};
use lexer::token::stream::Position;
use parser::api::Node;
use std::clone::Clone;

/////////////
// exports //
/////////////

pub type Print = GenericNode<Data>;

#[derive(Debug, Default)]
pub struct Data;

impl Print {
    pub fn boxed(expr: Box<Node>, position: &Position) -> Box<Print> {
        Box::new(Print {
            position: (*position).clone(),
            nodes: vec![expr],
            ..GenericNode::default()
        })
    }
}

impl Execute for Print {
    fn execute(&self, runtime: &Runtime, job: &mut Job) {
        for node in &self.nodes {
             node.execute(runtime, job)
        }
    }
}
