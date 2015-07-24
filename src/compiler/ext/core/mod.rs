/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Core Extension
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use compiler;
use compiler::ext;
use super::Extension;
use std::collections::HashMap;

/////////////
// exports //
/////////////

pub mod token_parser;


#[derive(Default, Debug, PartialEq)]
pub struct Core {
    fmt_date: String, // "F j, Y H:i"
    fmt_date_interval: String, // "%d days"
    fmt_format: (usize, char, char), // (0, '.', ',')
    timezone: Option<String>, // type?
    escapers: Vec<()>, //??
}

impl Extension for Core {
    fn name(&self) -> &str { "core" }

    /// Initialize the compiler.
    /// This is where you can load some file that contains filter functions for instance.
    fn init(&self, _compiler: &mut compiler::Compiler) {} // TODO add error handling ???

    /// Get the token parser instances to register with the compiler.
    fn token_parsers(&self) -> HashMap<String, Box<ext::TokenParser>> {
        let mut p : HashMap<String, Box<ext::TokenParser>> = HashMap::new();
        p.insert("for".to_string(), Box::new(token_parser::For::default()));
        p.insert("if".to_string(), Box::new(token_parser::If::default()));
        p.insert("extends".to_string(), Box::new(token_parser::Extends::default()));
        p.insert("include".to_string(), Box::new(token_parser::Include::default()));
        p.insert("block".to_string(), Box::new(token_parser::Block::default()));
        p.insert("use".to_string(), Box::new(token_parser::Use::default()));
        p.insert("filter".to_string(), Box::new(token_parser::Filter::default()));
        p.insert("macro".to_string(), Box::new(token_parser::Macro::default()));
        p.insert("import".to_string(), Box::new(token_parser::Import::default()));
        p.insert("from".to_string(), Box::new(token_parser::From::default()));
        p.insert("set".to_string(), Box::new(token_parser::Set::default()));
        p.insert("spaceless".to_string(), Box::new(token_parser::Spaceless::default()));
        p.insert("flush".to_string(), Box::new(token_parser::Flush::default()));
        p.insert("do".to_string(), Box::new(token_parser::Do::default()));
        p.insert("embed".to_string(), Box::new(token_parser::Embed::default()));

        return p;
    }

    // /// Get the filters to register with the compiler.
    // fn filters(&self) -> HashMap<String, Box<ext::Filter>> {
    //     unimplemented!()
    // }
    //
    // /// Get the tests to register with the compiler.
    // fn tests(&self) -> HashMap<String, Box<ext::Test>> {
    //     unimplemented!()
    // }
    //
    // /// Get the functions to register with the compiler.
    // fn functions(&self) -> HashMap<String, Box<ext::Function>> {
    //     unimplemented!()
    // }
    //
    // /// Get the unary operators to register with the compiler.
    // fn operators_unary(&self) -> Vec<ext::UnaryOperator> {
    //     unimplemented!()
    // }
    //
    // /// Get the binary operators to register with the compiler.
    // fn operators_binary(&self) -> Vec<ext::BinaryOperator> {
    //     unimplemented!()
    // }
}

impl Core {
    pub fn new() -> Box<Core> {
        Box::new(Core::default())
    }
}
