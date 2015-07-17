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

use std::fmt;
use std::convert::Into;
use super::{Token, Type};
use template;

/////////////
// exports //
/////////////

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

#[derive(Default)]
pub struct Stream<'a> {
    items: Vec<Item>,
    _template: Option<&'a template::Raw>,
}

#[allow(unused_variables)]
impl<'a> Stream<'a> {
    /// Constructor
    pub fn new(template: &'a template::Raw) -> Stream<'a> {
        Stream {
            items: Vec::new(),
            _template: Some(template),
        }
    }

    pub fn push(&mut self, token: Token, position: usize) {
        self.items.push(Item {
            token: token,
            position: position,
        });
    }

    pub fn _is_eof(&self) -> bool {
        match self.items.last()  {
            Some(x) => x.token.is_type(Type::Eof),
            None    => true,
        }
    }

    pub fn _len(&self) -> usize {
        self.items.len()
    }
}

impl<'a> fmt::Display for Stream<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let v: Vec<String> = self.items.iter().map(|i| format!("{}", i.token)).collect();
        write!(f, "[\n\t{}\n]", v.connect("\n\t"))
    }
}

impl<'a> fmt::Debug for Stream<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let v: Vec<String> = self.items.iter().map(|i| format!("{:?}", i.token)).collect();
        write!(f, "[\n\t{}\n]", v.connect("\n\t"))
    }
}

// TODO add another token_iter() to the main implementation [using .map(|i| i.into()) as MapIterator]
impl<'a> IntoIterator for Stream<'a> {
    type Item = self::Item;
    type IntoIter = <Vec<self::Item> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
