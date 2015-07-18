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

#[macro_use]
mod error;
mod lexer;
mod parser;
mod template;
mod runtime;

/////////////
// exports //
/////////////

pub mod compiler;


// TODO switch to `if let` pattern https://github.com/rust-lang/rfcs/pull/160
// TODO switch to `while let` pattern https://github.com/rust-lang/rfcs/pull/214
