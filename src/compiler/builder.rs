// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Stores the Twig configuration.
///
/// @author Colin Kiegel <kiegel@gmx.de>


/////////////
// imports //
/////////////

use std::path::Path;
use std::rc::Rc;
use compiler::{Compiler, options, Options, extension, Extension, extension_registry, ExtensionRegistry};
use compiler::error::{TwigError};

/////////////
// exports //
/////////////


#[allow(dead_code)]
pub const VERSION : &'static str = "1.18.1";

#[derive(Debug)]
pub struct Builder {
    opt: Options,
    ext: ExtensionRegistry,
}

impl Default for Builder {
    fn default() -> Builder {
        let mut ext = ExtensionRegistry::default();
        ext.push(extension::Core::new()).unwrap(); // core extension

        Builder {
            opt: Options::default(),
            ext: ext,
        }
    }
}

/// Builds an instance of the Twig Compiler, according to supplied options and compiler extensions.
///
/// The following extensions will be registered by default:
/// * core
/// * escaper
/// * optimizer
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
        self.opt.debug = debug;

        self
    }

    /// The charset used by the templates (default to UTF-8)
    pub fn set_charset(mut self, set_charset: options::Charset) -> Self {
        self.opt.charset = set_charset;

        self
    }

    /// Whether to ignore invalid variables in templates
    ///     (default to false).
    pub fn set_strict_variables(mut self, strict_variables: bool) -> Self {
        self.opt.strict_variables = strict_variables;

        self
    }

    /// Whether to enable auto-escaping (default to html):
    ///     * false: disable auto-escaping
    ///     * true: equivalent to html
    ///     * html, js: set the autoescaping to one of the supported strategies
    ///     * filename: set the autoescaping strategy based on the template filename extension
    ///     * PHP callback: a PHP callback that returns an escaping strategy based on the template "filename"
    pub fn set_autoescape(mut self, autoescape: options::Autoescape) -> Self {
        self.opt.autoescape = autoescape;

        self
    }

    /// An absolute path where to store the compiled templates (optional)
    pub fn set_cache(mut self, cache: Option<&Path>) -> Self {
        self.opt.cache = cache.map(|reference| reference.to_owned());

        self
    }

    /// Whether to reload the template if the original source changed (optional).
    ///     If you don't provide the auto_reload option, it will be
    ///     determined automatically based on the debug value.
    pub fn set_auto_reload(mut self, auto_reload: Option<bool>) -> Self {
        self.opt.auto_reload = auto_reload;

        self
    }

    /// A flag that indicates whether optimizations are applied
    pub fn set_optimizations(mut self, optimizations: options::Optimizations) -> Self {
        self.opt.optimizations = optimizations;

        self
    }

    /// Get all options
    pub fn options(&self) -> &Options {
        &self.opt
    }

    /// Registers an extension
    pub fn add_extension(mut self, extension: Box<Extension>) -> Result<Self, TwigError> {
        try_chain!(self.ext.push(extension));

        Ok(self)
    }

    /// Get all registered extensions
    pub fn extensions(&self) -> extension_registry::Iter {
        self.ext.iter()
    }

    pub fn compiler(mut self) -> Result<Compiler, TwigError> {
        let mut c = Compiler::default();
        let o = self.opt;

        // add default extensions
        try_chain!(self.ext.push(extension::Escaper::new(o.autoescape)));
        try_chain!(self.ext.push(extension::Optimizer::new(o.optimizations)));

        // init extensions
        try_chain!(self.ext.init(&mut c));
        c.ext = Some(Rc::new(self.ext));

        // #TODO:360 register staging extension (!)
        // // init staging extension
        // let staging = ext::Staging::new();
        // try!(c.init_extension(&*staging));
        // c.ext_staging = Some(staging);

        return Ok(c);
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
