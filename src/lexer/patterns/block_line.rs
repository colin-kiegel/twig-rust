/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * The `block line` pattern used by the lexer to tokenize the templates.
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
pub use lexer::error::{LexerError, LexerErrorCode};


#[derive(PartialEq)]
pub struct Pattern {
    regex: regex::Regex,
    options: Rc<Options>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct CaptureData {
    pub position: (usize, usize),
    pub line: u64,
}

impl Pattern {
    pub fn new(opt: Rc<Options>) -> Result<Pattern, regexError> {
        Ok(Pattern {
            regex: try_new_regex!(format!(r"\A\s*line\s+(\d+)\s*{b1}",
                b1 = opt.tag_block_end.quoted())),
            options: opt,
        })
    }   // orig: '/\s*line\s+(\d+)\s*'.$tag_block[1].'/As'
}

impl<'t> super::Extract<'t> for Pattern {
    type Item = Result<CaptureData, LexerError>;

    fn regex(&self) -> &regex::Regex {
        &self.regex
    }

    #[inline]
    fn item_from_captures(&self, captures: &regex::Captures) -> Result<CaptureData, LexerError> {
        Ok(CaptureData {
            position: match captures.pos(0) {
                Some(position) => position,
                _ => unreachable!(),
            },
            line: match captures.at(1) {
                Some(x) => match x.parse::<u64>() {
                        Ok(line) => line,
                        Err(cause) => {
                            return err!(LexerErrorCode::InvalidValue, x, cause)
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
    use lexer::patterns::{Options, Extract};
    use std::rc::Rc;

    #[test]
    pub fn as_str() {
        let options = Rc::<Options>::default();
        let pattern = Pattern::new(options).unwrap();

        assert_eq!(
            pattern.as_str(),
            r"\A\s*line\s+(\d+)\s*%\}"
        );
    }

    #[test]
    pub fn extract() {
        let options = Rc::<Options>::default();
        let pattern = Pattern::new(options).unwrap();

        assert_eq!(
            // u64::max_value() == 18446744073709551615
            pattern.extract(&r"   line   1234567890  %}").unwrap().unwrap(),
            CaptureData {
                position: (0,24),
                line: 1234567890
            }
        );
    }

    #[test]
    pub fn extract_max_int() {
        let options = Rc::<Options>::default();
        let pattern = Pattern::new(options).unwrap();

        // u64::max_value() == 18446744073709551615
        let err = pattern.extract(&r"   line   1844674407370955161518446744073709551615  %}").unwrap().unwrap_err();

        assert_eq!(
            *err.code(),
            LexerErrorCode::InvalidValue
        );

        assert_eq!(
            err.details().message,
            Some("1844674407370955161518446744073709551615".to_string())
        );
    }
}