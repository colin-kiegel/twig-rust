// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Represents a node that executes an expression.

use super::GenericNode;
use runtime::{Runtime, Execute, Job};
use engine::parser::token::stream::Position;
use engine::Node;
use std::clone::Clone;

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
