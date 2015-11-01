/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Represents a node that outputs an expression.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

//////////////
// imports  //
//////////////

use super::GenericNode;
use runtime::{Runtime, NodeOutput};
use lexer::token::stream::Position;
use parser::api::Node;
use std::clone::Clone;
use std::collections::HashMap;

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

impl NodeOutput for Print {
    fn output(&self, runtime: &mut Runtime, data: &HashMap<String, String>) {
        for node in &self.nodes {
             node.run(runtime, data)
        }
    }
}