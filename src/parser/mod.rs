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

//////////////
// imports  //
//////////////

use compiler::Compiler;
use lexer::token;
use self::job::Job;
//use compiler::ext::NodeVisitor;

/////////////
// exports //
/////////////

pub mod error;
pub mod job;
pub mod node;
pub use self::error::*;
pub use self::node::Node;


#[derive(Debug)]
pub struct Parser;
    //_compiler: &'a Compiler, // TODO: rm circular reference(!)


impl Parser {
    pub fn new(_compiler: &Compiler) -> Parser {
        Parser
    }

    // pub fn _compiler(&self) -> &Compiler {
    //     &*self._compiler
    // }

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

        Parser::new(&compiler)
    }
}
