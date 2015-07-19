/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Stores the Twig configuration.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use std::collections::HashMap;

/////////////
// exports //
/////////////

// Extensions:
pub mod core;
pub mod debug;
pub mod escaper;
pub mod optimizer;
pub mod profiler;
pub mod sandbox;
pub mod staging;
pub mod string_loader;
pub use self::core::Core;
pub use self::debug::Debug;
pub use self::escaper::Escaper;
pub use self::optimizer::Optimizer;
pub use self::profiler::Profiler;
pub use self::sandbox::Sandbox;
pub use self::staging::Staging;
pub use self::string_loader::StringLoader;
// Other:
pub use compiler::{self, TokenParser};
pub use parser::NodeVisitor;


pub trait Extension : ::std::fmt::Debug {
    /// Get the name of the extension.
    fn name(&self) -> &str;

    /// Initialize the compiler.
    /// This is where you can load some file that contains filter functions for instance.
    fn init(&self, _compiler: &mut compiler::Compiler) {} // TODO add error handling ???

    /// Get the token parser instances to register with the compiler.
    fn token_parsers(&self) -> HashMap<String, Box<TokenParser>> { // TODO switch to iterators or Option<Vec<...>> ???
        HashMap::new()
    }

    /// Get the node visitor instances to register with the compiler.
    fn node_visitors(&self) -> Vec<Box<NodeVisitor>> {
        Vec::new()
    }

    /// Get the filters to register with the compiler.
    fn filters(&self) -> HashMap<String, Box<()>> {
        HashMap::new()
    }

    /// Get the tests to register with the compiler.
    fn tests(&self) -> HashMap<String, Box<()>> {
        HashMap::new()
    }

    /// Get the functions to register with the compiler.
    fn functions(&self) -> HashMap<String, Box<()>> {
        HashMap::new()
    }

    /// Get the unary operators to register with the compiler.
    fn unary_operators(&self) -> Vec<()> {
        Vec::new()
    }

    /// Get the binary operators to register with the compiler.
    fn binary_operators(&self) -> Vec<()> {
        Vec::new()
    }

    /// Get the global variables to register with the compiler.
    fn globals(&self) -> Vec<()> {
        Vec::new()
    }
}
