// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Name Expression Node.

use engine::node::GenericNode;
use runtime::{Runtime, Execute, Job};
use engine::parser::token::stream::Position;
use std::clone::Clone;

pub type Name = GenericNode<Data>;

#[derive(Debug, Default)]
pub struct Data {
    key: String,
}

impl Name {
    pub fn boxed(key: String, position: &Position) -> Box<Name> {
        Box::new(Name {
            data: Data { key: key },
            position: (*position).clone(),
            ..GenericNode::default()
        })
    }
}

impl Execute for Name {
    fn execute(&self, runtime: &Runtime, job: &mut Job) {
        // TODO: Add some logging if lookup failed
        //      -> might make sense to do that locally
        let text: &str = runtime.get(&self.data.key).unwrap_or("");
        job.write(text)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use runtime::Runtime;
    use std::default::Default;

    #[test]
    fn run() {
        let key = "message";
        let mut rt = Runtime::default();
        rt.set("message".to_string(), "Hello World".to_string());

        let node = Name { data: Data { key: key.to_string() }, ..Default::default() };

        assert_eq!(rt.run(&node), "Hello World");
    }
}
