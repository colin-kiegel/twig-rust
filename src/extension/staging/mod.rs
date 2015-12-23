// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Staging Extension

use super::api::Extension;

#[allow(dead_code)]
// dummy
#[derive(Default, Debug, PartialEq)]
pub struct Staging;

impl Extension for Staging {
    fn name(&self) -> &'static str {
        "staging"
    }
}

impl Staging {
    pub fn new() -> Box<Staging> {
        Box::new(Staging)
    }
}
