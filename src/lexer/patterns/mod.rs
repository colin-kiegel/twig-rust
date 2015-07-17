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
pub mod token_start; // var, comment, block
pub mod var_end;
pub mod comment_end;
pub mod block_end;
pub mod block_line;
pub mod verbatim_start;
pub mod verbatim_end;
pub mod interpolation_start;
pub mod interpolation_end;
pub mod operator;
pub mod name;
pub mod number;
pub mod punctuation;
pub mod string;
pub mod string_dq_delim;
pub mod string_dq_part;
pub use self::options::Options;


pub struct Patterns {
    pub options: Rc<Options>,
    pub environment: Rc<Environment>,
    pub var_end: var_end::Pattern,
    pub block_end: block_end::Pattern,
    pub verbatim_end: verbatim_end::Pattern,
    //pub operator: operator::Pattern,
    pub comment_end: comment_end::Pattern,
    pub verbatim_start: verbatim_start::Pattern,
    pub block_line: block_line::Pattern,
    pub token_start: token_start::Pattern,
    pub interpolation_start: interpolation_start::Pattern,
    pub interpolation_end: interpolation_end::Pattern,
    pub name: name::Pattern,
    pub number: number::Pattern,
    pub punctuation: &'static punctuation::Pattern,
    pub string: string::Pattern,
    pub string_dq_delim: string_dq_delim::Pattern,
    pub string_dq_part: string_dq_part::Pattern,
}

#[allow(unused_variables)]
impl Patterns {
    pub fn new(env: Rc<Environment>, opt: Rc<Options>) -> Result<Patterns, regexError> {
        Ok(Patterns {
            var_end: try!(var_end::Pattern::new(opt.clone())),
            verbatim_end: try!(verbatim_end::Pattern::new(opt.clone())),
            //operator: try!(operator::Pattern::new(&env)),
            block_end: try!(block_end::Pattern::new(opt.clone())),
            comment_end: try!(comment_end::Pattern::new(opt.clone())),
            verbatim_start: try!(verbatim_start::Pattern::new(opt.clone())),
            block_line: try!(block_line::Pattern::new(opt.clone())),
            token_start: try!(token_start::Pattern::new(opt.clone())),
            interpolation_start: try!(interpolation_start::Pattern::new(opt.clone())),
            interpolation_end: try!(interpolation_end::Pattern::new(opt.clone())),
            name: try!(name::Pattern::new()),
            number: try!(number::Pattern::new()),
            punctuation: punctuation::Pattern::instance(),
            string: try!(string::Pattern::new()),
            string_dq_delim: try!(string_dq_delim::Pattern::new()),
            string_dq_part: try!(string_dq_part::Pattern::new()),
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

pub struct ExtractIter<'p, 't, Pattern: 'p> {
    pattern: &'p Pattern,
    captures_iter: regex::FindCaptures<'p, 't>
}

impl<'p, 't, Pattern> Iterator for ExtractIter<'p, 't, Pattern>
    where Pattern: Extract<'t>
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
pub trait Extract<'t> {
    type Item;

    #[inline]
    fn regex(&self) -> &regex::Regex;
    fn item_from_captures(&self, captures: &regex::Captures<'t>) -> Self::Item
        where Self::Item: Sized;

    fn extract(&self, text: &'t str) -> Option<Self::Item> {
        match self.captures(text) {
            Some(captures) => Some(self.item_from_captures(&captures)),
            None => None,
        }
    }

    fn extract_iter<'p>(&'p self, text: &'t str) -> ExtractIter<'p, 't, Self>
        where Self: Sized
    {
        ExtractIter {
            pattern: self,
            captures_iter: self.captures_iter(text)
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
    fn find_iter<'r>(&'r self, text: &'t str) -> regex::FindMatches<'r, 't> {
        self.regex().find_iter(text)
    }

    /// http://doc.rust-lang.org/regex/regex/enum.Regex.html#method.captures
    #[inline]
    fn captures(&self, text: &'t str) -> Option<regex::Captures<'t>> {
        self.regex().captures(text)
    }

    /// http://doc.rust-lang.org/regex/regex/enum.Regex.html#method.captures_iter
    #[inline]
    fn captures_iter<'r>(&'r self, text: &'t str) -> regex::FindCaptures<'r, 't> {
        self.regex().captures_iter(text)
    }

    /// http://doc.rust-lang.org/regex/regex/enum.Regex.html#method.split
    #[inline]
    fn split<'r>(&'r self, text: &'t str) -> regex::RegexSplits<'r, 't> {
        self.regex().split(text)
    }

    /// http://doc.rust-lang.org/regex/regex/enum.Regex.html#method.splitn
    #[inline]
    fn splitn<'r>(&'r self, text: &'t str, limit: usize) -> regex::RegexSplitsN<'r, 't> {
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

/// Trim whitespace in a *php-compatible* manner
///
/// http://php.net/manual/en/function.rtrim.php
/// trims the following whitespace characters from the end of a string
/// - " " (ASCII 32 (0x20)), an ordinary space.
/// - "\t" (ASCII 9 (0x09)), a tab.
/// - "\n" (ASCII 10 (0x0A)), a new line (line feed).
/// - "\r" (ASCII 13 (0x0D)), a carriage return.
/// - "\0" (ASCII 0 (0x00)), the NULL-byte.
/// - "\x0B" (ASCII 11 (0x0B)), a vertical tab.
///
/// Rusts built-in trim_right does not seem to trim "\0"
pub fn _php_trim_right(slice: &str) -> &str {
    let ws: &[_] = &[' ', '\t', '\n', '\r', '\0', '\x0B'];
    slice.trim_right_matches(ws)
}

fn to_hex(c: &char) -> Option<u32> {
    match *c {
        '0'...'9' => Some(*c as u32 - '0' as u32),
        'a'...'f' => Some(10 + *c as u32 - 'a' as u32),
        'A'...'F' => Some(10 + *c as u32 - 'A' as u32),
        _ => None,
    }
}

fn to_oct(c: &char) -> Option<u32> {
    match *c {
        '0'...'7' => Some(*c as u32 - '0' as u32),
        _ => None,
    }
}

// supposed to be *compatible* with PHP implementation
// - http://php.net/manual/en/function.stripcslashes.php
// - https://github.com/php/php-src/blob/master/ext/standard/string.c
//
// see also https://en.wikipedia.org/wiki/Escape_sequences_in_C#Table_of_escape_sequences
//
pub fn php_stripcslashes(string: &str) -> String {
    let mut result = String::with_capacity(string.len());
    let mut it = string.chars();
    let mut cur : Option<char>;

    'next: loop { // we need manual loop control for lookaheads in some match branches
        cur = it.next();
        'current: loop { match cur {
            None => break 'next,
            Some('\\') => match it.next() {
                None => { result.push('\\'); continue 'next },
                Some('a') => { result.push('\x07'); continue 'next }, // alarm (beep/bell)
                Some('b') => { result.push('\x08'); continue 'next }, // backspace
                Some('f') => { result.push('\x0C'); continue 'next }, // formfeed
                Some('n') => { result.push('\n'); continue 'next }, // new line
                Some('r') => { result.push('\r'); continue 'next }, // cariage return
                Some('t') => { result.push('\t'); continue 'next }, // horizontal tab
                Some('v') => { result.push('\x0B'); continue 'next }, // vertical tab
                Some('\\') => { result.push('\\'); continue 'next }, // backslash
                Some('x') => { // assuming *hex* UTF32 representation
                    let mut v: char;
                    let mut parsed = 0;
                    let mut char_u32 = 0;

                    v = match {cur = it.next(); cur} {
                        None => { result.push('x'); break 'next },
                        Some(value) => value,
                    };

                    'hex: while parsed < 3 { match to_hex(&v) {
                        None => break 'hex,
                        Some(hex) => {
                            parsed += 1;
                            char_u32 = 16*char_u32 + hex;
                            v = match {cur = it.next(); cur} {
                                None => break 'hex,
                                Some(value) => value,
                            }
                        }
                    }}

                    // we don't parse invalid hex
                    if parsed == 0 { result.push('x'); continue 'next }

                    match ::std::char::from_u32(char_u32) {
                        None => continue 'current, // discard invalid UTF32
                        Some(converted) => {
                            result.push(converted);
                            continue 'current
                        },
                    }
                },
                Some(escaped) => { // assuming *octal* UTF32 representation
                    let mut v = escaped;
                    let mut parsed = 0;
                    let mut char_u32 = 0;

                    'octal: while parsed < 3 { match to_oct(&v) {
                        None => break 'octal,
                        Some(oct) => {
                            parsed += 1;
                            char_u32 = 8*char_u32 + oct;
                            v = match {cur = it.next(); cur} {
                                None => break 'octal,
                                Some(value) => value,
                            }
                        }
                    }}

                    // we don't parse invalid oct
                    if parsed == 0 { result.push(escaped); continue 'next };

                    match ::std::char::from_u32(char_u32) {
                        None => continue 'current, // discard invalid UTF32
                        Some(converted) => {
                            result.push(converted);
                            continue 'current
                        },
                    }
                }
            },
            Some(value) => { result.push(value); continue 'next },
        }}
    }

    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // must not panic!
    pub fn default() {
        Patterns::default();
    }

    #[test]
    pub fn _php_stripcslashes() {
        assert_eq!(
            php_stripcslashes(&r#"\a\b\f\n\r\t\v\\\'\"\?\013\x00"#),
            "\x07\x08\x0C\n\r\t\x0B\\'\"?\x0B\x00".to_string()
        );

        assert_eq!(
            php_stripcslashes(&r#"nothing to strip \"#),
            r#"nothing to strip \"#.to_string()
        );

        assert_eq!(
            php_stripcslashes(&r#"almost nothing to strip \x"#),
            r#"almost nothing to strip x"#.to_string()
        );
    }

    #[test]
    pub fn php_trim_right() {
        // Rusts built-in trim_right does not trim "\0"
        assert_eq!(
            _php_trim_right("trim me PHP! \0 \t \n \r \x0B \n "),
            "trim me PHP!"
        );

        assert_eq!(
            "trim me RUST! \0 \t \n \r \x0B \n ".trim_right(),
            "trim me RUST! \0"
        );
    }
}
