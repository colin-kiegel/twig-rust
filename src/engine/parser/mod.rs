// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Parser

use engine::{Engine, ExtensionRegistry};
use extension;
use extension::api::TokenParser;
use std::rc::Rc;
use template;
use std::collections::HashMap;
use api::error::Traced;

pub mod error;
pub mod job;
pub mod expression_parser;
pub mod token;
pub mod lexer;
pub use self::lexer::Lexer;
pub use self::lexer::LexerError;
pub use self::token::Token;
pub use self::error::*;
pub use self::job::Job;
pub use self::expression_parser::ExpressionParser;

#[derive(Debug)]
pub struct Parser {
    ext: Rc<ExtensionRegistry>,
    expression_parser: ExpressionParser,
} // avoid a circular reference to the engine!

impl Parser {
    pub fn new(twig: &Engine) -> Result<Parser, Traced<ParserError>> {
        let ext = match twig.extensions() {
            Err(_) => return traced_err!(ParserError::MissingExtensions),
            Ok(ext) => ext
        };

        Ok(Parser {
            ext: (*ext).clone(),
            expression_parser: ExpressionParser::new(ext),
        })
    }

    #[allow(dead_code)] // TODO: testcase
    pub fn parse<'a, 't> (&'a self, stream: &'t token::Stream<'t>) -> Result<template::Compiled, Traced<ParserError>>
        where 't: 'a // the token stream must outlive the Parser
    {
        let job = Job::new(stream, &self);

        job.parse()
    }

    /// Returns the engine extensions.
    pub fn extensions(&self) -> &ExtensionRegistry {
        &*self.ext
    }

    /// Returns the token parser registered for `tag`
    ///
    /// Note: Tag handlers and token parsers are *identical*.
    pub fn tag_handler(&self, tag: &str) -> Option<&extension::api::TokenParser> {
        self.ext.token_parsers().get(tag).map(|x| &**x)
    }

    /// Returns all token parsers defined by engine extensions.
    ///
    /// Note: Tag handlers and token parsers are *identical*.
    pub fn _token_parsers(&self) -> &HashMap<String, Box<extension::api::TokenParser>> {
        self.ext.token_parsers()
    }

    /// Returns the visitors defined by engine extensions.
    pub fn visitors(&self) -> &Vec<Box<extension::api::NodeVisitor>> {
        self.ext.node_visitors()
    }
}

impl Default for Parser {
    fn default() -> Parser {
        let mut engine = Engine::default();
        engine.set_extensions(ExtensionRegistry::default());

        Parser::new(&engine).unwrap()
    }
}
