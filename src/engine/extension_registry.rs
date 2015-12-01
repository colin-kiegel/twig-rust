// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Extension loader.

use extension::api::{self, Extension};
use std::collections::HashMap;
use engine::Engine;
use engine::error::{ExtensionRegistryError, ExtensionRegistryErrorCode};

pub type Iter<'a> = ::std::collections::hash_map::Values<'a, String, Box<Extension>>;

#[derive(Debug, Default)]
pub struct ExtensionRegistry {
    ext: HashMap<String, Box<api::Extension>>, // TODO: check for alternative Map-Types
    initialized: bool,
    filters: HashMap<String, Box<api::Filter>>,
    functions: HashMap<String, Box<api::Function>>,
    tests: HashMap<String, Box<api::Test>>,
    token_parsers: HashMap<String, Box<api::TokenParser>>,
    //_token_parser_by_tags: HashMap<String, Box<api::TokenParser>>,
    node_visitors: Vec<Box<api::NodeVisitor>>,
    operators_unary: HashMap<String, api::UnaryOperator>,
    operators_binary: HashMap<String, api::BinaryOperator>,
    _globals: Vec<Box<api::Global>>,
}

impl ExtensionRegistry {
    /// Register an extension
    pub fn push(&mut self, extension: Box<Extension>) -> Result<&mut Self, ExtensionRegistryError> {
        if self.initialized {
            return err!(ExtensionRegistryErrorCode::AlreadyInitialized)
        }

        if let Some(prev) = self.ext.insert(extension.name().to_string(), extension)  {
            return err!(ExtensionRegistryErrorCode::DuplicateExtension {
                prev: prev
            })
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

    /// Returns true if the extensions have been initialized with the engine
    pub fn initialized(&self) -> bool {
        self.initialized
    }

    /// # Failures
    /// If the extensions have not been initialized with the engine
    pub fn check_initialized(&self) -> Result<&Self, ExtensionRegistryError> {
        if self.initialized {
            Ok(self)
        } else {
            err!(ExtensionRegistryErrorCode::NotInitialized)
        }
    }

    /// Initialize extensions with the engine
    /// This is where you can load some file that contains filter functions for instance.
    pub fn init(&mut self, engine: &mut Engine) -> Result<(), ExtensionRegistryError> {
        if self.initialized {
            return err!(ExtensionRegistryErrorCode::AlreadyInitialized)
        }

        for (_, ext) in self.ext.iter() {
            ext.init(engine);

            for (k, v) in ext.filters() {
                if let Some(prev) = self.filters.insert(k, v) {
                    return err!(ExtensionRegistryErrorCode::DuplicateFilter {
                        prev: prev,
                        ext_name: ext.name()
                    })
                }
            }
            for (k, v) in ext.functions() {
                if let Some(prev) = self.functions.insert(k, v) {
                    return err!(ExtensionRegistryErrorCode::DuplicateFunction {
                        prev: prev,
                        ext_name: ext.name()
                    })
                }
            }
            for (k, v) in ext.tests() {
                if let Some(prev) = self.tests.insert(k, v) {
                    return err!(ExtensionRegistryErrorCode::DuplicateTest {
                        prev: prev,
                        ext_name: ext.name()
                    })
                }
            }
            for (k, v) in ext.token_parsers() {
                // #NOTE:60 can't have a reference to something owned within the same struct
                // and don't want to clone!
                //
                // if let Some(prev) = self._token_parser_by_tags.insert(v.tag().to_string(), &v) {
                //     return err!(ExtensionRegistryErrorCode::DuplicateTagHandler {
                //         prev: prev,
                //         ext_name: ext.name()
                //     })
                // }

                if let Some(prev) = self.token_parsers.insert(k, v) {
                    return err!(ExtensionRegistryErrorCode::DuplicateTokenParser {
                        prev: prev,
                        ext_name: ext.name()
                    })
                }
            }

            // TODO: `vec.append()` is not yet stable ...
            for v in ext.node_visitors() { self.node_visitors.push(v) }
            for v in ext.operators_unary() {
                if let Some(prev) = self.operators_unary.insert(v.repr.clone(), v) {
                    return err!(ExtensionRegistryErrorCode::DuplicateOperatorUnary {
                        prev: prev,
                        ext_name: ext.name()
                    })
                }
            }

            for v in ext.operators_binary() {
                if let Some(prev) = self.operators_binary.insert(v.repr.clone(), v) {
                    return err!(ExtensionRegistryErrorCode::DuplicateOperatorBinary {
                        prev: prev,
                        ext_name: ext.name()
                    })
                }
            }
        }

        Ok(())
    }

    /// Get the token parser instances defined by engine extensions.
    pub fn token_parsers(&self) -> &HashMap<String, Box<api::TokenParser>> {
        &self.token_parsers
    }

    // /// Get token parsers by registered tag
    // pub fn _token_parser_by_tags(&self) -> &HashMap<String, Box<api::TokenParser>> {
    //     &self._token_parser_by_tags
    // }

    /// Get the node visitor instances defined by engine extensions.
    pub fn node_visitors(&self) -> &Vec<Box<api::NodeVisitor>> {
        &self.node_visitors
    }

    /// Get the filters defined by engine extensions.
    pub fn filters(&self) -> &HashMap<String, Box<api::Filter>> {
        &self.filters
    }

    /// Get the tests defined by engine extensions.
    pub fn tests(&self) -> &HashMap<String, Box<api::Test>> {
        &self.tests
    }

    /// Get the functions defined by engine extensions.
    pub fn functions(&self) -> &HashMap<String, Box<api::Function>> {
        &self.functions
    }

    /// Get the unary operators defined by engine extensions.
    pub fn operators_unary(&self) -> &HashMap<String, api::UnaryOperator> {
        &self.operators_unary
    }

    /// Get the binary operators defined by engine extensions.
    pub fn operators_binary(&self) -> &HashMap<String, api::BinaryOperator> {
        &self.operators_binary
    }

    /// Get the global variables defined by engine extensions.
    pub fn _globals(&self) -> &Vec<Box<api::Global>> {
        &self._globals
    }
}
