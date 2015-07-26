// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Extension loader.
///
/// @author Colin Kiegel <kiegel@gmx.de>

/////////////
// imports //
/////////////

use compiler::ext::{self, Extension};
use std::collections::HashMap;
use compiler::{Compiler, TwigError, TwigErrorCode};

/////////////
// exports //
/////////////


pub type Iter<'a> = ::std::collections::hash_map::Values<'a, String, Box<Extension>>;

#[derive(Debug, Default)]
pub struct ExtensionRegistry {
    ext: HashMap<String, Box<ext::Extension>>, // TODO check for alternative Map-Types
    initialized: bool,
    filters: HashMap<String, Box<ext::Filter>>,
    functions: HashMap<String, Box<ext::Function>>,
    tests: HashMap<String, Box<ext::Test>>,
    token_parsers: HashMap<String, Box<ext::TokenParser>>,
    //_token_parser_by_tags: HashMap<String, Box<ext::TokenParser>>,
    node_visitors: Vec<Box<ext::NodeVisitor>>,
    operators_unary: HashMap<String, ext::UnaryOperator>,
    operators_binary: HashMap<String, ext::BinaryOperator>,
    _globals: Vec<Box<ext::Global>>,
}

impl ExtensionRegistry {
    /// Register an extension
    pub fn push(&mut self, extension: Box<Extension>) -> Result<&mut Self, TwigError> {
        if self.initialized {
            return err!(TwigErrorCode::Logic)
                .explain(format!("Compiler are already initialized"))
                .into()
        }

        if let Some(prev) = self.ext.insert(extension.name().to_string(), extension)  {
            return err!(TwigErrorCode::Logic)
                .explain(format!("Duplicate extension {p:?} has already been registered.",
                    p = prev))
                .into();
        };

        Ok(self)
    }

    /// Returns true if the given extension is registered
    pub fn has(&self, name: &str) -> bool {
        self.ext.contains_key(name)
    }

    /// Get extension by name
    pub fn get(&self, name: &str) -> Option<&Extension> {
        use std::ops::Deref;
        self.ext.get(name).map(|x| x.deref())
    }

    /// Get all registered extensions
    pub fn iter(&self) -> Iter {
        self.ext.values()
    }

    /// Returns true if the extensions have been initialized with the compiler
    pub fn initialized(&self) -> bool {
        self.initialized
    }

    /// # Failures
    /// If the extensions have not been initialized with the compiler
    pub fn check_initialized(&self) -> Result<&Self, TwigError> {
        if self.initialized {
            Ok(self)
        } else {
            err!(TwigErrorCode::Logic)
                .explain(format!("Compiler extensions are not yet initialized"))
                .into()
        }
    }

    /// Initialize extensions with the compiler
    /// This is where you can load some file that contains filter functions for instance.
    pub fn init(&mut self, compiler: &mut Compiler) -> Result<(), TwigError> {
        if self.initialized {
            return err!(TwigErrorCode::Logic)
                .explain(format!("Compiler extensions already initialized"))
                .into()
        }

        for (_, ext) in self.ext.iter() {
            ext.init(compiler);

            for (k, v) in ext.filters() {
                if let Some(prev) = self.filters.insert(k, v) {
                    return err!(TwigErrorCode::Logic)
                        .explain(format!("Duplicate filter {p:?} while loading extension {x:?}.",
                            p = prev, x = ext.name()))
                        .into();
                    }
            }
            for (k, v) in ext.functions() {
                if let Some(prev) = self.functions.insert(k, v) {
                    return err!(TwigErrorCode::Logic)
                        .explain(format!("Duplicate function {p:?} while loading extension {x:?}.",
                            p = prev, x = ext.name()))
                        .into();
                }
            }
            for (k, v) in ext.tests() {
                if let Some(prev) = self.tests.insert(k, v) {
                    return err!(TwigErrorCode::Logic)
                        .explain(format!("Duplicate test {p:?} while loading extension {x:?}.",
                            p = prev, x = ext.name()))
                        .into();
                }
            }
            for (k, v) in ext.token_parsers() {
                // NOTE: can't have a reference to something owned within the same struct
                // and don't want to clone!
                //
                // if let Some(prev) = self._token_parser_by_tags.insert(v.tag().to_string(), &v) {
                //     return err!(TwigErrorCode::Logic)
                //         .explain(format!("Duplicate token parser by tag {p:?} while loading extension {x:?}.",
                //             p = prev, x = ext.name()))
                //         .into();
                // }

                if let Some(prev) = self.token_parsers.insert(k, v) {
                    return err!(TwigErrorCode::Logic)
                        .explain(format!("Duplicate token parser {p:?} while loading extension {x:?}.",
                            p = prev, x = ext.name()))
                        .into();
                }
            }

            // TODO: `vec.append()` is not yet stable ...
            for v in ext.node_visitors() { self.node_visitors.push(v) }
            for v in ext.operators_unary() {
                if let Some(prev) = self.operators_unary.insert(v.repr.clone(), v) {
                    return err!(TwigErrorCode::Logic)
                        .explain(format!("Duplicate unary operator {p:?} while loading extension {x:?}.",
                            p = prev, x = ext.name()))
                        .into();
                }
            }

            for v in ext.operators_binary() {
                if let Some(prev) = self.operators_binary.insert(v.repr.clone(), v) {
                    return err!(TwigErrorCode::Logic)
                        .explain(format!("Duplicate binary operator {p:?} while loading extension {x:?}.",
                            p = prev, x = ext.name()))
                        .into();
                }
            }
        }

        Ok(())
    }

    /// Get the token parser instances to register with the compiler.
    pub fn token_parsers(&self) -> &HashMap<String, Box<ext::TokenParser>> {
        &self.token_parsers
    }

    // /// Get token parsers by registered tag
    // pub fn _token_parser_by_tags(&self) -> &HashMap<String, Box<ext::TokenParser>> {
    //     &self._token_parser_by_tags
    // }

    /// Get the node visitor instances to register with the compiler.
    pub fn node_visitors(&self) -> &Vec<Box<ext::NodeVisitor>> {
        &self.node_visitors
    }

    /// Get the filters to register with the compiler.
    pub fn filters(&self) -> &HashMap<String, Box<ext::Filter>> {
        &self.filters
    }

    /// Get the tests to register with the compiler.
    pub fn tests(&self) -> &HashMap<String, Box<ext::Test>> {
        &self.tests
    }

    /// Get the functions to register with the compiler.
    pub fn functions(&self) -> &HashMap<String, Box<ext::Function>> {
        &self.functions
    }

    /// Get the unary operators to register with the compiler.
    pub fn operators_unary(&self) -> &HashMap<String, ext::UnaryOperator> {
        &self.operators_unary
    }

    /// Get the binary operators to register with the compiler.
    pub fn operators_binary(&self) -> &HashMap<String, ext::BinaryOperator> {
        &self.operators_binary
    }

    /// Get the global variables to register with the compiler.
    pub fn _globals(&self) -> &Vec<Box<ext::Global>> {
        &self._globals
    }
}