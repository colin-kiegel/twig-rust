// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Parser

use engine::{Engine, ExtensionRegistry, extension};
use lexer::token;
use engine::extension::api::TokenParser;
use std::rc::Rc;
use template;
use std::collections::HashMap;

pub mod api;
pub mod error;
pub mod job;
pub mod node;
pub mod expression_parser;
pub use self::error::*;
pub use self::job::Job;
pub use self::api::Node;
pub use self::expression_parser::ExpressionParser;
use error::api::ErrorCode;

#[derive(Debug)]
pub struct Parser {
    ext: Rc<ExtensionRegistry>,
    expression_parser: ExpressionParser,
} // avoid a circular reference to the engine!

impl Parser {
    pub fn new(twig: &Engine) -> Result<Parser, ParserError> {
        let ext = match twig.extensions() {
            Err(e) => return Err(ParserErrorCode::MissingExtensions
                .at(loc!())
                .caused_by(e)),
            Ok(ext) => ext
        };

        Ok(Parser {
            ext: (*ext).clone(),
            expression_parser: ExpressionParser::new(ext),
        })
    }

    #[allow(dead_code)] // #TODO:720 testcase
    pub fn parse<'a, 't> (&'a self, stream: &'t token::Stream<'t>) -> Result<template::Compiled, ParserError>
        where 't: 'a // the token stream must outlive the Parser
    {
        let job = Job::new(stream, &self);

        job.parse()
    }

    // NOTE: deprecated! -> moved to parser::Job
    // pub fn parse_expression (
    //     &self,
    //     job: &mut Job,
    //     precedence: Precedence
    // ) -> Result<Box<Node>, ParserError>
    // {
    //     self.expression_parser.parse(job, precedence)
    // }

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
