/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * The `name` pattern used by the lexer to tokenize the templates.
 *
 * Written as regular expressions (perl-style).
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use regex;
use regex::Error as regexError;

/////////////
// exports //
/////////////

pub type ExtractIter<'a, 'b> = super::ExtractIter<'a, 'b, Pattern>;
pub use lexer::error::{LexerError, LexerErrorCode};

#[derive(PartialEq)]
pub struct Pattern {
    regex: regex::Regex,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct ItemData {
    pub position: (usize, usize),
    pub number: Number,
}

#[derive(Debug, PartialEq)]
pub enum Number {
    Integer(u64),
    Floating(f64)
}

impl Number {

}

impl Pattern {
    pub fn new() -> Result<Pattern, regexError> {
        Ok(Pattern {
            regex: try_new_regex!(r"\A[0-9]+(\.[0-9]+)?"),
        })
    }   // orig: /[0-9]+(?:\.[0-9]+)?/A'
}

impl<'t> super::Extract<'t> for Pattern {
    type Item = Result<ItemData, LexerError>;

    fn regex(&self) -> &regex::Regex {
        &self.regex
    }

    fn item_from_captures(&self, captures: &regex::Captures) -> Result<ItemData, LexerError> {
        let number_string = captures.at(0).unwrap_or_else(|| unreachable!());

        let number = match captures.pos(1) { // float or int?
            Some(_) => match number_string.parse::<f64>() {
                Ok(float)
                    => Number::Floating(float),
                Err(e)
                    => return err!(LexerErrorCode::InvalidValue, number_string)
                        .caused_by(e)
                        .into(),
            },
            None => match number_string.parse::<u64>() {
                Ok(int)
                    => Number::Integer(int),
                Err(e)
                    => return err!(LexerErrorCode::InvalidValue, number_string)
                        .caused_by(e)
                        .into(),
            },
        };

        Ok(ItemData {
            position: match captures.pos(0) {
                Some(position) => position,
                _ => unreachable!(),
            },
            number: number,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use lexer::patterns::Extract;

    #[test]
    pub fn extract() {
        let pattern = Pattern::new().unwrap();

        assert!(
            pattern.extract(&r"{Lorem Ipsum").is_none()
        );

        assert_eq!(
            pattern.extract(&r"1.Lorem Ipsum").unwrap().unwrap(),
            ItemData {
                position: (0,1),
                number: Number::Integer(1),
            }
        );

        assert_eq!(
            pattern.extract(&r"123abc").unwrap().unwrap(),
            ItemData {
                position: (0,3),
                number: Number::Integer(123),
            }
        );

        assert_eq!(
            pattern.extract(&r"12345 ...").unwrap().unwrap(),
            ItemData {
                position: (0,5),
                number: Number::Integer(12345),
            }
        );

        assert_eq!(
            pattern.extract(&r"123.456 Schalom").unwrap().unwrap(),
            ItemData {
                position: (0,7),
                number: Number::Floating(123.456),
            }
        );
    }
}