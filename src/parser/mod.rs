// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Parser
///
/// @author Colin Kiegel <kiegel@gmx.de>


//////////////
// imports  //
//////////////

use compiler::{Compiler, ExtensionRegistry, extension};
use lexer::token;
use compiler::extension::api::TokenParser;
use std::rc::Rc;
use template;
use std::collections::HashMap;

/////////////
// exports //
/////////////

pub mod api;
pub mod error;
pub mod job;
pub mod node;
pub mod expression_parser;
pub use self::error::*;
pub use self::job::Job;
pub use self::api::Node;
pub use self::expression_parser::ExpressionParser;


#[derive(Debug)]
pub struct Parser {
    ext: Rc<ExtensionRegistry>,
    expression_parser: ExpressionParser,
} // avoid a circular reference to the compiler!

impl Parser {
    pub fn new(cp: &Compiler) -> Result<Parser, ParserError> {
        let ext = match cp.extensions() {
            Err(e) => return err!(ParserErrorCode::Logic)
                .explain(format!("Could not initialize parser due to missing compiler extensions"))
                .caused_by(e)
                .into(),
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

    /// Returns the compiler extensions.
    pub fn extensions(&self) -> &ExtensionRegistry {
        &*self.ext
    }

    /// Returns the token parser registered for `tag`
    ///
    /// Note: Tag handlers and token parsers are *identical*.
    pub fn tag_handler(&self, tag: &str) -> Option<&extension::api::TokenParser> {
        self.ext.token_parsers().get(tag).map(|x| &**x)
    }

    /// Returns all token parsers defined by compiler extensions.
    ///
    /// Note: Tag handlers and token parsers are *identical*.
    pub fn _token_parsers(&self) -> &HashMap<String, Box<extension::api::TokenParser>> {
        self.ext.token_parsers()
    }

    /// Returns the visitors defined by compiler extensions.
    pub fn visitors(&self) -> &Vec<Box<extension::api::NodeVisitor>> {
        self.ext.node_visitors()
    }
}

impl Default for Parser {
    fn default() -> Parser {
        let mut compiler = Compiler::default();
        compiler.set_extensions(ExtensionRegistry::default());

        Parser::new(&compiler).unwrap()
    }
}
