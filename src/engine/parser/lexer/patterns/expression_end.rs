// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! The `expression` pattern used by the lexer to tokenize the templates.
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
}

impl Pattern {
    pub fn new(opt: &Rc<Options>) -> Result<Pattern, Traced<regexError>> {
        Ok(Pattern {
            regex: try_new_regex!(format!(r"\A\s*(?:{ws}{v1}\s*|{v1})",
                ws = opt.whitespace_trim.quoted(),
                v1 = opt.tag_variable_end.quoted())),
            options: opt.clone(),
        })
    }
}   // orig: '/\s*'.$whitespace_trim.$tag_variable[1].'\s*|\s*'.$tag_variable[1].'/A'

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
        }
    }

    // overwrite for better performance, as long as we only need the position
    fn extract(&self, text: &'t str) -> Option<Self::Item> {
        self.find(text).map(|position|
            ItemData {
                position: position
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
            r"\A\s*(?:-\}\}\s*|\}\})"
        );
    }

    #[test]
    pub fn extract() {
        let ref options = Rc::<Options>::default();
        let pattern = Pattern::new(options).unwrap();

        assert_eq!(
            pattern.extract(&r"Lorem Ipsum }}"),
            None
        );

        assert_eq!(
            pattern.extract(&r" }} Lorem Ipsum").unwrap(),
            ItemData {
                position: (0, 3),
            }
        );

        assert_eq!(
            pattern.extract(&r" -}} Lorem Ipsum").unwrap(),
            ItemData {
                position: (0, 5),
            }
        );
    }
}
