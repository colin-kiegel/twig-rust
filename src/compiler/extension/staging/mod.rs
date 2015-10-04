/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Staging Extension
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use super::api::Extension;

/////////////
// exports //
/////////////


#[allow(dead_code)] // dummy
#[derive(Default, Debug, PartialEq)]
pub struct Staging;

impl Extension for Staging {
    fn name(&self) -> &str { "staging" }
}

impl Staging {
    pub fn new() -> Box<Staging> {
        Box::new(Staging)
    }
}
