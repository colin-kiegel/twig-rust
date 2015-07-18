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

/////////////
// exports //
/////////////


pub type Text<'a> = GenericNode<Data<'a>>;

#[derive(Debug, Default)]
pub struct Data<'a> {
    text: &'a str,
}

impl<'a> NodeOutput for Text<'a> {
    fn run(&self, runtime: &mut Runtime) {
        runtime.write(self.data.text)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use runtime::{Runtime, NodeOutput};
    use std::default::Default;

    #[test]
    fn run() {
        let text = "Hello World";
        let mut rt = Runtime::with_capacity(10, 0);
        let node = Text { data: Data {
                text: &text
            }, ..Default::default() };

        node.run(&mut rt);

        assert_eq!(
            rt.get_result(),
            "Hello World"
        );
    }
}
