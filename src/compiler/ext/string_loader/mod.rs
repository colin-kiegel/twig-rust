/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * String Loader Extension
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
pub struct StringLoader;

impl Extension for StringLoader {
    fn name(&self) -> &str { "string_loader" }
}

impl StringLoader {
    pub fn _new() -> Box<StringLoader> {
        Box::new(StringLoader)
    }
}
