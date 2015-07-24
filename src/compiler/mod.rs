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
use std::collections::HashMap;
use std::path::Path;
use template::Template;

/////////////
// exports //
/////////////

pub mod error;
pub mod options;
pub mod builder;
pub mod ext;
pub use self::error::{TwigError, TwigErrorCode};
pub use self::options::Options;
pub use self::builder::Builder;
pub use self::ext::Extension;
pub use lexer::{self, Lexer};
pub use parser::{self, Parser};
pub use runtime::{self, Runtime};


#[derive(Debug)]
pub struct Loader; // TODO: dummy
impl Loader { pub fn new() -> Loader { Loader } }

#[derive(Default, Debug)] // TODO - provide a different constructor
pub struct Compiler {
    options: Options,
    extensions: HashMap<String, Box<ext::Extension>>, // TODO check for alternative Map-Types
    ext_staging: Option<Box<ext::Staging>>,
    loader: Option<Loader>,
    lexer: Option<Lexer>,
    parser: Option<Parser>,
    runtime: Option<Runtime>,

    filters: HashMap<String, Box<ext::Filter>>,
    functions: HashMap<String, Box<ext::Function>>,
    tests: HashMap<String, Box<ext::Test>>,
    token_parsers: HashMap<String, Box<ext::TokenParser>>,
    node_visitors: Vec<Box<ext::NodeVisitor>>,
    operators_unary: Vec<ext::UnaryOperator>,
    operators_binary: Vec<ext::BinaryOperator>,

    _globals: Vec<Box<ext::Global>>,
    _function_callbacks: Vec<()>,
    _filter_callbacks: Vec<()>,

    _template_class_prefix: String, // default: '__TwigTemplate_'
    _loaded_templates: Vec<()>,
}


impl Compiler {



    /**
     * Renders a template.
     *
     * @param string $name    The template name
     * @param array  $context An array of parameters to pass to the template
     *
     * @return string The rendered template
     *
     * @throws Twig_Error_Loader  When the template cannot be found
     * @throws Twig_Error_Syntax  When an error occurred during compilation
     * @throws Twig_Error_Runtime When an error occurred during rendering
     */
    pub fn render(&self, path: &Path, context: Vec<()>) -> Result<String, TwigError> {
        return Ok(try!(self.load_template(path, None)).render(context));
    }

    /**
     * Displays a template.
     *
     * @param string $name    The template name
     * @param array  $context An array of parameters to pass to the template
     *
     * @throws Twig_Error_Loader  When the template cannot be found
     * @throws Twig_Error_Syntax  When an error occurred during compilation
     * @throws Twig_Error_Runtime When an error occurred during rendering
     */
    pub fn display(&self, path: &Path, context: Vec<()>) -> Result<(), TwigError> {
       return Ok(try!(self.load_template(path, None)).display(context, None));
    }

    /**
     * Loads a template by name.
     *
     * @param string $name  The template name
     * @param int    $index The index if it is an embedded template
     *
     * @return Twig_TemplateInterface A template instance representing the given template name
     *
     * @throws Twig_Error_Loader When the template cannot be found
     * @throws Twig_Error_Syntax When an error occurred during compilation
     */
    pub fn load_template(&self, _path: &Path, _index: Option<u32>) -> Result<Box<Template>, TwigError> {
        unimplemented!()
    }

    /**
     * Returns true if the given extension is registered.
     *
     * @param string $name The extension name
     *
     * @return bool Whether the extension is registered or not
     */
    pub fn has_extension(&self, name: &str) -> bool {
        self.extensions.contains_key(name)
    }

    /**
     * Gets an extension by name.
     *
     * @param string $name The extension name
     *
     * @return Twig_ExtensionInterface A Twig_ExtensionInterface instance
     */
    pub fn get_extension(&self, name: &str) -> Option<&Extension> {
        use std::ops::Deref;
        self.extensions.get(name).map(|x| x.deref())
    }

    /// Gets the registered unary Operators.
    pub fn operators_unary(&self) -> &Vec<ext::UnaryOperator> {
        &self.operators_unary
    }

    /// Gets the registered binary Operators.
    pub fn operators_binary(&self) -> &Vec<ext::BinaryOperator> {
        &self.operators_binary
    }

    /// Get all registered Token Parsers.
    pub fn token_parsers(&self) -> &HashMap<String, Box<ext::TokenParser>> {
        &self.token_parsers
    }

    /**
     * Gets registered tags.
     *
     * Be warned that this method cannot return tags defined by Twig_TokenParserBrokerInterface classes.
     *
     * #returns
     *  Twig_TokenParserInterface[] An array of Twig_TokenParserInterface instances
     */
    pub fn tags() {
        unimplemented!()
        // tags = HashMap::new();
        // for parser in self.get_token_parsers()->get_parsers() {
        //     if parser instanceof Twig_TokenParserInterface {
        //         tags.insert(parser.get_tag(), parser);
        //     }
        // }
        //
        // return tags;
    }

    /**
     * Gets the registered Node Visitors.
     *
     * @return Twig_NodeVisitorInterface[] An array of Twig_NodeVisitorInterface instances
     */
    pub fn node_visitors(&self) -> &Vec<Box<ext::NodeVisitor>> {
        &self.node_visitors
    }

    /// Sets the Lexer instance.
    pub fn set_loader(&mut self, loader: Loader) -> &mut Compiler {
        self.loader = Some(loader); // TODO switch to callback pattern to provide arguments

        self
    }

    /// Gets the parser instance.
    pub fn loader(&mut self) -> &Loader {
        match self.loader {
            Some(ref loader) => return loader,
            None => {
                self.loader = Some(Loader::new());
                return self.loader();
            }
        }
    }

    /// Sets the Lexer instance.
    pub fn set_lexer(&mut self, lexer: Lexer) -> &mut Compiler {
        self.lexer = Some(lexer); // TODO switch to callback pattern to provide arguments

        self
    }

    /// Gets the Lexer instance.
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

    /// Sets the Lexer instance.
    pub fn set_parser(&mut self, parser: Parser) -> &mut Compiler {
        self.parser = Some(parser); // TODO switch to callback pattern to provide arguments

        self
    }

    /// Gets the parser instance.
    pub fn parser(&mut self) -> &Parser {
        match self.parser {
            Some(ref parser) => return parser,
            None => {
                self.parser = Some(Parser::new(&self));
                return self.parser();
            }
        }
    }

    /// Sets the Lexer instance.
    pub fn set_runtime(&mut self, runtime: Runtime) -> &mut Compiler {
        self.runtime = Some(runtime); // TODO switch to callback pattern to provide arguments

        self
    }

    /// Gets the runtime instance.
    pub fn runtime(&mut self) -> &Runtime {
        match self.runtime {
            Some(ref runtime) => return runtime,
            None => {
                self.runtime = Some(Runtime::new());
                return self.runtime();
            }
        }
    }
}
