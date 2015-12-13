// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! The `block line` pattern used by the lexer to tokenize the templates.
///
/// Written as regular expressions (perl-style).

use super::Options;
use regex;
use regex::Error as regexError;
use std::rc::Rc;
use api::error::{Traced, ErrorExt};

pub type ExtractIter<'a, 'b> = super::ExtractIter<'a, 'b, Pattern>;
pub use engine::parser::lexer::LexerError;


#[derive(Debug, PartialEq)]
pub struct Pattern {
    regex: regex::Regex,
    options: Rc<Options>,
}

#[derive(Debug, PartialEq)]
pub struct ItemData {
    pub position: (usize, usize),
    pub line: usize,
}

impl Pattern {
    pub fn new(opt: &Rc<Options>) -> Result<Pattern, Traced<regexError>> {
        Ok(Pattern {
            regex: try_new_regex!(format!(r"\A\s*line\s+(\d+)\s*(?:{ws}{b1}\s*|{b1})",
                ws = opt.whitespace_trim.quoted(),
                b1 = opt.tag_block_end.quoted())),
            options: (*opt).clone(),
        })
    }   // orig: '/\s*line\s+(\d+)\s*'.$tag_block[1].'/As'
}

impl<'t> super::Extract<'t> for Pattern {
    type Item = Result<ItemData, Traced<LexerError>>;

    fn regex(&self) -> &regex::Regex {
        &self.regex
    }

    #[inline]
    fn item_from_captures(&self, captures: &regex::Captures) -> Result<ItemData, Traced<LexerError>> {
        Ok(ItemData {
            position: match captures.pos(0) {
                Some(position) => position,
                _ => unreachable!(),
            },
            line: match captures.at(1) {
                Some(x) => match x.parse::<usize>() {
                        Ok(line) => line,
                        Err(e) => {
                            return traced_err!(LexerError::InvalidInteger {
                                value: x.to_string(),
                                parse_error: e
                            })
                        },
                    },
                _ => unreachable!(),
            },
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use engine::parser::lexer::patterns::{Options, Extract};
    use std::rc::Rc;

    #[test]
    pub fn as_str() {
        let ref options = Rc::<Options>::default();
        let pattern = Pattern::new(options).unwrap();

        assert_eq!(
            pattern.as_str(),
            r"\A\s*line\s+(\d+)\s*(?:-%\}\s*|%\})"
        );
    }

    #[test]
    pub fn extract() {
        let ref options = Rc::<Options>::default();
        let pattern = Pattern::new(options).unwrap();

        assert_eq!(
            // u64::max_value() == 18446744073709551615
            // u32::max_value() == 4294967295
            pattern.extract(&r"   line   1234567890  %}").unwrap().unwrap(),
            ItemData {
                position: (0,24),
                line: 1234567890
            }
        );
    }

    #[test]
    pub fn extract_max_int() {
        let ref options = Rc::<Options>::default();
        let pattern = Pattern::new(options).unwrap();

        // u64::max_value() == 18446744073709551615
        // u32::max_value() == 4294967295
        let err = pattern.extract(&r"   line   1844674407370955161518446744073709551615  %}").unwrap().unwrap_err();

        if let LexerError::InvalidInteger { value: ref x, parse_error: _ } = *err.error() {
            assert_eq!(x, "1844674407370955161518446744073709551615");
        } else {
            panic!("expected `LexerError::InvalidInteger`");
        }
    }
}
