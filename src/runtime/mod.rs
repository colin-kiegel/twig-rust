// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Runtime for compiled templates
/// - it holds all data
/// - defines all necessary interfaces
/// - manages i/o for the compiled templates
///
/// @author Colin Kiegel <kiegel@gmx.de>


//////////////
// imports  //
//////////////

use std::collections::HashMap;

/////////////
// exports //
/////////////

pub mod api;
pub mod job;
pub mod error;
pub use self::api::{NodeOutput, DataProvider};
pub use self::job::Job;
pub use self::error::{RuntimeError, RuntimeErrorCode};

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Runtime {
    data: HashMap<String, String>
}

#[allow(dead_code)]
impl Runtime {
    pub fn new(data: HashMap<String, String>) -> Runtime {
        Runtime {
            data: data,
        }
    }

    pub fn run(&self, node: &NodeOutput) -> String {
        // TODO debug-switch
        Job::new().run(self, node)
    }


    pub fn data(&self) -> &HashMap<String, String> {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.data
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(|x| x.as_ref())
    }

    pub fn set<K,V>(&mut self, key: K, value: V) -> Option<String> where
        K: Into<String>,
        V: Into<String>
    {
        self.data.insert(key.into(), value.into())
    }

    // pub fn _clear_data(&mut self) {
    //     self.data.clear()
    // }
}
