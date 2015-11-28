// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Escaper Extension
///
/// @author Colin Kiegel <kiegel@gmx.de>


/////////////
// imports //
/////////////

use super::api::Extension;

/////////////
// exports //
/////////////


#[derive(Default, Debug, PartialEq)]
pub struct Escaper {
    mode: Mode,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode {
    Html,
    _Enabled,
    _Disabled,
    _Filename,
    _Callback,
}

impl Default for Mode {
    fn default() -> Mode {
        Mode::Html
    }
}

impl Extension for Escaper {
    fn name(&self) -> &'static str { "escaper" }
}

impl Escaper {
    pub fn new(mode: Mode) -> Box<Escaper> {
        Box::new(Escaper {
            mode: mode
        })
    }
}
