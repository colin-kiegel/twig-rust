// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Root node of the template body.

use super::GenericNode;
use runtime::{Runtime, Execute, Job};
use parser::api::Node;

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

impl Execute for Body {
    fn execute(&self, runtime: &Runtime, job: &mut Job) {
        for node in &self.nodes {
             node.execute(runtime, job)
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
