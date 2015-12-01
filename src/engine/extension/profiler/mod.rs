// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Profiler Extension

use super::api::Extension;

#[allow(dead_code)] // dummy
#[derive(Default, Debug, PartialEq)]
pub struct Profiler;

impl Extension for Profiler {
    fn name(&self) -> &'static str { "profiler" }
}

impl Profiler {
    pub fn _new() -> Box<Profiler> {
        Box::new(Profiler)
    }
}
