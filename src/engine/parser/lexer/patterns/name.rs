// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! The `name` pattern used by the lexer to tokenize the templates.
///
/// Written as regular expressions (perl-style).

use regex;
use regex::Error as regexError;
use api::error::Traced;

pub type ExtractIter<'a, 'b> = super::ExtractIter<'a, 'b, Pattern>;

#[derive(Debug, PartialEq)]
pub struct Pattern {
    regex: regex::Regex,
}

#[derive(Debug, PartialEq)]
pub struct ItemData<'a> {
    pub position: (usize, usize),
    pub name: &'a str,
}

impl Pattern {
    pub fn new() -> Result<Pattern, Traced<regexError>> {
        Ok(Pattern { regex: try_new_regex!(r"\A[a-zA-Z_\x7f-\xff][a-zA-Z0-9_\x7f-\xff]*") })
    }   // orig: '/[a-zA-Z_\x7f-\xff][a-zA-Z0-9_\x7f-\xff]*/A'
}

impl<'t> super::Extract<'t> for Pattern {
    type Item = ItemData<'t>;

    fn regex(&self) -> &regex::Regex {
        &self.regex
    }

    fn item_from_captures(&self, captures: &regex::Captures<'t>) -> ItemData<'t> {
        ItemData {
            position: match captures.pos(0) {
                Some(position) => position,
                _ => unreachable!(),
            },
            name: match captures.at(0) {
                Some(ref name) => name,
                _ => unreachable!(),
            },
        }
    }

    // overwrite for better performance, as long as we only need the position
    fn extract(&self, text: &'t str) -> Option<Self::Item> {
        self.find(text).map(|position| {
            ItemData {
                position: position,
                name: &text[position.0..position.1],
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use engine::parser::lexer::patterns::Extract;

    #[test]
    pub fn extract() {
        let pattern = Pattern::new().unwrap();

        assert_eq!(pattern.extract(&r"{Lorem Ipsum"), None);

        assert_eq!(pattern.extract(&r"123abc"), None);

        assert_eq!(pattern.extract(&r"Lorem Ipsum"),
                   Some(ItemData {
                       position: (0, 5),
                       name: "Lorem",
                   }));
    }
}
