// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! The `name` pattern used by the lexer to tokenize the templates.
///
/// Written as regular expressions (perl-style).

pub use engine::parser::token::{Punctuation, BracketType};

#[derive(Debug, PartialEq)]
pub struct Pattern;

impl Pattern {
    pub fn instance() -> &'static Pattern {
        static INSTANCE: &'static Pattern = &Pattern;

        return INSTANCE;
    }
}

impl Pattern {
    // orig: '()[]{}?:.,|'
    pub fn extract(&self, text: &str) -> Option<Punctuation> {
        return match text.chars().next() {
            Some(c) => {
                match c {
                    '.' => Some(Punctuation::Dot),
                    ',' => Some(Punctuation::Comma),
                    ':' => Some(Punctuation::Colon),
                    '|' => Some(Punctuation::VerticalBar),
                    '?' => Some(Punctuation::QuestionMark),
                    '(' => Some(Punctuation::OpeningBracket(BracketType::Round)),
                    '[' => Some(Punctuation::OpeningBracket(BracketType::Square)),
                    '{' => Some(Punctuation::OpeningBracket(BracketType::Curly)),
                    ')' => Some(Punctuation::ClosingBracket(BracketType::Round)),
                    ']' => Some(Punctuation::ClosingBracket(BracketType::Square)),
                    '}' => Some(Punctuation::ClosingBracket(BracketType::Curly)),
                    _ => None,
                }
            }
            None => None,
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn extract() {
        let pattern = Pattern::instance();

        assert_eq!(pattern.extract("{Lorem Ipsum"),
                   Some(Punctuation::OpeningBracket(BracketType::Curly)));

        assert_eq!(pattern.extract("XYZ ,.-/()=?{[}]}"), None);
    }
}
