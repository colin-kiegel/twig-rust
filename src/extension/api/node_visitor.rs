// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Extension `node visitor` definition.

use std::fmt::Debug;

pub trait NodeVisitor : Debug {} // #TODO:480 switch from trait to struct?
