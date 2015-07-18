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

use std::rc::Rc;
use environment::Environment;
use lexer::token;
use self::job::Job;

/////////////
// exports //
/////////////

pub mod job;
pub mod node;
pub mod error;
pub use self::error::*;
pub use self::node::Node;


pub struct Parser {
    _environment: Rc<Environment>,
}

impl Parser {
    pub fn new(env: Environment) -> Parser {
        Parser {
            _environment: Rc::new(env),
        }
    }

    pub fn _environment(&self) -> &Environment {
        use std::ops::Deref;
        self._environment.deref()
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
        let env = Environment::default();

        Parser::new(env)
    }
}
