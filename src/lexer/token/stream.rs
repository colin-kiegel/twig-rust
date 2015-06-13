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
#[derive(Default)]
pub struct Stream {
    tokens: Vec<Token>,
    current: usize,
    template: Option<Rc<template::Raw>>,
//    filename: String,
}

impl ToString for Stream {
    /// Returns a string representation of the token stream.
    
    fn to_string(&self) -> String {
        let v: Vec<String> = self.tokens.iter().map(|x| x.to_string()).collect();
        v.connect("\n")
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl Stream {
    /// Constructor

    pub fn new(tokens: Vec<Token>, template: Rc<template::Raw>) -> Stream {
        Stream {
            tokens: tokens,
            current: 0,
            template: Some(template),
//            filename: filename.to_string(),
        }
    }
    
    pub fn is_eof(&self) -> bool {
        // TODO - switch to self.tokens.last() if it is safe
        self.tokens[self.current].is_type(Type::Eof)
    }
    
}
