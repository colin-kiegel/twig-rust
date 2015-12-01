// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Sandbox Extension

use super::api::Extension;

#[allow(dead_code)] // dummy
#[derive(Default, Debug, PartialEq)]
pub struct Sandbox;

impl Extension for Sandbox {
    fn name(&self) -> &'static str { "sandbox" }
}

impl Sandbox {
    pub fn _new() -> Box<Sandbox> {
        Box::new(Sandbox)
    }
}
