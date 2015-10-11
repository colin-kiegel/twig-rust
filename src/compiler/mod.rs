// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// The Twig Compiler
///
/// @author Colin Kiegel <kiegel@gmx.de>

/////////////
// imports //
/////////////

#[cfg(test)]
mod test;
mod template_cache;
use loader::api::Loader;
use std::path::Path;
use std::rc::Rc;
use template::Template;

/////////////
// exports //
/////////////

pub mod error;
pub mod options;
pub mod builder;
pub mod extension;
pub mod extension_registry;
pub use self::error::{TwigError, TwigErrorCode};
pub use self::options::Options;
pub use self::builder::Builder;
pub use self::extension::api::Extension;
pub use self::extension_registry::ExtensionRegistry;
pub use lexer::{self, Lexer};
pub use parser::{self, Parser};
pub use runtime::{self, Runtime};


#[derive(Default, Debug)] // #TODO:0 - provide a different constructor
pub struct Compiler {
    options: Options,
    ext: Option<Rc<ExtensionRegistry>>,
    //ext_staging: Option<Box<ext::Staging>>,
    loader: Option<Box<Loader>>,
    lexer: Option<Lexer>,
    parser: Option<Parser>,
    runtime: Option<Runtime>,

    _function_callbacks: Vec<()>,
    _filter_callbacks: Vec<()>,
    _template_class_prefix: String, // default: '__TwigTemplate_'
    _loaded_templates: Vec<()>,
}


impl Compiler {
    /// Renders a template.
    ///
    /// # Failures
    /// * When the template cannot be found
    /// * When an error occurred during compilation
    /// * When an error occurred during rendering
    pub fn render(&self, path: &Path, context: Vec<()>) -> Result<String, TwigError> {
        return Ok(try!(self.load_template(path, None)).render(context));
    }

    /// Displays a template.
    ///
    /// # Failures
    /// * When the template cannot be found
    /// * When an error occurred during compilation
    /// * When an error occurred during rendering
    pub fn display(&self, path: &Path, context: Vec<()>) -> Result<(), TwigError> {
       return Ok(try!(self.load_template(path, None)).display(context, None));
    }

    /// Loads and compiles a template.
    ///
    /// # Failures
    /// * When the template cannot be found
    /// * When an error occurred during compilation
    pub fn load_template(&self, _path: &Path, _index: Option<u32>) -> Result<Box<Template>, TwigError> {
        unimplemented!()
    }

    /// Sets the compiler extensions.
    pub fn set_extensions(&mut self, ext: ExtensionRegistry) -> &mut Compiler {
        self.ext = Some(Rc::new(ext)); // #TODO:570 switch to callback pattern to provide arguments

        self
    }

    /// Gets the compiler extensions.
    pub fn extensions(&self) -> Result<&Rc<ExtensionRegistry>, TwigError> {
        match self.ext {
            Some(ref ext) => Ok(ext),
            None => {
                err!(TwigErrorCode::Logic)
                    .explain(format!("Compiler extensions are not initialized"))
                    .into()
            }
        }
    }

    /// Sets the loader instance.
    pub fn set_loader(&mut self, loader: Box<Loader>) -> &mut Compiler {
        self.loader = Some(loader); // #TODO:580 switch to callback pattern to provide arguments

        self
    }

    /// Gets the loader instance.
    pub fn loader(&mut self) -> &Loader {
        match self.loader {
            Some(ref mut loader) => return ::std::ops::Deref::deref(loader), // #TODO:310 nicer way to deref??
            None => {
                self.loader = unimplemented!();
                return self.loader();
            }
        }
    }

    /// Sets the lexer instance.
    pub fn set_lexer(&mut self, lexer: Lexer) -> &mut Compiler {
        self.lexer = Some(lexer); // #TODO:590 switch to callback pattern to provide arguments

        self
    }

    /// Gets the lexer instance.
    pub fn lexer(&mut self) -> Result<&Lexer, TwigError> {
        match self.lexer {
            Some(ref lexer) => return Ok(lexer),
            None => {
                self.lexer = match Lexer::new(self, lexer::Options::default()) {
                    Err(e) => return err!(TwigErrorCode::Lexer).caused_by(e).into(),
                    Ok(lexer) => Some(lexer)
                };
                return self.lexer();
            }
        }
    }

    /// Sets the parser instance.
    pub fn set_parser(&mut self, parser: Parser) -> &mut Compiler {
        self.parser = Some(parser); // #TODO:600 switch to callback pattern to provide arguments

        self
    }

    /// Gets the parser instance.
    pub fn parser(&mut self) -> Result<&Parser, TwigError> {
        match self.parser {
            Some(ref parser) => return Ok(parser),
            None => {
                self.parser = match Parser::new(&self) {
                    Err(e) => return err!(TwigErrorCode::Parser).caused_by(e).into(),
                    Ok(parser) => Some(parser)
                };
                return self.parser();
            }
        }
    }

    /// Sets the runtime instance.
    pub fn set_runtime(&mut self, runtime: Runtime) -> &mut Compiler {
        self.runtime = Some(runtime); // #TODO:610 switch to callback pattern to provide arguments

        self
    }

    /// Gets the runtime instance.
    pub fn runtime(&mut self) -> Result<&Runtime, TwigError> {
        match self.runtime {
            Some(ref runtime) => return Ok(runtime),
            None => {
                self.runtime = Some(Runtime::new());
                return self.runtime();
            }
        }
    }
}
