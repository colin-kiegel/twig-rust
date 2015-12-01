// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Engine Tests.

use engine;

#[test]
fn new() {
    let _engine = engine::Setup::default().engine();
}
