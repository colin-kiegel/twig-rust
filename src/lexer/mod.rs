/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Lexes a template string.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

//////////////
// imports  //
//////////////

use std::rc::Rc;
#[cfg(test)]
mod test;
use template;
use compiler::Compiler;
use lexer::job::Job;
use lexer::job::state::TokenizeState;

/////////////
// exports //
/////////////

pub mod error;
pub mod token;
pub mod job;
pub mod patterns;
pub use self::token::Token;
pub use self::patterns::Patterns;
pub use self::patterns::options::Options;
pub use self::error::{LexerError, LexerErrorCode, SyntaxError, SyntaxErrorCode};


#[derive(PartialEq, Debug)]
pub struct Lexer {
    patterns: Patterns,
}

impl Lexer {
    pub fn new(compiler: &Compiler, opt: Options) -> Result<Lexer, LexerError> {
        let opt = Rc::new(opt); // ToDo -> switch to &Options (!)

        let p = match Patterns::new(opt, compiler.operators_unary(), compiler.operators_binary()) {
            Err(e) => return err!(LexerErrorCode::InvalidPattern).caused_by(e).into(),
            Ok(p) => p
        };

        Ok(Lexer {
            patterns: p,
        })
    }

    #[allow(dead_code)] // TODO testcase
    pub fn tokenize<'a, 't> (&'a self, template: &'t template::Raw) -> Result<token::Stream, LexerError>
        where 't: 'a // the template must outlive the Lexer
    {
        let job = Job::new(template, &self.patterns);

        job.tokenize()
    }
}

impl Default for Lexer {
    fn default() -> Lexer {
        let compiler = Compiler::default();
        let options = Options::default();

        Lexer::new(&compiler, options).unwrap()
    }
}
