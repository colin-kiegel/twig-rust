// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Compiler Tests.

use compiler;

#[test]
fn new() {
    let _compiler = compiler::Setup::default().compiler();
}
