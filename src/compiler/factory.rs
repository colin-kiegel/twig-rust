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

use std::ops::Deref;
use std::collections::HashMap;
use compiler::{Compiler, Options, Extension, extension};
use compiler::error::*;

/////////////
// exports //
/////////////


#[allow(dead_code)]
pub const VERSION : &'static str = "1.18.1";

// TODO: Refactor Environment
//      * split into factory and actual `Compiler`
//        rename `init_extensions` -> `into_compiler` ?
//      * the checks for 'extension_initialized' can then be removed
//        these checks indicate what belongs to the `Compiler` vs. its factory
//      * we will call the orig. compiler something like `compiler-backend` or `code-generator`
//        if we keep it at all (!)
#[derive(Default)]
pub struct Factory {
    _options: Options,
    _auto_reload: bool,
    extensions: HashMap<String, Box<Extension>>, // TODO check for alternative Map-Types

    _globals: Vec<()>,
    _loaded_templates: Vec<()>,
    _template_class_prefix: String, // default: '__TwigTemplate_'
    _function_callbacks: Vec<()>,
    _filter_callbacks: Vec<()>,
    staging: Box<extension::Staging>,
}

#[allow(dead_code)]
impl Factory {
    pub fn new(options: Options) -> Factory {
        let auto_reload = options.auto_reload.unwrap_or(options.debug);

        let extensions : Vec<Box<Extension>> = vec![
            extension::Core::new(),
            extension::Escaper::new(options.autoescape),
            extension::Optimizer::new(options.optimizations),
        ];

        let mut f = Factory {
            _options: options,
            _auto_reload: auto_reload,
            ..Factory::default()
        };

        f.add_extensions(extensions);
        f.staging = extension::Staging::new();

        return f;
    }

    /**
     * Registers an extension.
     *
     * @param Twig_ExtensionInterface $extension A Twig_ExtensionInterface instance
     */
    pub fn add_extension(&mut self, extension: Box<Extension>) {
        extension.init(self);

        self.extensions.insert(extension.name().to_string(), extension);
    }

    /**
     * Registers an array of extensions.
     *
     * @param array $extensions An array of extensions
     */
    pub fn add_extensions(&mut self, extensions: Vec<Box<Extension>>) {
        for x in extensions {
            self.add_extension(x);
        }
    }

    /// Get all registered extensions.
    pub fn extensions(&self) -> ::std::collections::hash_map::Iter<String, Box<Extension>> {
        self.extensions.iter()
    }

    // TODO : Environment to Compiler
    pub fn to_compiler(mut self) -> Compiler {
        // if self.extension_initialized { return }

        let mut c = Compiler {

            ..Compiler::default()
        };

        for (_, extension) in self.extensions.iter_mut() {
            c.init_extension(extension.deref().deref());
        }
        c.extensions = self.extensions;

        c.init_extension(self.staging.deref());
        c.staging = self.staging;

        return c;
    }
}

impl Compiler {
    // protected fn
    fn init_extension(&mut self, _extension: &Extension) {
        unimplemented!()
    // // filters
    // foreach ($extension->getFilters() as $name => $filter) {
    //     if ($name instanceof Twig_SimpleFilter) {
    //         $filter = $name;
    //         $name = $filter->getName();
    //     } elseif ($filter instanceof Twig_SimpleFilter) {
    //         $name = $filter->getName();
    //     }
    //
    //     $this->filters[$name] = $filter;
    // }
    //
    // // functions
    // foreach ($extension->getFunctions() as $name => $function) {
    //     if ($name instanceof Twig_SimpleFunction) {
    //         $function = $name;
    //         $name = $function->getName();
    //     } elseif ($function instanceof Twig_SimpleFunction) {
    //         $name = $function->getName();
    //     }
    //
    //     $this->functions[$name] = $function;
    // }
    //
    // // tests
    // foreach ($extension->getTests() as $name => $test) {
    //     if ($name instanceof Twig_SimpleTest) {
    //         $test = $name;
    //         $name = $test->getName();
    //     } elseif ($test instanceof Twig_SimpleTest) {
    //         $name = $test->getName();
    //     }
    //
    //     $this->tests[$name] = $test;
    // }
    //
    // // token parsers
    // foreach ($extension->getTokenParsers() as $parser) {
    //     if ($parser instanceof Twig_TokenParserInterface) {
    //         $this->parsers->addTokenParser($parser);
    //     } elseif ($parser instanceof Twig_TokenParserBrokerInterface) {
    //         $this->parsers->addTokenParserBroker($parser);
    //     } else {
    //         throw new LogicException('getTokenParsers() must return an array of Twig_TokenParserInterface or Twig_TokenParserBrokerInterface instances');
    //     }
    // }
    //
    // // node visitors
    // foreach ($extension->getNodeVisitors() as $visitor) {
    //     $this->visitors[] = $visitor;
    // }
    //
    // // operators
    // if ($operators = $extension->getOperators()) {
    //     if (2 !== count($operators)) {
    //         throw new InvalidArgumentException(sprintf('"%s::getOperators()" does not return a valid operators array.', get_class($extension)));
    //     }
    //
    //     $this->unaryOperators = array_merge($this->unaryOperators, $operators[0]);
    //     $this->binaryOperators = array_merge($this->binaryOperators, $operators[1]);
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
