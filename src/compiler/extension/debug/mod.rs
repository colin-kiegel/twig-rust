// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Debug Extension
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
pub struct Debug;

impl Extension for Debug {
    fn name(&self) -> &str { "debug" }
}

impl Debug {
    pub fn _new() -> Box<Debug> {
        Box::new(Debug)
    }
}
