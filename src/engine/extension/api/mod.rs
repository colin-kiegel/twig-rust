// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Stores the Twig configuration.

use std::collections::HashMap;

use engine;

// Abstract extension traits + structs // #TODO:120 check what needs to be trait / can be struct
pub mod filter;
pub mod function;
pub mod global;
pub mod node_visitor;
pub mod operator;
pub mod test;
pub mod token_parser;
pub use self::filter::Filter;
pub use self::function::Function;
pub use self::global::Global;
pub use self::node_visitor::NodeVisitor;
pub use self::operator::{UnaryOperator, BinaryOperator};
pub use self::test::Test;
pub use self::token_parser::TokenParser;

pub trait Extension : ::std::fmt::Debug {
    /// Get the name of the extension.
    fn name(&self) -> &'static str;

    /// Initialize the engine.
    /// This is where you can load some file that contains filter functions for instance.
    fn init(&self, _engine: &mut engine::Engine) {} // #TODO:80 add error handling ???

    /// Get the token parser instances to register with the engine.
    fn token_parsers(&self) -> HashMap<String, Box<TokenParser>> { // #TODO:620 switch to iterators or Option<Vec<...>> ???
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
