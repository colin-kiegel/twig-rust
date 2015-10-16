/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Default base class for compiled templates.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use super::api::Template;
use compiler::Compiler;
use compiler::TwigError;

/////////////
// exports //
/////////////

#[allow(dead_code)]
pub struct Compiled;

impl Template for Compiled {
    fn render(&self, _compiler: &mut Compiler, _context: Vec<()>) -> Result<String, TwigError> {
        unimplemented!()
    }

    fn display(&self, _compiler: &mut Compiler, _context: Vec<()>, _blocks: Option<Vec<()>>) {
        unimplemented!()
    }
}
