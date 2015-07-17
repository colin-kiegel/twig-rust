/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * The `block` pattern used by the lexer to tokenize the templates.
 *
 * Written as regular expressions (perl-style).
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use super::Options;
use regex;
use regex::Error as regexError;
use std::rc::Rc;

/////////////
// exports //
/////////////

pub type ExtractIter<'a, 'b> = super::ExtractIter<'a, 'b, Pattern>;

#[derive(PartialEq)]
pub struct Pattern {
    regex: regex::Regex,
    options: Rc<Options>,
}

#[derive(Debug, PartialEq)]
pub struct ItemData {
    pub position: (usize, usize)
}

impl Pattern {
    pub fn new(opt: Rc<Options>) -> Result<Pattern, regexError> {
        Ok(Pattern {
            regex: try_new_regex!(format!(r"\A\s*(?:{ws}{b1}\s*|{b1})\n?",
                ws = opt.whitespace_trim.quoted(),
                b1 = opt.tag_block_end.quoted())),
            options: opt,
        })
    }   // orig: '/\s*(?:'.$whitespace_trim.$tag_block[1].'\s*|\s*'.$tag_block[1].')\n?/A'
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
            }
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
    use lexer::patterns::{Options, Extract};
    use std::rc::Rc;

    #[test]
    pub fn as_str() {
        let options = Rc::<Options>::default();
        let pattern = Pattern::new(options).unwrap();

        assert_eq!(
            pattern.as_str(),
            r"\A\s*(?:-%\}\s*|%\})\n?"
        );
    }

    #[test]
    pub fn extract() {
        let options = Rc::<Options>::default();
        let pattern = Pattern::new(options).unwrap();

        assert_eq!(
            pattern.extract(&r"Lorem Ipsum -%}"),
            None
        );

        assert_eq!(
            pattern.extract(&r"  -%} Lorem Ipsum").unwrap(),
            ItemData {
                position: (0, 6),
            }
        );

        assert_eq!(
            pattern.extract(&r"%}Lorem Ipsum").unwrap(),
            ItemData {
                position: (0, 2),
            }
        );
    }
}
