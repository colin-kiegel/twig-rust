// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Core Extension

use engine;
use extension::api;
use std::collections::HashMap;

pub mod token_parser;


#[derive(Default, Debug, PartialEq)]
pub struct Core {
    fmt_date: String, // "F j, Y H:i"
    fmt_date_interval: String, // "%d days"
    fmt_format: (usize, char, char), // (0, '.', ',')
    timezone: Option<String>, // type?
    escapers: Vec<()>, // ??
}

impl api::Extension for Core {
    fn name(&self) -> &'static str {
        "core"
    }

    /// Initialize the engine.
    ///
    /// This is where you can load some file that contains filter functions for instance.
    fn init(&self, _engine: &mut engine::Engine) {} // TODO: add error handling ???

    /// Get the token parser instances to register with the engine.
    fn token_parsers(&self) -> HashMap<String, Box<api::TokenParser>> {
        let mut p: HashMap<String, Box<api::TokenParser>> = HashMap::new();
        p.insert("for".to_string(), Box::new(token_parser::For::default()));
        p.insert("if".to_string(), Box::new(token_parser::If::default()));
        p.insert("extends".to_string(),
                 Box::new(token_parser::Extends::default()));
        p.insert("include".to_string(),
                 Box::new(token_parser::Include::default()));
        p.insert("block".to_string(),
                 Box::new(token_parser::Block::default()));
        p.insert("use".to_string(), Box::new(token_parser::Use::default()));
        p.insert("filter".to_string(),
                 Box::new(token_parser::Filter::default()));
        p.insert("macro".to_string(),
                 Box::new(token_parser::Macro::default()));
        p.insert("import".to_string(),
                 Box::new(token_parser::Import::default()));
        p.insert("from".to_string(), Box::new(token_parser::From::default()));
        p.insert("set".to_string(), Box::new(token_parser::Set::default()));
        p.insert("spaceless".to_string(),
                 Box::new(token_parser::Spaceless::default()));
        p.insert("flush".to_string(),
                 Box::new(token_parser::Flush::default()));
        p.insert("do".to_string(), Box::new(token_parser::Do::default()));
        p.insert("embed".to_string(),
                 Box::new(token_parser::Embed::default()));

        return p;
    }

    // /// Get the filters to register with the engine.
    // fn filters(&self) -> HashMap<String, Box<api::Filter>> {
    //     unimplemented!()
    // }
    //
    // /// Get the tests to register with the engine.
    // fn tests(&self) -> HashMap<String, Box<api::Test>> {
    //     unimplemented!()
    // }
    //
    // /// Get the functions to register with the engine.
    // fn functions(&self) -> HashMap<String, Box<api::Function>> {
    //     unimplemented!()
    // }
    //
    // /// Get the unary operators to register with the engine.
    // fn operators_unary(&self) -> Vec<api::UnaryOperator> {
    //     unimplemented!()
    // }
    //
    // /// Get the binary operators to register with the engine.
    // fn operators_binary(&self) -> Vec<api::BinaryOperator> {
    //     unimplemented!()
    // }
}

impl Core {
    pub fn new() -> Box<Core> {
        Box::new(Core::default())
    }
}
