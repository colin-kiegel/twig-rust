/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Root node of the template body.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

//////////////
// imports  //
//////////////

use super::GenericNode;
use runtime::{Runtime, NodeOutput};
use parser::api::Node;
use std::collections::HashMap;

/////////////
// exports //
/////////////

pub type Body = GenericNode<Data>;

#[derive(Debug, Default)]
pub struct Data;

impl Body {
    pub fn boxed(nodes: Vec<Box<Node>>) -> Box<Body> {
        Box::new(Body {
            nodes: nodes,
            ..GenericNode::default()
        })
    }
}

impl NodeOutput for Body {
    fn output(&self, runtime: &mut Runtime, data: &HashMap<String, String>) {
        for node in &self.nodes {
             node.run(runtime, data)
        }
    }
}

// #[cfg(test)]
// mod test {
//     #[test]
//     fn run() {
//         unimplemented!()
//     }
// }
