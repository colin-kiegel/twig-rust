/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Represents a token stream
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use std::convert::Into;
use super::{Token, Type};
use template;

/////////////
// exports //
/////////////

#[allow(dead_code)]
#[derive(Debug)]
pub struct Item {
    token:      Token,
    position:   usize,
}

impl Into<Token> for Item {
    fn into(self) -> Token {
        self.token
    }
}

#[allow(dead_code)]
#[derive(Default)]
pub struct Stream<'a> {
    items: Vec<Item>,
    template: Option<&'a template::Raw>,
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl<'a> Stream<'a> {
    /// Constructor
    pub fn new(template: &'a template::Raw) -> Stream<'a> {
        Stream {
            items: Vec::new(),
            template: Some(template),
        }
    }

    pub fn push(&mut self, token: Token, position: usize) {
        self.items.push(Item {
            token: token,
            position: position,
        });
    }

    pub fn is_eof(&self) -> bool {
        match self.items.last()  {
            Some(x) => x.token.is_type(Type::Eof),
            None    => true,
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl<'a> ToString for Stream<'a> {
    /// Returns a string representation of the token stream.
    fn to_string(&self) -> String {
        let v: Vec<String> = self.items.iter().map(|i| i.token.to_string()).collect();
        v.connect("\n")
    }
}

impl<'a> IntoIterator for Stream<'a> {
    type Item = self::Item;
    type IntoIter = <Vec<self::Item> as IntoIterator>::IntoIter;//ExactSizeIterator;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
