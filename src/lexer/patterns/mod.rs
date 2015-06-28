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

pub struct Extractor<'p, 't, Pattern: 'p> {
    pattern: &'p Pattern,
    captures_iter: regex::FindCaptures<'p, 't>
}

impl<'p, 't, Pattern> Iterator for Extractor<'p, 't, Pattern>
    where Pattern: Extract
{
    type Item = Pattern::Item;

    fn next(&mut self) -> Option<Pattern::Item> {
        match self.captures_iter.next() {
            Some(captures) => Some(self.pattern.item_from_captures(&captures)),
            None => None,
        }
    }
}

// to be implemented by all sub-patterns
pub trait Extract {
    type Item;

    #[inline]
    fn regex(&self) -> &regex::Regex;
    fn item_from_captures(&self, captures: &regex::Captures) -> Self::Item
        where Self::Item: Sized;

    fn extract(&self, text: &str) -> Option<Self::Item> {
        match self.captures(text) {
            Some(captures) => Some(self.item_from_captures(&captures)),
            None => None,
        }
    }

    fn extract_iter<'p, 't>(&'p self, text: &'t str) -> Extractor<'p, 't, Self>
        where Self: Sized
    {
        Extractor {
            pattern: self,
            captures_iter: self.captures_iter(text),
        }
    }

    /// http://doc.rust-lang.org/regex/regex/enum.Regex.html#method.is_match
    #[inline]
    fn is_match(&self, text: &str) -> bool {
        self.regex().is_match(text)
    }

    /// http://doc.rust-lang.org/regex/regex/enum.Regex.html#method.find
    #[inline]
    fn find(&self, text: &str) -> Option<(usize, usize)> {
        self.regex().find(text)
    }

    /// http://doc.rust-lang.org/regex/regex/enum.Regex.html#method.find_iter
    #[inline]
    fn find_iter<'r, 't>(&'r self, text: &'t str) -> regex::FindMatches<'r, 't> {
        self.regex().find_iter(text)
    }

    /// http://doc.rust-lang.org/regex/regex/enum.Regex.html#method.captures
    #[inline]
    fn captures<'t>(&self, text: &'t str) -> Option<regex::Captures<'t>> {
        self.regex().captures(text)
    }

    /// http://doc.rust-lang.org/regex/regex/enum.Regex.html#method.captures_iter
    #[inline]
    fn captures_iter<'r, 't>(&'r self, text: &'t str) -> regex::FindCaptures<'r, 't> {
        self.regex().captures_iter(text)
    }

    /// http://doc.rust-lang.org/regex/regex/enum.Regex.html#method.split
    #[inline]
    fn split<'r, 't>(&'r self, text: &'t str) -> regex::RegexSplits<'r, 't> {
        self.regex().split(text)
    }

    /// http://doc.rust-lang.org/regex/regex/enum.Regex.html#method.splitn
    #[inline]
    fn splitn<'r, 't>(&'r self, text: &'t str, limit: usize) -> regex::RegexSplitsN<'r, 't> {
        self.regex().splitn(text, limit)
    }

    /// http://doc.rust-lang.org/regex/regex/enum.Regex.html#method.replace
    #[inline]
    fn replace<R: regex::Replacer>(&self, text: &str, rep: R) -> String {
        self.regex().replace(text, rep)
    }

    /// http://doc.rust-lang.org/regex/regex/enum.Regex.html#method.replace_all
    #[inline]
    fn replace_all<R: regex::Replacer>(&self, text: &str, rep: R) -> String {
        self.regex().replace_all(text, rep)
    }

    /// http://doc.rust-lang.org/regex/regex/enum.Regex.html#method.replacen
    #[inline]
    fn replacen<R: regex::Replacer>(&self, text: &str, limit: usize, rep: R) -> String {
        self.regex().replacen(text, limit, rep)
    }

    /// http://doc.rust-lang.org/regex/regex/enum.Regex.html#method.as_str
    #[inline]
    fn as_str<'a>(&'a self) -> &'a str {
        self.regex().as_str()
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
