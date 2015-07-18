/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Optimizer Extension
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


#[derive(Default, PartialEq)]
pub struct Optimizer {
    mode: Mode
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode {
    Enabled,
    Disabled,
}

impl Default for Mode {
    fn default() -> Mode {
        Mode::Enabled
    }
}

impl Extension for Optimizer {
    fn name(&self) -> &str { "optimizer" }
}

impl Optimizer {
    pub fn new(mode: Mode) -> Box<Optimizer> {
        Box::new(Optimizer {
            mode: mode
        })
    }
}
