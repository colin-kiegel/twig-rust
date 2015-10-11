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
#[derive(Debug)]
pub struct Runtime {
    data: HashMap<String, String>,
    writer: String, // #TODO:560 switch to a 'more generic' string writer
                    // - maybe the writer should not be part of the runtime?
}

#[allow(dead_code)]
impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            data: HashMap::new(),
            writer: String::new(),
        }
    }

    pub fn reserve_writer(&mut self, additional: usize) -> &mut Runtime {
        self.writer.reserve(additional);

        self
    }

    pub fn reserve_data(&mut self, additional: usize) -> &mut Runtime {
        self.data.reserve(additional);

        self
    }

    pub fn run(&mut self, node: &NodeOutput) {
        // debug-switch
        node.run(self)
    }

    pub fn _has(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    pub fn _get(&self, key: &str) -> Option<&str> {
        use std::ops::Deref; // #TODO:390 replace with as_str() - as soon as this API is stable
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
