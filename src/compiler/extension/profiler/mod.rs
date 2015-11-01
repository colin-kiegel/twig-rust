// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Profiler Extension
///
/// @author Colin Kiegel <kiegel@gmx.de>


/////////////
// imports //
/////////////

use super::api::Extension;

/////////////
// exports //
/////////////


#[allow(dead_code)] // dummy
#[derive(Default, Debug, PartialEq)]
pub struct Profiler;

impl Extension for Profiler {
    fn name(&self) -> &str { "profiler" }
}

impl Profiler {
    pub fn _new() -> Box<Profiler> {
        Box::new(Profiler)
    }
}
