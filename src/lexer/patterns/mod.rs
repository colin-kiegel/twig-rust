/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * The patterns used by the lexer to tokenize the templates.
 *
 * Written as regular expressions (perl-style).
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use std::rc::Rc;
use regex;
use regex::Error as regexError;
use environment::Environment;

/////////////
// exports //
/////////////

#[macro_use]
pub mod macros;
pub mod options;
pub mod var;
pub mod block;
pub mod raw_data;
pub mod operator;
pub mod comment;
pub mod block_raw;
pub mod block_line;
pub mod token_start;
pub mod interpolation_start;
pub mod interpolation_end;
pub use self::options::Options;


//const REGEX_NAME            : &'static str = r"\A[a-zA-Z_\x7f-\xff][a-zA-Z\d_\x7f-\xff]*";
//const REGEX_NUMBER          : &'static str = r"\A\d+(?:\.\d+)?";
//const REGEX_STRING          : &'static str = r"\A\"([^#\"\\]*(?:\\[.\n][^#\"\\]*)*)\"|'([^'\\]*(?:\\[.\n][^'\\]*)*)'";
//const REGEX_DQ_STRING_DELIM : &'static str = r"\A\"";
//const REGEX_DQ_STRING_PART  : &'static str = r"\A[^#\"\\]*(?:(?:\\[.\n]|#(?!\\{))[^#\"\\]*)*";

//#[derive(Debug)]
//#[derive(PartialEq)]
pub struct Patterns {
    pub options: Rc<Options>,
    pub environment: Rc<Environment>,
    pub var: var::Regex,
    pub block: block::Regex,
    pub raw_data: raw_data::Regex,
    //pub operator: operator::Regex,
    pub comment: comment::Regex,
    pub block_raw: block_raw::Regex,
    pub block_line: block_line::Regex,
    pub token_start: token_start::Pattern,
    pub interpolation_start: interpolation_start::Regex,
    pub interpolation_end: interpolation_end::Regex,
}

// to be implemented by the sub-modules
pub trait Extract<T> {
    fn regex(&self) -> &regex::Regex;
    fn extract(&self, captures: &regex::Captures) -> T
        where T: Sized;

//    fn extractor<F>(&self) -> F
//        where F: Fn(&regex::Captures) -> T;

//
//    fn extract_iter<'a, 't, F>(&'a self, text: &'t str) -> Map<regex::FindCaptures<'a, 't>, F>
//        where F: Fn(regex::Captures<'t>) -> T;
//        where Self: Sized,
//              T: Sized {
//            self.regex().captures_iter(text).map(|captures| self.extract(&captures))
//    }
}

#[allow(dead_code)]
pub struct Extractor<'a, 't, T> {
    extract: Box<Fn(regex::Captures<'t>) -> T>,
    captures_iter: regex::FindCaptures<'a, 't>
}

// Take a look at https://github.com/rust-lang/rust/pull/19849
// for implementing iterators
/*impl<'a, 't, T> Iterator for Extractor<'a, 't, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.captures_iter.next() {
            Some(item) => call(self.extract, item),
            None       => None,
        }
    }
}*/
//pub type ExtractIterator<'a, 't, T, F> = Map<regex::FindCaptures<'a, 't>, Fn(regex::Captures<'t>) -> T>;

#[allow(dead_code)]
#[allow(unused_variables)]
impl Patterns {
    pub fn new(env: Rc<Environment>, opt: Rc<Options>) -> Result<Patterns, regexError> {
        Ok(Patterns {
            var: try!(var::regex(&opt)),
            raw_data: try!(raw_data::regex(&opt)),
            //operator: try!(operator::regex(&env)),
            block: try!(block::regex(&opt)),
            comment: try!(comment::regex(&opt)),
            block_raw: try!(block_raw::regex(&opt)),
            block_line: try!(block_line::regex(&opt)),
            token_start: try!(token_start::Pattern::new(opt.clone())),
            interpolation_start: try!(interpolation_start::regex(&opt)),
            interpolation_end: try!(interpolation_end::regex(&opt)),
            options: opt,
            environment: env,
        })
    }
}

impl<'a> Default for Patterns {
    fn default() -> Patterns {
        let env = Rc::new(Environment::default());
        let opt = Rc::new(Options::default());

        Patterns::new(env, opt).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // must not panic!
    pub fn default() {
        Patterns::default();
    }
}
