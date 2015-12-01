// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! The `operator` pattern used by the lexer to tokenize the templates.
///
/// Written as regular expressions (perl-style).

use regex;
use regex::Error as regexError;
use extension::api::{UnaryOperator, BinaryOperator};
use engine::ExtensionRegistry;
use std::collections::HashMap;
use std::rc::Rc;

pub type ExtractIter<'a, 'b> = super::ExtractIter<'a, 'b, Pattern>;

#[derive(Debug)]
pub struct Pattern {
    regex: regex::Regex,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct ItemData<'a> {
    pub position: (usize, usize),
    pub operator: &'a str
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

    fn operators_to_regex(&self, unary: &HashMap<String, UnaryOperator>, binary: &HashMap<String, BinaryOperator>) -> String {
        let mut operators : Vec<(usize, &str)> = Vec::with_capacity(1 + unary.len() + binary.len());

        operators.push(("=".len(), "="));

        for (ref op_repr, _) in unary.iter()  { operators.push((op_repr.len(), op_repr)) }
        for (ref op_repr, _) in binary.iter() { operators.push((op_repr.len(), op_repr)) }

        // sort operators by length in reverse order (i.e. descending): 10,9,8,7,6 ..
        // -> such that a pattern `abc` is dominant over a subpattern `a`
        operators.sort_by(|&(ref len_a,_),&(ref len_b,_)| len_b.cmp(len_a));

        // collect regex "patternA|patternB|.."
        return operators.iter().map(|&(_, op)| self.operator_to_regex(op))
             .collect::<Vec<String>>().join("|");
    }

    fn operator_to_regex(&self, operator: &str) -> String {
        // an operator that ends with a character must be followed by
        // a whitespace or a parenthesis
        let mut rx : String = format!(r"\A{}", regex::quote(operator));

        // whitespaces shall match *any* whitespace
        rx = self.whitespace.replace_all(&rx, r"\s+");

        if let Some(c) = operator.chars().last() {
            if c.is_alphabetic() {
                panic!("operator_to_regex(): operator ends in alphanumeric character (!)");
                // #NOTE:70 regex does not support lookahead(!)
                //  -> orig: r.push(r"(?=[\s()])");
                //  -> #TODO:320 overwrite extract() in operator pattern and do aftermath
            }
        }

        return rx;
    }
}

#[allow(dead_code, unused_variables)]
impl Pattern {
    pub fn new(ext: &Rc<ExtensionRegistry>) -> Result<Pattern, regexError> {
        let unary = ext.operators_unary();
        let binary = ext.operators_binary();

        Ok(Pattern {
            regex: {
                let regex = try!(Builder::new()).operators_to_regex(unary, binary);

                try!(regex::Regex::new(&regex))
            }
        })
    }
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
            operator: match captures.at(0) {
                Some(name) => name,
                _ => unreachable!(),
            }
        }
    }
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        self.regex == other.regex
    }
}


// #TODO:650 test-cases?

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
