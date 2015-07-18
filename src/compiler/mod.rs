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

use std::collections::HashMap;
use std::path::Path;
use template::Template;
use lexer::Lexer;
//use parser::Parser;

/////////////
// exports //
/////////////

pub mod error;
pub mod options;
pub mod parser_broker;
pub mod factory;
pub mod extension;
pub use self::error::*;
pub use self::options::Options;
pub use self::parser_broker::ParserBroker;
pub use self::extension::Extension;
pub use self::factory::Factory;


#[derive(Default)]
pub struct Compiler {
    // options: Options,
    // auto_reload: bool,
    _loader: (),
    _lexer: Option<Lexer>,
    _parser: (),//Option<Parser>,
    _compiler: (),

    extensions: HashMap<String, Box<Extension>>, // TODO check for alternative Map-Types

    parsers: Vec<()>,
    visitors: Vec<()>,
    _filters: Vec<()>,
    _tests: Vec<()>,
    _functions: Vec<()>,
    _globals: Vec<()>,
    unary_operators: Vec<()>,
    binary_operators: Vec<()>,

    _loaded_templates: Vec<()>,
    _template_class_prefix: String, // default: '__TwigTemplate_'
    _function_callbacks: Vec<()>,
    _filter_callbacks: Vec<()>,
    staging: Box<extension::Staging>,
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
        self.extensions.get(name).map(|x| ::std::ops::Deref::deref(x))
    }

    /// Gets the registered unary Operators.
    pub fn unary_operators(&mut self) -> &Vec<()> {
        &self.unary_operators
    }

    /// Gets the registered binary Operators.
    pub fn binary_operators(&mut self) -> &Vec<()> {
        &self.binary_operators
    }

    /// Get all registered Token Parsers.
    pub fn token_parsers(&mut self) -> &Vec<()> {
        &self.parsers
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
    pub fn node_visitors(&mut self) -> &Vec<()> {
        &self.visitors
    }
}
