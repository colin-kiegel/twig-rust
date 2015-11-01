// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Node of compiled templates (forming an Abstract-Syntax-Tree)
///
/// @author Colin Kiegel <kiegel@gmx.de>


//////////////
// imports  //
//////////////

use runtime::{Runtime, Job};
use std::fmt::Debug;

/////////////
// exports //
/////////////


pub trait Execute : Debug {
    fn execute(&self, runtime: &Runtime, job: &mut Job);
}
