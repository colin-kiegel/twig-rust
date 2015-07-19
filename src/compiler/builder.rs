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

use std::path::Path;
use std::ops::Deref;
use std::collections::HashMap;
use compiler::{Compiler, options, Options, ext, Extension};
use compiler::error::*;

/////////////
// exports //
/////////////

// pub mod options;
// pub use self::options::Options;


#[allow(dead_code)]
pub const VERSION : &'static str = "1.18.1";

#[derive(Debug)]
pub struct Builder {
    options: Options,
    extensions: HashMap<String, Box<Extension>>, // TODO check for alternative Map-Types
    ext_staging: Option<Box<ext::Staging>>,
}

impl Default for Builder {
    fn default() -> Builder {
        let b = Builder {
            options: Options::default(),
            extensions: HashMap::default(), // TODO check for alternative Map-Types
            ext_staging: Some(ext::Staging::new()),
        };

        let autoescape = b.options.autoescape;
        let optimizations = b.options.optimizations;

        return b
            .add_extension(ext::Core::new())
            .add_extension(ext::Escaper::new(autoescape))
            .add_extension(ext::Optimizer::new(optimizations));
    }
}

/// Builds instances of the Twig Compiler
// /
// / # Examples
// /
// / ```
// / use compiler::Builder;
// /
// / let compiler_default = Builder::default().compiler();
// / ```
// /
// / ```
// / use compiler::Builder;
// /
// / let compiler_custom = Builder::default()
// /     .set_strict_variables(true)
// /     .add_extension(extension::Profiler::new())
// /     .compiler();
// / ```
#[allow(dead_code)]
impl Builder {
    /// When set to true, it automatically set "auto_reload" to true as well
    ///     (default to false)
    pub fn set_debug(mut self, debug: bool) -> Self {
        self.options.debug = debug;

        self
    }

    /// The charset used by the templates (default to UTF-8)
    pub fn set_charset(mut self, set_charset: options::Charset) -> Self {
        self.options.charset = set_charset;

        self
    }

    /// Whether to ignore invalid variables in templates
    ///     (default to false).
    pub fn set_strict_variables(mut self, strict_variables: bool) -> Self {
        self.options.strict_variables = strict_variables;

        self
    }

    /// Whether to enable auto-escaping (default to html):
    ///     * false: disable auto-escaping
    ///     * true: equivalent to html
    ///     * html, js: set the autoescaping to one of the supported strategies
    ///     * filename: set the autoescaping strategy based on the template filename extension
    ///     * PHP callback: a PHP callback that returns an escaping strategy based on the template "filename"
    pub fn set_autoescape(mut self, autoescape: options::Autoescape) -> Self {
        self.options.autoescape = autoescape;

        self
    }

    /// An absolute path where to store the compiled templates (optional)
    pub fn set_cache(mut self, cache: Option<&Path>) -> Self {
        self.options.cache = cache.map(|reference| reference.to_owned());

        self
    }

    /// Whether to reload the template if the original source changed (optional).
    ///     If you don't provide the auto_reload option, it will be
    ///     determined automatically based on the debug value.
    pub fn set_auto_reload(mut self, auto_reload: Option<bool>) -> Self {
        self.options.auto_reload = auto_reload;

        self
    }

    /// A flag that indicates whether optimizations are applied
    pub fn set_optimizations(mut self, optimizations: options::Optimizations) -> Self {
        self.options.optimizations = optimizations;

        self
    }

    /// Get all options
    pub fn options(&self) -> &Options {
        &self.options
    }

    /// Registers an extension
    pub fn add_extension(mut self, extension: Box<Extension>) -> Self {
        self.extensions.insert(extension.name().to_string(), extension);

        self
    }

    /// Get all registered extensions
    pub fn extensions(&self) -> ::std::collections::hash_map::Iter<String, Box<Extension>> {
        self.extensions.iter()
    }

    // TODO : Environment to Compiler
    pub fn compiler(mut self) -> Compiler {
        let mut c = Compiler::default();

        for (_, extension) in self.extensions.iter_mut() {
            c.init_extension(extension.deref().deref());
        }
        c.extensions = c.extensions;

        if let Some(staging) = self.ext_staging {
            c.init_extension(staging.deref());
            c.ext_staging = Some(staging);
        }

        return c;
    }
}

impl Compiler {
    // protected fn
    fn init_extension(&mut self, ext: &Extension) {
        ext.init(self);

        // filters
        for (key, value) in ext.filters() { // TODO optimize
            self.filters.insert(key, value);
        }

        // functions
        for (key, value) in ext.functions() { // TODO optimize
            self.functions.insert(key, value);
        }

        // tests
        for (key, value) in ext.tests() { // TODO optimize
            self.tests.insert(key, value);
        }

        // token parsers
        for (key, value) in ext.token_parsers() { // TODO optimize
            self.token_parsers.insert(key, value);
        }

        // TODO: `vec.append()` is not yet stable ...
        for x in ext.node_visitors() { self.node_visitors.push(x) }
        for x in ext.unary_operators() { self.unary_operators.push(x) }
        for x in ext.binary_operators() { self.binary_operators.push(x) }

        // TODO register globals???
    }

    /**
     * Registers a Node Visitor.
     *
     * #arguments
     *  Twig_NodeVisitorInterface $visitor A Twig_NodeVisitorInterface instance
     */
    fn _add_node_visitor(&self, _visitor: ()) -> Result<(), TwigError> {
        unimplemented!();
        // return self.staging.add_node_visitor(visitor);
    }
}

#[allow(dead_code)]
#[cfg(test)]
mod test {
    // use super::*;

    //#[test]
    // pub fn get_unary_operators() {
    //     let mut e = Environment;
    //     e.get_unary_operators();
    // }

    //#[test]
    // pub fn get_binary_operators() {
    //     let mut e = Environment;
    //     e.get_binary_operators();
    // }
}
