/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Twig library for rust
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */
 
// http://rustbyexample.com/mod/split.html

// mod my;
// 
// This declaration will look for a file named `my.rs` or `my/mod.rs` and will
// insert its contents inside a module named `my` under this scope

extern crate regex;

#[macro_use]
mod error;
mod lexer;
mod environment;
mod template;
