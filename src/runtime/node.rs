/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Node of compiled templates (forming an Abstract-Syntax-Tree)
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

//////////////
// imports  //
//////////////

use runtime::Runtime;
use std::fmt::Debug;

/////////////
// exports //
/////////////


// TODO: Mimic fmt::Display trait - where runtime has the role of formatter + 'database'
pub trait NodeOutput : Debug {
    fn run(&self, runtime: &mut Runtime);
}
