// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! The `name` pattern used by the lexer to tokenize the templates.
///
/// Written as regular expressions (perl-style).

use regex;
use regex::Error as regexError;

pub type ExtractIter<'a, 'b> = super::ExtractIter<'a, 'b, Pattern>;

#[derive(Debug, PartialEq)]
pub struct Pattern {
    regex: regex::Regex,
}

#[derive(Debug, PartialEq)]
pub struct ItemData<'a> {
    pub position: (usize, usize),
    pub escaped_string: &'a str,
}

impl<'a> ItemData<'a> {
    pub fn _unescape_string(&self) -> String {
        super::php_stripcslashes(self.escaped_string)
    }
}

impl Pattern {
    pub fn new() -> Result<Pattern, regexError> {
        Ok(Pattern {
            // #NOTE:0 Rusts regexes don't support lookarounds like `(?!\{)`,
            //       so we need to change behaviour slightly:
            //       - `#` must not be followed by `"` or `\\` which was legal before
            regex: try_new_regex!(r##"(?s)\A[^#"\\]*(?:(?:\\.|#)[^#"\\]*)*"##),
        })
    }   // orig: '/[^#"\\\\]*(?:(?:\\\\.|#(?!\{))[^#"\\\\]*)*/As'
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
            escaped_string: match captures.at(0) {
                Some(ref val) => val,
                _ => unreachable!(),
            }
        }
    }

    // overwrite for better performance, as long as we only need the position
    fn extract(&self, text: &'t str) -> Option<Self::Item> {
        self.find(text).map(|position|
            ItemData {
                position: position,
                escaped_string: &text[position.0..position.1],
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

        assert_eq!(
            pattern.extract(&r##"" "string two""##),
            Some(ItemData {
                position: (0, 0),
                escaped_string: "",
            })
        );

        assert_eq!(
            pattern.extract(&r##"123\.abc"def"##),
            Some(ItemData {
                position: (0, 8),
                escaped_string: r##"123\.abc"##
            })
        );

        assert_eq!(
            pattern.extract(&r"'Lorem' Ipsum"),
            Some(ItemData {
                position: (0,13),
                escaped_string: "'Lorem' Ipsum"
            })
        );
    }
}
