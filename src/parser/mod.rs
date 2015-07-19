/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Parser
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */


/////////////
// exports //
/////////////

pub mod error;
pub mod job;
pub mod node;
pub mod node_visitor;
pub use self::error::*;
pub use self::node::Node;
pub use self::node_visitor::NodeVisitor;

//////////////
// imports  //
//////////////

use std::rc::Rc;
use compiler::Compiler;
use lexer::token;
use self::job::Job;

pub struct Parser {
    _compiler: Rc<Compiler>,
}

impl Parser {
    pub fn new(compiler: Compiler) -> Parser {
        Parser {
            _compiler: Rc::new(compiler),
        }
    }

    pub fn _compiler(&self) -> &Compiler {
        use std::ops::Deref;
        self._compiler.deref()
    }

    #[allow(dead_code)] // TODO testcase
    pub fn parse<'a, 't> (&'a self, stream: &'t token::Stream<'t>) -> Result<(), ParserError>
        where 't: 'a // the token stream must outlive the Parser
    {
        let job = Job::new(stream, &self);

        job.parse()
    }
}

impl Default for Parser {
    fn default() -> Parser {
        let compiler = Compiler::default();

        Parser::new(compiler)
    }
}
