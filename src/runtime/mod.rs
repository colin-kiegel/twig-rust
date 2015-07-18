/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Runtime for compiled templates
 * - it holds all data
 * - defines all necessary interfaces
 * - manages i/o for the compiled templates
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

//////////////
// imports  //
//////////////

use std::collections::HashMap;

/////////////
// exports //
/////////////

pub mod node;
pub use self::node::NodeOutput;

#[allow(dead_code)]
pub struct Runtime {
    data: HashMap<String, String>,
    writer: String, // TODO switch to a 'more generic' string writer
                    // - maybe the writer should not be part of the runtime?
}

#[allow(dead_code)]
impl Runtime {
    pub fn with_capacity(writer: usize, data: usize) -> Runtime {
        Runtime {
            data: HashMap::with_capacity(data),
            writer: String::with_capacity(writer),
        }
    }

    pub fn run(&mut self, node: &NodeOutput) {
        // debug-switch
        node.run(self)
    }

    pub fn _has(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    pub fn _get(&self, key: &str) -> Option<&str> {
        use std::ops::Deref; // TODO replace with as_str() - as soon as this API is stable
        self.data.get(key).map(|x| x.deref())
    }

    pub fn write(&mut self, text: &str) {
        self.writer.push_str(text)
    }

    pub fn get_result(&self) -> &str {
        &self.writer
    }

    pub fn _clear_result(&mut self) {
        self.writer.clear()
    }

    pub fn _clear_data(&mut self) {
        self.data.clear()
    }
}
