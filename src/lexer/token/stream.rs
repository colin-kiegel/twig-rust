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
use std::ops::Deref;
use super::{Token, Type};
use template::{self, Cursor};

/////////////
// exports //
/////////////

#[derive(Debug, Default, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "line {line} column {column}",
            line = self.line,
            column = self.column)
    }
}

#[derive(Debug)]
pub struct Item {
    token: Token,
    position: Position,
}

impl Item {
    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn position(&self) -> &Position {
        &self.position
    }
}

#[derive(Default)]
pub struct Stream<'a> {
    items: Vec<Item>,
    _template: Option<&'a template::Raw>,
}

impl Into<Token> for Item {
    fn into(self) -> Token {
        self.token
    }
}

impl Deref for Item {
    type Target = Token;

    fn deref(&self) -> &Token {
        &self.token
    }
}

pub type Iter<'a> = ::std::slice::Iter<'a, Item>;

pub struct DerefIter<'a> {
    items: ::std::slice::Iter<'a, Item>,
}

impl<'a> Iterator for DerefIter<'a> {
    type Item = &'a Token;

    fn next(&mut self) -> Option<&'a Token> {
        self.items.next().map(|i| &i.token)
    }
}

#[allow(unused_variables)]
impl<'a> Stream<'a> {
    /// Constructor
    pub fn new(template: &'a template::Raw) -> Stream<'a> {
        Stream {
            items: Vec::new(),
            _template: Some(template), // TODO rename path??
        }
    }

    pub fn push(&mut self, token: Token, cursor: &Cursor) {
        self.items.push(Item {
            token: token,
            position: Position {
                line: cursor.line(),
                column: cursor.column(),
            },
        });
    }

    pub fn template(&self) -> Option<&template::Raw> {
        self._template
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

    pub fn as_vec(&self) -> &Vec<Item> {
        &self.items
    }

    pub fn iter(&'a self) -> Iter<'a> {
        (&self.items).into_iter()
    }

    pub fn deref_iter(&'a self) -> DerefIter<'a> {
        DerefIter { items: (&self.items).into_iter() }
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
