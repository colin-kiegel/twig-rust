/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * The tokens_start pattern used by the lexer to tokenize the templates.
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

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct ItemData {
    pub position: (usize, usize),
    pub whitespace_trim: bool,
    pub tag: Tag,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Tag {
    Block,
    Comment,
    Variable,
}

impl Pattern {
    pub fn new(opt: Rc<Options>) -> Result<Pattern, regexError> {
        Ok(Pattern {
            regex: try_new_regex!(format!(r"({v0}|{b0}|{c0})({ws})?",
                ws = opt.whitespace_trim.quoted(),
                b0 = opt.tag_block_start.quoted(),
                c0 = opt.tag_comment_start.quoted(),
                v0 = opt.tag_variable_start.quoted())),
            options: opt,
        })
    }   // orig: '/('.$tag_variable[0].'|'.$tag_block[0].'|'.$tag_comment[0].')('.$whitespace_trim.')?/s'
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
            whitespace_trim: match captures.at(2) {
                Some(_) => true,
                None    => false,
            },
            tag: match captures.at(1) {
                Some(x) if x == self.options.tag_block_start.raw()    => Tag::Block,
                Some(x) if x == self.options.tag_comment_start.raw()  => Tag::Comment,
                Some(x) if x == self.options.tag_variable_start.raw() => Tag::Variable,
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
            r"(\{\{|\{%|\{\#)(-)?"
        );
    }

    #[test]
    pub fn extract() {
        let options = Rc::<Options>::default();
        let pattern = Pattern::new(options).unwrap();

        assert_eq!(
            pattern.extract(&r"{-{"),
            None
        );

        assert_eq!(
            pattern.extract(&r"{{-"),
            Some(ItemData {
                position: (0,3),
                whitespace_trim: true,
                tag: Tag::Variable
            }));

        assert_eq!(
            pattern.extract(&r"{{"),
            Some(ItemData {
                position: (0,2),
                whitespace_trim: false,
                tag: Tag::Variable
            }));
    }
}
