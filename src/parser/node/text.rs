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


pub type Text = GenericNode<Data>;

#[derive(Debug, Default)]
pub struct Data {
    text: String,
}

impl Text {
    pub fn boxed(text: String, position: &Position) -> Box<Text> {
        Box::new(Text {
            data: Data { text: text },
            position: (*position).clone(),
            ..GenericNode::default()
        })
    }
}

impl NodeOutput for Text {
    fn output(&self, runtime: &mut Runtime, _data: &HashMap<String, String>) {
        runtime.write(&self.data.text)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use runtime::Runtime;
    use std::default::Default;
    use std::collections::HashMap;
    use parser::api::Node;

    #[test]
    fn run() {
        let text = "Hello World";
        let data = HashMap::<String, String>::default();
        let mut rt = Runtime::default();

        let node = Text { data: Data {
                text: text.to_string()
            }, ..Default::default() };

        node.run(&mut rt, &data);

        assert_eq!(
            rt.get_result(),
            "Hello World"
        );
    }
}
