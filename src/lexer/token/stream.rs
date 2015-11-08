// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Represents a token stream
///
/// @author Colin Kiegel <kiegel@gmx.de>


/////////////
// imports //
/////////////

use std::fmt;
use std::convert::Into;
use std::ops::Deref;
use lexer::token::{self, Token, Type};
use template;
use lexer::error::{TokenError, TokenErrorCode};
use lexer::job::Cursor;

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
        write!(f, "{line}:{column}",
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

    pub fn expect<T>(&self, pattern: T, reason: Option<&str>) -> Result<&Item, TokenError>
        where T: token::Pattern
    {
        if pattern.matches(self.token()) {
            Ok(&self)
        } else {
            let mut error = err!(TokenErrorCode::UnexpectedToken,
                "Expected token {t:?} but found item {x:?} at {p:?}",
                t = pattern, x = self.token(), p = self.position());
            if let Some(x) = reason {
                error = error.explain(x.to_string());
            }
            error.into()
        }
    }
}

//#[derive(Default)]
pub struct Stream<'a> {
    items: Vec<Item>,
    template: &'a template::Raw,
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
            template: template, // #TODO:380 rename path??
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

    pub fn template(&self) -> &template::Raw {
        self.template
    }

    pub fn _is_eof(&self) -> bool {
        match self.items.last()  {
            Some(x) => x.token.is_type(Type::Eof),
            None    => true,
        }
    }

    pub fn len(&self) -> usize {
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
        write!(f, "[\n\t{}\n]", v.join("\n\t"))
    }
}

impl<'a> fmt::Debug for Stream<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let v: Vec<String> = self.items.iter().map(|i| format!("{:?}", i.token)).collect();
        write!(f, "[\n\t{}\n]", v.join("\n\t"))
    }
}

// #TODO:70 add another token_iter() to the main implementation [using .map(|i| i.into()) as MapIterator]
impl<'a> IntoIterator for Stream<'a> {
    type Item = self::Item;
    type IntoIter = <Vec<self::Item> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
