/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Text Node
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

//////////////
// imports  //
//////////////

use super::GenericNode;
use runtime::{Runtime, NodeOutput, Job};
use lexer::token::stream::Position;
use std::clone::Clone;

/////////////
// exports //
/////////////

pub type Virtual = GenericNode<Data>;

#[derive(Debug, Default)]
pub struct Data;

impl Virtual {
    pub fn boxed(position: &Position) -> Box<Virtual> {
        Box::new(Virtual {
            position: (*position).clone(),
            ..GenericNode::default()
        })
    }
}

impl NodeOutput for Virtual {
    fn output(&self, runtime: &Runtime, job: &mut Job) {
        for node in &self.nodes {
             node.run(runtime, job)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use parser::node::text::Text;
    use runtime::Runtime;
    use std::default::Default;
    use parser::api::Node;

    #[test]
    fn run() {
        let rt = Runtime::default();

        let mut node_virtual = Virtual::default();

        {
            let children = node_virtual.children_mut();
            let node_hello = Text::boxed("Hello ".to_string(), &Default::default());
            let node_world = Text::boxed("world!".to_string(), &Default::default());
            children.push(node_hello);
            children.push(node_world);
        }

        assert_eq!(
            rt.run(&node_virtual),
            "Hello world!"
        );
    }
}
