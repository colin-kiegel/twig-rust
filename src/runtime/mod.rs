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

pub mod api;
pub mod error;
pub use self::api::{NodeOutput, DataProvider};
pub use self::error::{RuntimeError, RuntimeErrorCode};

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Runtime {
    writer: String, // #TODO:560 switch to a 'more generic' string writer
                    // - maybe the writer should not be part of the runtime?
                    // - the writer could be part of a runtime job instead.
}

#[allow(dead_code)]
impl Runtime {
    // pub fn new() -> Runtime {
    //     Runtime {
    //         writer: String::new(),
    //     }
    // }

    pub fn reserve_writer(&mut self, additional: usize) -> &mut Runtime {
        self.writer.reserve(additional);

        self
    }

    pub fn run(&mut self, node: &NodeOutput, data: &HashMap<String, String>) {
        // debug-switch
        node.run(self, data)
    }

    pub fn write<T>(&mut self, text: T) where
        T: AsRef<str>
    {
        self.writer.push_str(text.as_ref())
    }

    pub fn get_result(&self) -> &str {
        &self.writer
    }

    pub fn _clear_result(&mut self) {
        self.writer.clear()
    }

    // pub fn _clear_data(&mut self) {
    //     self.data.clear()
    // }
}

impl ::std::convert::From<Runtime> for String {
    fn from(rt: Runtime) -> String {
        rt.writer
    }
}
