/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * The `var` pattern used by the lexer to tokenize the templates.
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
pub struct CaptureData {
    pub position: (usize, usize),
    pub tag: Tag,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Tag {
    // Block,
    // Comment,
    // Variable,
}

impl Pattern {
    pub fn new(opt: Rc<Options>) -> Result<Pattern, regexError> {
        Ok(Pattern {
            regex: try_new_regex!(format!(r"\A\s*{ws}{v1}\s*|\s*{v1}",
                ws = opt.whitespace_trim.quoted(),
                v1 = opt.tag_variable_end.quoted())),
            options: opt,
        })
    }
}   // orig: '/\s*'.$whitespace_trim.$tag_variable[1].'\s*|\s*'.$tag_variable[1].'/A'

impl<'t> super::Extract<'t> for Pattern {
    type Item = CaptureData;

    fn regex(&self) -> &regex::Regex {
        &self.regex
    }

    fn item_from_captures(&self, captures: &regex::Captures) -> CaptureData {
        CaptureData {
            position: match captures.pos(0) {
                Some(position) => position,
                _ => unreachable!(),
            },
            tag: match captures.at(1) {
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
            r"\A\s*-\}\}\s*|\s*\}\}"
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

        unimplemented!();
    }
}