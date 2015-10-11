// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Twig library for rust
///
/// @author Colin Kiegel <kiegel@gmx.de>

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

/////////////
// imports //
/////////////

extern crate regex;

mod template;



// #TODO:220 http://keepachangelog.com
// #TODO:540 switch to `if let` pattern https://github.com/rust-lang/rfcs/pull/160
// #TODO:550 switch to `while let` pattern https://github.com/rust-lang/rfcs/pull/214
