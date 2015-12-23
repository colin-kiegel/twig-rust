// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Debug Extension.

use super::api::Extension;

#[allow(dead_code)]
// dummy
#[derive(Default, Debug, PartialEq)]
pub struct Debug;

impl Extension for Debug {
    fn name(&self) -> &'static str {
        "debug"
    }
}

impl Debug {
    pub fn new() -> Box<Debug> {
        Box::new(Debug)
    }
}
