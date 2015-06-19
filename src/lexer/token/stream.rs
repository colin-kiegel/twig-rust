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

use super::Token;
use super::Type;
use template;

use std::rc::Rc;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Item {
    token:      Token,
    position:   usize,
}

#[allow(dead_code)]
#[derive(Default)]
pub struct Stream {
    items: Vec<Item>,
    template: Option<Rc<template::Raw>>,
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl Stream {
    /// Constructor
    pub fn new(template: Rc<template::Raw>) -> Stream {
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
}

impl ToString for Stream {
    /// Returns a string representation of the token stream.
    fn to_string(&self) -> String {
        let v: Vec<String> = self.items.iter().map(|i| i.token.to_string()).collect();
        v.connect("\n")
    }
}
