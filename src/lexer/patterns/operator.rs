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

use environment::Environment;
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
    environment: Rc<Environment>,
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

#[allow(dead_code, unused_variables)]
impl Pattern {
    pub fn new(env: Rc<Environment>) -> Result<Pattern, regexError> {
        Ok(Pattern {
            regex: {
                //$operators = array_merge(
                    //array('='),
                    // TODO array_keys($this->env->getUnaryOperators()),
                    // TODO array_keys($this->env->getBinaryOperators())
                //);

                //$operators = array_combine($operators, array_map('strlen', $operators));
                //arsort($operators);

                //$regex = array();
                //foreach ($operators as $operator => $length) {
                    // an operator that ends with a character must be followed by
                    // a whitespace or a parenthesis
                    //if (ctype_alpha($operator[$length - 1])) {
                    //    $r = preg_quote($operator, '/').'(?=[\s()])';
                    //} else {
                    //    $r = preg_quote($operator, '/');
                    //}

                    // an operator with a space can be any amount of whitespaces
                    //$r = preg_replace('/\s+/', '\s+', $r);

                    //$regex[] = $r;
                //}

                //return '/'.implode('|', $regex).'/A';
                unimplemented!()
            },
            environment: env,
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
