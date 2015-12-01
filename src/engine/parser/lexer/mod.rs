// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Lexes a template string.

use std::rc::Rc;
#[cfg(test)]
mod test;
use template;
use engine::{Engine, ExtensionRegistry};
use engine::parser::lexer::job::Job;
use engine::parser::lexer::job::state::TokenizeState;
use error::ErrorCode;
use engine::parser::token;
pub mod error;
pub mod job;
pub mod patterns;
pub use self::patterns::Patterns;
pub use self::patterns::options::Options;
pub use self::error::{LexerError, LexerErrorCode, SyntaxError, SyntaxErrorCode};


#[derive(PartialEq, Debug)]
pub struct Lexer {
    patterns: Patterns,
}

impl Lexer {
    pub fn new(twig: &Engine, opt: Options) -> Result<Lexer, LexerError> {
        let ref opt = Rc::new(opt); // TODO: -> switch to &Options (!?)
        let ext = match twig.extensions() {
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

    #[allow(dead_code)] // TODO: testcase
    pub fn tokenize<'a, 't> (&'a self, template: &'t template::Raw) -> Result<token::Stream<'t>, LexerError>
        where 't: 'a // the template must outlive the Lexer
    {
        let job = Job::new(template, &self.patterns);

        job.tokenize()
    }
}

impl Default for Lexer {
    fn default() -> Lexer {
        let mut engine = Engine::default();
        engine.set_extensions(ExtensionRegistry::default());
        let options = Options::default();

        Lexer::new(&engine, options).unwrap()
    }
}
