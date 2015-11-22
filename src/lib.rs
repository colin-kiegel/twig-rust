// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Twig library for rust
///
/// @author Colin Kiegel <kiegel@gmx.de>

/////////////
// imports //
/////////////

extern crate regex;

/////////////
// exports //
/////////////

#[macro_use]
pub mod error;
pub mod compiler;
pub mod lexer;
pub mod parser;
pub mod runtime;
pub mod loader;
pub mod template;
