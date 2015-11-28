// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Sandbox Extension
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
pub struct Sandbox;

impl Extension for Sandbox {
    fn name(&self) -> &'static str { "sandbox" }
}

impl Sandbox {
    pub fn _new() -> Box<Sandbox> {
        Box::new(Sandbox)
    }
}
