// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Twig Extension API

use std::fmt;
use std::collections::HashMap;
use engine::{self, Node};
use engine::parser::{Job, ParserError};
use engine::parser::token::stream::Item;
use api::error::Traced;

/// Extends the Twig Engine with new behaviour.
pub trait Extension : fmt::Debug {
    /// Get the name of the extension.
    fn name(&self) -> &'static str;

    /// Initialize the engine.
    /// This is where you can load some file that contains filter functions for instance.
    fn init(&self, _engine: &mut engine::Engine) {} // TODO: add error handling ???

    /// Get the token parser instances to register with the engine.
    fn token_parsers(&self) -> HashMap<String, Box<TokenParser>> {
        HashMap::new()
    }

    /// Get the node visitor instances to register with the engine.
    fn node_visitors(&self) -> Vec<Box<NodeVisitor>> {
        Vec::new()
    }

    /// Get the filters to register with the engine.
    fn filters(&self) -> HashMap<String, Box<Filter>> {
        HashMap::new()
    }

    /// Get the tests to register with the engine.
    fn tests(&self) -> HashMap<String, Box<Test>> {
        HashMap::new()
    }

    /// Get the functions to register with the engine.
    fn functions(&self) -> HashMap<String, Box<Function>> {
        HashMap::new()
    }

    /// Get the unary operators to register with the engine.
    fn operators_unary(&self) -> Vec<UnaryOperator> {
        Vec::new()
    }

    /// Get the binary operators to register with the engine.
    fn operators_binary(&self) -> Vec<BinaryOperator> {
        Vec::new()
    }

    /// Get the global variables to register with the engine.
    fn globals(&self) -> Vec<Box<Global>> {
        Vec::new()
    }
}

// Abstract extension traits + structs - TODO: check what needs to be trait / can be struct

/// Filter defined by Twig extensions.
pub trait Filter : fmt::Debug {}

/// Function defined by Twig extensions.
pub trait Function : fmt::Debug {}

/// Global defined by Twig extensions.
pub trait Global : fmt::Debug {}

/// Node visitor defined by Twig extensions.
pub trait NodeVisitor : fmt::Debug {}

/// Test defined by Twig extensions.
pub trait Test : fmt::Debug {}

/// Token Parser defined by Twig extensions.
///
/// Also called 'tag handler' by twig parser.
pub trait TokenParser : fmt::Debug {
    fn tag(&self) -> &'static str;

    fn parse(&self, job: &mut Job, item: &Item) -> Result<Box<Node>, Traced<ParserError>>;
}

pub mod token_parser {
    use engine::parser::token::stream::Item;

    pub type Test = Fn(&Item) -> TestResult;

    #[derive(Debug)]
    pub enum TestResult {
        Continue,  // orig: no_match
        KeepToken, // orig: is_match + dropNeedle == false
        DropToken, // orig: is_match + dropNeedle == true
    }
}

/// Unary operator defined by Twig extensions.
#[derive(Debug, PartialEq)]
pub struct UnaryOperator {
    pub repr: String, // token representation like "-"
    pub ext: op::Extension,
    pub prec: op::Precedence,
    pub op: op::Operation,
}

/// Binary operator defined by Twig extensions.
#[derive(Debug, PartialEq)]
pub struct BinaryOperator {
    pub repr: String, // token representation like "!="
    pub ext: op::Extension,
    pub prec: op::Precedence,
    pub op: op::Operation,
    pub assoc: op::Assoc,
}

pub mod op {
    #[derive(Debug, PartialEq)]
    pub struct Extension(String); // might switch to ID for faster lookups

    #[derive(Debug, PartialEq, PartialOrd)]
    pub struct Precedence(pub usize);

    #[derive(Debug, PartialEq)]
    pub enum Operation {
        Class(Class),
        Callable(Function)
    }

    /// Associativity
    #[derive(Debug, PartialEq)]
    pub enum Assoc {
        Left,
        Right,
    }

    #[derive(Debug, PartialEq)]
    pub struct Function {
        name: String
    }

    #[derive(Debug, PartialEq)]
    pub struct Class {
        name: String
    }
}
