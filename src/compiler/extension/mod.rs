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

pub use compiler::{self, ParserBroker};
pub use parser::NodeVisitor;

/////////////
// exports //
/////////////

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

pub trait Extension {
    /// Initializes the compiler factory.
    /// This is where you can load some file that contains filter functions for instance.
    fn init(&self, _factory: &mut compiler::Factory) {} // TODO add error handling ???

    /// Get the token parser instances to register with the environment.
    fn token_parsers(&self) -> Vec<Box<ParserBroker>> { // TODO switch to iterators or Option<Vec<...>> ???
        Vec::new()
    }

    /// Get the node visitor instances to register with the environment.
    fn node_visitors(&self) -> Vec<Box<NodeVisitor>> {
        Vec::new()
    }

    /// Get the list of filters to register with the environment.
    fn filters(&self) -> Vec<()> {
        Vec::new()
    }

    /// Get the list of tests to register with the environment.
    fn tests(&self) -> Vec<()> {
        Vec::new()
    }

    /// Get the list of functions to register with the environment.
    fn functions(&self) -> Vec<()> {
        Vec::new()
    }

    /// Get the list of operators to register with the environment.
    fn operators(&self) {

    }

    /// Get the list of global variables to register with the environment.
    fn globals(&self) -> Vec<()> {
        Vec::new()
    }

    /// Get the name of the extension.
    fn name(&self) -> &str;
}
