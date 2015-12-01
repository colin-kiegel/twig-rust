// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Text Node.

use super::GenericNode;
use runtime::{Runtime, Execute, Job};
use engine::parser::token::stream::Position;
use std::clone::Clone;


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

impl Execute for Text {
    fn execute(&self, _runtime: &Runtime, job: &mut Job) {
        job.write(&self.data.text)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use runtime::Runtime;
    use std::default::Default;

    #[test]
    fn run() {
        let text = "Hello World";
        let rt = Runtime::default();

        let node = Text { data: Data {
                text: text.to_string()
            }, ..Default::default() };

        assert_eq!(
            rt.run(&node),
            "Hello World"
        );
    }
}
