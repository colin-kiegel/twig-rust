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
use environment::Environment;
use lexer::job::Job;
use lexer::job::state::TokenizeState;
use regex::Error as regexError;

/////////////
// exports //
/////////////

pub mod error;
pub mod token;
pub mod job;
pub mod patterns;
pub use self::patterns::Patterns;
pub use self::patterns::options::Options;
pub use self::error::{LexerError, LexerErrorCode, SyntaxError, SyntaxErrorCode};


struct Lexer {
    _environment: Rc<Environment>,
    _options: Rc<Options>,
    patterns: Patterns,
}

impl Lexer {
    pub fn new(env: Environment, opt: Options) -> Result<Lexer, regexError> {
        let env = Rc::new(env);
        let opt = Rc::new(opt);
        let patterns = try!(Patterns::new(env.clone(), opt.clone())); // TODO Error-Handling ?

        Ok(Lexer {
            _environment: env,
            _options: opt,
            patterns: patterns,
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
        let env = Environment::default();
        let options = Options::default();

        Lexer::new(env, options).unwrap()
    }
}
