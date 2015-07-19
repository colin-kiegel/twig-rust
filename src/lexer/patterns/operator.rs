/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * The `operator` pattern used by the lexer to tokenize the templates.
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
use compiler::ext::{UnaryOperator, BinaryOperator};

/////////////
// exports //
/////////////

pub type ExtractIter<'a, 'b> = super::ExtractIter<'a, 'b, Pattern>;


#[derive(Debug)]
pub struct Pattern {
    regex: regex::Regex,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct ItemData {
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

struct Builder {
    whitespace: regex::Regex,
}

impl Builder {
    fn new() -> Result<Builder, regexError> {
        Ok(Builder {
            whitespace: try!(regex::Regex::new(r"\s+"))
        })
    }

    fn operators_to_regex(&self, unary: &Vec<UnaryOperator>, binary: &Vec<BinaryOperator>) -> String {
        let mut operators : Vec<(usize, &str)> = Vec::with_capacity(1 + unary.len() + binary.len());

        operators.push(("=".len(), "="));

        for x in unary.iter()  { operators.push((x.repr.len(), &x.repr)) }
        for x in binary.iter() { operators.push((x.repr.len(), &x.repr)) }

        // sort in reverse order (i.e. descending): 10,9,8,7,6 ..
        // -> not sure why we do this(!)
        operators.sort_by(|&(ref len_a,_),&(ref len_b,_)| len_b.cmp(len_a));

        // collect regex "patternA|patternB|.."
        return operators.iter().map(|&(_, op)| self.operator_to_regex(op))
             .collect::<Vec<String>>().connect("|");
    }

    fn operator_to_regex(&self, operator: &str) -> String {
        // an operator that ends with a character must be followed by
        // a whitespace or a parenthesis
        let mut rx : String = regex::quote(operator);

        rx = self.whitespace.replace_all(&rx, r"\s+");
        // orig: r = preg_replace('/\s+/', '\s+', r);

        if let Some(c) = operator.chars().last() {
            if c.is_alphabetic() { // TODO:  regex does not support lookahead(!)
                panic!("operator_to_regex(): operator ends in alphanumeric character (!)");
                // r.push(r"(?=[\s()])");
            }
        }

        return rx;
    }
}

#[allow(dead_code, unused_variables)]
impl Pattern {
    pub fn new(unary: &Vec<UnaryOperator>, binary: &Vec<BinaryOperator>) -> Result<Pattern, regexError> {
        Ok(Pattern {
            regex: {
                let regex = try!(Builder::new()).operators_to_regex(unary, binary);

                try!(regex::Regex::new(&regex))
            }
        })
    }
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
            tag: match captures.at(1) {
                _ => unreachable!(),
            },
        }
    }
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        self.regex == other.regex
    }
}


// TODO: test-cases?

// #[cfg(test)]
// mod test {
//     use super::*;
//     use environment::Environment;
//     use lexer::patterns::Extract;
//     use std::rc::Rc;
//
//     #[test]
//     pub fn as_str() {
//         let environment = Rc::<Environment>::default();
//         let pattern = Pattern::new(environment).unwrap();
//
//         assert_eq!(
//             pattern.as_str(),
//             r""
//         );
//     }
//
//     #[test]
//     pub fn extract() {
//         let environment = Rc::<Environment>::default();
//         let pattern = Pattern::new(environment).unwrap();
//
//         assert_eq!(
//             pattern.extract(&r"Lorem Ipsum"),
//             None
//         );
//     }
// }
