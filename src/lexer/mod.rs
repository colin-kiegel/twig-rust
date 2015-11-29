// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Lexes a template string.

use std::rc::Rc;
#[cfg(test)]
mod test;
use template;
use compiler::{Compiler, ExtensionRegistry};
use lexer::job::Job;
use lexer::job::state::TokenizeState;
use error::api::ErrorCode;

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
    pub fn new(cp: &Compiler, opt: Options) -> Result<Lexer, LexerError> {
        let ref opt = Rc::new(opt); // #TODO:20 -> switch to &Options (!?)
        let ext = match cp.extensions() {
            Err(e) => return Err(LexerErrorCode::MissingExtensions
                .at(loc!())
                .caused_by(e)),
            Ok(ext) => ext
        };

        let p = match Patterns::new(opt, ext) {
            Err(e) => return Err(LexerErrorCode::PatternRegexError
                .at(loc!())
                .caused_by(e)),
            Ok(p) => p
        };

        Ok(Lexer {
            patterns: p,
        })
    }

    #[allow(dead_code)] // #TODO:680 testcase
    pub fn tokenize<'a, 't> (&'a self, template: &'t template::Raw) -> Result<token::Stream<'t>, LexerError>
        where 't: 'a // the template must outlive the Lexer
    {
        let job = Job::new(template, &self.patterns);

        job.tokenize()
    }
}

impl Default for Lexer {
    fn default() -> Lexer {
        let mut compiler = Compiler::default();
        compiler.set_extensions(ExtensionRegistry::default());
        let options = Options::default();

        Lexer::new(&compiler, options).unwrap()
    }
}
