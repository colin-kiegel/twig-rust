// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// String Loader Extension

use super::api::Extension;

#[allow(dead_code)] // dummy
#[derive(Default, Debug, PartialEq)]
pub struct StringLoader;

impl Extension for StringLoader {
    fn name(&self) -> &'static str { "string_loader" }
}

impl StringLoader {
    pub fn _new() -> Box<StringLoader> {
        Box::new(StringLoader)
    }
}
