/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * The `verbatim_end` pattern used by the lexer to tokenize the templates.
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
pub use super::verbatim_start::Tag; // enum {Raw, Verbatim}

#[derive(Debug, PartialEq)]
pub struct Pattern {
    regex: regex::Regex,
    options: Rc<Options>,
}

#[derive(Debug, PartialEq)]
pub struct ItemData {
    pub position: (usize, usize),
    pub whitespace_trim: bool,
    pub tag: Tag,
}

impl Pattern {
    pub fn new(opt: Rc<Options>) -> Result<Pattern, regexError> {
        Ok(Pattern {
            regex: try_new_regex!(format!(r"{b0}({ws})?\s*(?:end(raw|verbatim))\s*(?:{ws}{b1}\s*|{b1})",
                ws = opt.whitespace_trim.quoted(),
                b0 = opt.tag_block_start.quoted(),
                b1 = opt.tag_block_end.quoted())),
            options: opt,
        })
    }   // orig: '/('.$tag_block[0].$whitespace_trim.'|'.$tag_block[0].')\s*(?:end%s)\s*(?:'.$whitespace_trim.$tag_block[1].'\s*|\s*'.$tag_block[1].')/s'
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
            whitespace_trim: match captures.at(1) {
                Some(_) => true,
                None    => false,
            },
            tag: match captures.at(2) {
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
    use lexer::patterns::{Options, Extract};
    use std::rc::Rc;

    #[test]
    pub fn as_str() {
        let options = Rc::<Options>::default();
        let pattern = Pattern::new(options).unwrap();

        assert_eq!(
            pattern.as_str(),
            r"\{%(-)?\s*(?:end(raw|verbatim))\s*(?:-%\}\s*|%\})"
        );
    }

    #[test]
    pub fn extract() {
        let options = Rc::<Options>::default();
        let pattern = Pattern::new(options).unwrap();

        assert_eq!(
            pattern.extract(&r"Lorem Ipsum"),
            None
        );

        assert_eq!(
            pattern.extract(&r"Lorem Ipsum {% endraw %} dolor").unwrap(),
            ItemData {
                position: (12,24),
                whitespace_trim: false,
                tag: Tag::Raw,
            }
        );

        assert_eq!(
            pattern.extract(&r"And then there was silence {%- endverbatim -%}       .").unwrap(),
            ItemData {
                position: (27,53),
                whitespace_trim: true,
                tag: Tag::Verbatim,
            }
        );
    }
}
