// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Node of compiled templates (forming an Abstract-Syntax-Tree)
///
/// @author Colin Kiegel <kiegel@gmx.de>


//////////////
// imports  //
//////////////

use runtime::{Runtime, Job};
use std::fmt::Debug;
use parser::api::Node;
use std::collections::HashMap;

/////////////
// exports //
/////////////


// #TODO:40 Mimic fmt::Display trait - where runtime has the role of formatter + 'database'
pub trait NodeOutput : Debug + Node {
    // NOTE: Can't use generic trait `runtime::api::DataProvider`
    //      because a generic function would not be object safe.
    //      Thus we restrict to HashMap first.
    fn output(&self, runtime: &Runtime, job: &mut Job);
}

pub trait DataProvider : Debug {
    fn get (&self, key: &str) -> Option<&str>;
    fn has (&self, key: &str) -> bool { self.get(key).is_some() }
}

impl DataProvider for HashMap<String, String> {
    fn get(&self, key: &str) -> Option<&str> {
        self.get(key).map(|x| x.as_ref())
    }

    fn has(&self, key: &str) -> bool {
        self.contains_key(key)
    }
}
