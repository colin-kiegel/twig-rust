// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! The `block raw` pattern used by the lexer to tokenize the templates.
///
/// Written as regular expressions (perl-style).

use super::Options;
use regex;
use regex::Error as regexError;
use std::rc::Rc;
use api::error::Traced;

pub type ExtractIter<'a, 'b> = super::ExtractIter<'a, 'b, Pattern>;

#[derive(Debug, PartialEq)]
pub struct Pattern {
    regex: regex::Regex,
    options: Rc<Options>,
}

#[derive(Debug, PartialEq)]
pub struct ItemData {
    pub position: (usize, usize),
    pub tag: Tag,
}

#[derive(Debug, PartialEq)]
pub enum Tag {
    Raw,
    Verbatim,
}

impl Pattern {
    pub fn new(opt: &Rc<Options>) -> Result<Pattern, Traced<regexError>> {
        Ok(Pattern {
            regex: try_new_regex!(format!(r"\A\s*(raw|verbatim)\s*(?:{ws}{b1}\s*|{b1})",
                                          ws = opt.whitespace_trim.quoted(),
                                          b1 = opt.tag_block_end.quoted())),
            options: (*opt).clone(),
        })
    }   // orig: '/\s*(raw|verbatim)\s*(?:'.$whitespace_trim.$tag_block[1].'\s*|\s*'.$tag_block[1].')/As'
}

impl<'t> super::Extract<'t> for Pattern {
    type Item = ItemData;

    fn regex(&self) -> &regex::Regex {
        &self.regex
    }

    fn item_from_captures(&self, captures: &regex::Captures) -> ItemData {
        ItemData {
            position: match captures.pos(0) {
                Some(position) => position,
                _ => unreachable!(),
            },
            tag: match captures.at(1) {
                Some("raw") => Tag::Raw,
                Some("verbatim") => Tag::Verbatim,
                _ => unreachable!(),
            },
        }
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

        assert_eq!(pattern.as_str(), r"\A\s*(raw|verbatim)\s*(?:-%\}\s*|%\})");
    }

    #[test]
    pub fn extract() {
        let ref options = Rc::<Options>::default();
        let pattern = Pattern::new(options).unwrap();

        assert_eq!(pattern.extract(&r"Lorem Ipsum raw %}"), None);

        assert_eq!(pattern.extract(&r"   raw   %} Lorem").unwrap(),
                   ItemData {
                       position: (0, 11),
                       tag: Tag::Raw,
                   });

        assert_eq!(pattern.extract(&r"verbatim-%}     Lorem Ipsum").unwrap(),
                   ItemData {
                       position: (0, 16),
                       tag: Tag::Verbatim,
                   });
    }
}
