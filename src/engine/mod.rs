// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! The Twig Engine

#[cfg(test)]
mod test;
mod template_cache;
use loader::api::Loader;
use std::rc::Rc;
use template;
use error::ErrorCode;

pub mod error;
pub mod options;
pub mod setup;
pub mod extension_registry;
pub use self::error::{TwigError, TwigErrorCode, ExtensionRegistryError, ExtensionRegistryErrorCode};
pub use self::options::Options;
pub use self::setup::Setup;
pub use self::extension_registry::ExtensionRegistry;
pub use lexer::{self, Lexer};
pub use parser::{self, Parser};
pub use runtime::{self, Runtime};


#[derive(Default, Debug)] // #TODO:0 - provide a different constructor
pub struct Engine {
    options: Options,
    ext: Option<Rc<ExtensionRegistry>>,
    //ext_staging: Option<Box<ext::Staging>>,
    loader: Option<Box<Loader>>,
    lexer: Option<Lexer>,
    parser: Option<Parser>,
    //runtime: Option<Runtime>,

    _function_callbacks: Vec<()>,
    _filter_callbacks: Vec<()>,
    _template_class_prefix: String, // default: '__TwigTemplate_'
    _loaded_templates: Vec<()>,
}


impl Engine {
    /// Renders a template.
    ///
    /// # Failures
    /// * When the template cannot be found
    /// * When an error occurred during compilation
    /// * When an error occurred during rendering
    pub fn render(&mut self, _path: &str, _data: ()) -> Result<String, TwigError> {
        unimplemented!()
        //return Ok(try!(self.load_template(path, None)).render(context));
    }

    /// Displays a template.
    ///
    /// # Failures
    /// * When the template cannot be found
    /// * When an error occurred during compilation
    /// * When an error occurred during rendering
    pub fn display(&mut self, _path: &str, _data: ()) -> Result<(), TwigError> {
       unimplemented!()
       // return Ok(try!(self.load_template(path, None)).display(context, None));
    }

    /// Loads and compiles a template.
    ///
    /// # Failures
    /// * When the template cannot be found
    /// * When an error occurred during compilation
    pub fn load_template(&mut self, path: &str, _index: Option<u32>) -> Result<template::Compiled, TwigError> {
        // TODO: Cache compiled templates
        //  * cache lookup
        //  * check if cache is fresh
        //  * store in cache

        let template_raw = try!(self.load_template_raw(path));
        return self.compile_template(&template_raw);
    }

    /// Loads raw template.
    ///
    /// # Failures
    /// * When the template cannot be found
    fn load_template_raw(&mut self, path: &str) -> Result<template::Raw, TwigError> {
        let loader = try!(self.loader());
        let source = try_chain!(loader.source(path));
        Ok(template::Raw::new(source, path))
    }

    /// Compiles a template.
    ///
    /// # Failures
    /// * When an error occurred during lexing or parsing.
    fn compile_template(&mut self, template: &template::Raw) -> Result<template::Compiled, TwigError> {
        let tokenstream = {
            let lexer = try!(self.lexer());
            try_chain!(lexer.tokenize(template))
        };

        let compiled = {
            let parser = try!(self.parser());
            try_chain!(parser.parse(&tokenstream))
        };

        Ok(compiled)
    }

    /// Sets the engine extensions.
    pub fn set_extensions(&mut self, ext: ExtensionRegistry) -> &mut Engine {
        self.ext = Some(Rc::new(ext)); // #TODO:570 switch to callback pattern to provide arguments

        self
    }

    /// Gets the engine extensions.
    pub fn extensions(&self) -> Result<&Rc<ExtensionRegistry>, TwigError> {
        match self.ext {
            Some(ref ext) => Ok(ext),
            None => {
                try_chain!(err!(ExtensionRegistryErrorCode::NotInitialized))
            }
        }
    }

    /// Sets the loader instance.
    pub fn set_loader(&mut self, loader: Box<Loader>) -> &mut Engine {
        self.loader = Some(loader); // #TODO:580 switch to callback pattern to provide arguments

        self
    }

    /// Gets the loader instance.
    pub fn loader(&mut self) -> Result<&mut Loader, TwigError> {
        match self.loader {
            Some(ref mut loader) => return Ok(&mut **loader), // dereferencing the Box<>
            None => {
                return err!(TwigErrorCode::LoaderNotInitialized)
            }
        }
    }

    /// Sets the lexer instance.
    pub fn set_lexer(&mut self, lexer: Lexer) -> &mut Engine {
        self.lexer = Some(lexer); // #TODO:590 switch to callback pattern to provide arguments

        self
    }

    /// Gets the lexer instance.
    pub fn lexer(&mut self) -> Result<&Lexer, TwigError> {
        match self.lexer {
            Some(ref lexer) => return Ok(lexer),
            None => {
                self.lexer = Some(try_chain!(Lexer::new(self, lexer::Options::default())));
                return self.lexer();
            }
        }
    }

    /// Sets the parser instance.
    pub fn set_parser(&mut self, parser: Parser) -> &mut Engine {
        self.parser = Some(parser); // #TODO:600 switch to callback pattern to provide arguments

        self
    }

    /// Gets the parser instance.
    pub fn parser(&mut self) -> Result<&Parser, TwigError> {
        match self.parser {
            Some(ref parser) => return Ok(parser),
            None => {
                self.parser = match Parser::new(&self) {
                    Err(e) => return Err(TwigErrorCode::Parser
                        .at(loc!())
                        .caused_by(e)),
                    Ok(parser) => Some(parser)
                };
                return self.parser();
            }
        }
    }

    // /// Sets the runtime instance.
    // pub fn set_runtime(&mut self, runtime: Runtime) -> &mut Engine {
    //     self.runtime = Some(runtime); // #TODO:610 switch to callback pattern to provide arguments
    //
    //     self
    // }
    //
    // /// Gets the runtime instance.
    // pub fn runtime(&mut self) -> Result<&Runtime, TwigError> {
    //     match self.runtime {
    //         Some(ref runtime) => return Ok(runtime),
    //         None => {
    //             self.runtime = Some(Runtime::default());
    //             return self.runtime();
    //         }
    //     }
    // }
}
