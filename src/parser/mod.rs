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

use compiler::{Compiler, ExtensionRegistry};
use lexer::token;
use self::job::Job;
use compiler::extension::api::TokenParser;
use compiler::extension::api::operator::Precedence;
use self::expression_parser::Expression;
use std::rc::Rc;

/////////////
// exports //
/////////////

pub mod error;
pub mod job;
pub mod node;
pub mod expression_parser;
pub use self::error::*;
pub use self::node::Node;
pub use self::expression_parser::ExpressionParser;


#[derive(Debug)]
pub struct Parser {
    ext: Rc<ExtensionRegistry>,
    expression_parser: ExpressionParser,
} // avoid a circular reference to the compiler!

impl Parser {
    pub fn new(cp: &Compiler) -> Result<Parser, ParserError> {
        let ext = match cp.extensions() {
            Err(e) => return err!(ParserErrorCode::Logic)
                .explain(format!("Could not initialize parser due to missing compiler extensions"))
                .caused_by(e)
                .into(),
            Ok(ext) => ext
        };

        Ok(Parser {
            ext: (*ext).clone(),
            expression_parser: ExpressionParser::new(ext),
        })
    }

    #[allow(dead_code)] // #TODO:720 testcase
    pub fn parse<'a, 't> (&'a self, stream: &'t token::Stream<'t>) -> Result<(), ParserError>
        where 't: 'a // the token stream must outlive the Parser
    {
        let job = Job::new(stream, &self);

        let test = "test".to_string(); // #TODO:780 wtf
        let drop_needle = false; // #TODO:790 wtf

        job.parse(test, drop_needle) // #TODO:260 move params to constructor??
    }

    pub fn parse_expression (
        &self,
        job: &mut Job,
        precedence: Precedence
    ) -> Result<Expression, ParserError>
    {
        self.expression_parser.parse(job, precedence)
    }
}

impl Default for Parser {
    fn default() -> Parser {
        let mut compiler = Compiler::default();
        compiler.set_extensions(ExtensionRegistry::default());

        Parser::new(&compiler).unwrap()
    }
}
