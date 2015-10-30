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
use runtime::{Runtime, NodeOutput};
use lexer::token::stream::Position;
use std::clone::Clone;
use std::collections::HashMap;

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
    fn output(&self, runtime: &mut Runtime, data: &HashMap<String, String>) {
        for node in &self.nodes {
             node.run(runtime, data)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use parser::node::text::Text;
    use runtime::Runtime;
    use std::default::Default;
    use std::collections::HashMap;
    use parser::api::Node;

    #[test]
    fn run() {
        let data = HashMap::<String, String>::default();
        let mut rt = Runtime::default();

        let mut node_virtual = Virtual::default();

        {
            let children = node_virtual.children_mut();
            let node_hello = Text::boxed("Hello ".to_string(), &Default::default());
            let node_world = Text::boxed("world!".to_string(), &Default::default());
            children.push(node_hello);
            children.push(node_world);
        }

        node_virtual.run(&mut rt, &data);

        assert_eq!(
            rt.get_result(),
            "Hello world!"
        );
    }
}
