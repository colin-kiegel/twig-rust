// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Token parser
///
/// @author Colin Kiegel <kiegel@gmx.de>

/////////////
// imports //
/////////////

use std::fmt::Debug;

/////////////
// exports //
/////////////

pub trait TokenParser : Debug {}
