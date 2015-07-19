/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Core Extension
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use super::Extension;

/////////////
// exports //
/////////////


#[allow(dead_code)] // dummy
#[derive(Default, Debug, PartialEq)]
pub struct Core;

impl Extension for Core {
    fn name(&self) -> &str { "core" }
}

impl Core {
    pub fn new() -> Box<Core> {
        Box::new(Core)
    }
}
