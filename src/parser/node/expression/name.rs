/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Name Expression Node.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

//////////////
// imports  //
//////////////

use parser::node::GenericNode;
use runtime::{Runtime, NodeOutput, DataProvider};
use lexer::token::stream::Position;
use std::collections::HashMap;
use std::clone::Clone;

/////////////
// exports //
/////////////


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

impl NodeOutput for Name {
    fn output(&self, runtime: &mut Runtime, data: &HashMap<String, String>) {
        // TODO: Add some logging if lookup failed
        //      -> might make sense to do that locally
        let text: &str = data.get(&self.data.key).map_or("", |x| x.as_ref());
        runtime.write(text)
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
        let key = "message";
        let mut data = HashMap::<String, String>::default();
        data.insert("message".to_string(), "Hello World".to_string());
        let mut rt = Runtime::default();

        let node = Name { data: Data {
                key: key.to_string()
            }, ..Default::default() };

        node.run(&mut rt, &data);

        assert_eq!(
            rt.get_result(),
            "Hello World"
        );
    }
}
