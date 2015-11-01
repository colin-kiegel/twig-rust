/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Default base class for compiled templates.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use template::api::Template;
use compiler::TwigError;
use runtime::Runtime;
use parser::node;

/////////////
// exports //
/////////////

#[allow(dead_code)]
#[derive(Debug)]
pub struct Compiled {
    root: node::Module,
    // TODO move *some* information from node::Module to template::Compiled
    //  -> easier accessible for runtime
    //  -> less accessible for node traverser (optimizer?) ...
}

impl Compiled {
    pub fn new(root: node::Module) -> Compiled {
        Compiled {
            root: root,
        }
    }

    #[allow(dead_code)]
    pub fn module(&self) -> &node::Module {
        &self.root
    }
}

impl Template for Compiled {
    fn render(&self, runtime: &Runtime) -> Result<String, TwigError> {
        Ok(runtime.run(&self.root))
    }

    fn display(&self, _runtime: &Runtime, _blocks: Option<Vec<()>>) {
        unimplemented!()
    }
}
