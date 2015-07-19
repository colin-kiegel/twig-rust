// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Compiler Tests
///
/// @author Colin Kiegel <kiegel@gmx.de>

/////////////
// imports //
/////////////

use compiler;

/////////////
// exports //
/////////////

#[test]
fn new() {
    let _compiler = compiler::Builder::default().compiler();
}
