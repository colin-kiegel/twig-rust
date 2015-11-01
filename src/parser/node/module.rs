// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Root node of the template.
///
/// @author Colin Kiegel <kiegel@gmx.de>


//////////////
// imports  //
//////////////

use super::GenericNode;
use runtime::{Runtime, NodeOutput, Job};
use parser::node;

/////////////
// exports //
/////////////



pub type Module = GenericNode<Data>;

#[derive(Debug, Default)]
pub struct Data {
    parent: Option<()>,
    blocks: Vec<()>, // as nodes?
    macros: Vec<()>, // as nodes?
    traits: Vec<()>, // as nodes?
    embedded_templates: Vec<()>,
    filename: String,
}

impl Module {
    pub fn new(
        body: Box<node::Body>,
        parent: Option<()>,
        blocks: Vec<()>,
        macros: Vec<()>,
        traits: Vec<()>,
        embedded_templates: Vec<()>,
        filename: &str,
    ) -> Module {
        Module {
            data: Data {
                parent: parent,
                blocks: blocks,
                macros: macros,
                traits: traits,
                embedded_templates: embedded_templates,
                filename: filename.to_string(),
            },
            nodes: vec![body],
            ..GenericNode::default()
        }
    }
}

impl NodeOutput for Module {
    fn output(&self, runtime: &Runtime, job: &mut Job) {
        for node in &self.nodes {
             node.run(runtime, job)
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
