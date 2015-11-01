/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * A parser job
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use std::fmt;
use lexer::{token, Token};
use parser::error::*;
use parser::{node, Parser};
use compiler::extension::api::operator::Precedence;
use std::iter;
use template;
use parser::api::Node;

/////////////
// exports //
/////////////

pub mod cursor;
pub use self::cursor::Cursor;

type PeekableTokenStreamIterator<'a> = iter::Peekable<::std::slice::Iter<'a, token::stream::Item>>;

#[allow(dead_code)]
//#[derive(Debug)]
pub struct Job<'p, 'stream> {
    parser: &'p Parser,    // orig: env
    tokens: &'stream token::Stream<'stream>,

    // state:
    // stream: PeekableTokenStreamIterator<'stream>,
    cursor: Cursor<'stream>,
    state: State,
    stack: Vec<State>,
    template: &'stream template::Raw,
}

#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct State {
    parent: Option<()>,
    blocks: Vec<()>,
    block_stack: Vec<()>,
    macros: Vec<()>,
    reserved_macro_names: Vec<()>,
    imported_symbols: Vec<Vec<()>>,
    traits: Vec<()>,
    embedded_templates: Vec<()>,
}

impl<'p, 'stream> Job<'p, 'stream> {
    #[allow(dead_code)] // #TODO:700 testcase
    pub fn new(tokens: &'stream token::Stream, parser: &'p Parser) -> Job<'p, 'stream> {
        Job {
            tokens: tokens,
            //stream: tokens.iter().peekable(),
            cursor: Cursor::new(tokens),
            parser: parser,
            state: State::default(),
            stack: Vec::new(),
            template: tokens.template(),
        }
    }

    #[allow(unused_mut)]
    #[allow(dead_code)] // #TODO:710 testcase
    pub fn parse(mut self: Job<'p, 'stream>, test: String, drop_needle: bool) -> Result<template::Compiled, ParserError> {

        // NOTE: try to move this to other point
        //  - to avoid very first redundant push?
        //  - Better have unit tests forst
        self.stack.push(self.state);
        self.state = State::default();

        let nodes = match self.sub_parse(test, drop_needle) {
            Err(e) => return Err(e),
            Ok(nodes) => {
                if self.state.parent.is_some() {
                    unimplemented!()
                    // self.filter_body_nodes(body).unwrap_or_else(|| node::Virtual::boxed())
                } else {
                    nodes
                }
            }
        };

        let module = node::Module::new(
            node::Body::boxed(nodes),
            self.state.parent,
            self.state.blocks, // as nodes?
            self.state.macros, // as nodes?
            self.state.traits, // as nodes?
            self.state.embedded_templates,
            self.template.name());
        self.state = self.stack.pop().unwrap();

        // *IMPORTANT TODO*: move initialisation somewhere else(!)
        // let traverser = NodeTraverser::new(compiler, self.visitors);
        // module = traverser.traverse(module);

        let compiled = template::Compiled::new(module);

        return Ok(compiled);
    }

    pub fn sub_parse(&mut self, _test: String, _drop_needle: bool) -> Result<Vec<Box<Node>>, ParserError> {
        // let line = self.current_token().line();
        let mut nodes : Vec<Box<Node>> = Vec::new();

        while let Some(item) = self.cursor.next() {
            match *item.token() {
                Token::Text(ref value) => {
                    nodes.push(node::Text::boxed(value.to_string(), item.position()));
                },
                Token::ExpressionStart => {
                    let node = try!(self.parser.parse_expression(self, Precedence(0)));
                    try!(self.cursor().next_expect(Token::ExpressionEnd));

                    nodes.push(node::Print::boxed(node, item.position()));
                },
                Token::BlockStart => {
                    unimplemented!() // TODO
                },
                _ => return err!(ParserErrorCode::InvalidState,
                        "Parser ended up in unsupported state with token {token:?} at {pos} \
                        in {template:?} with tokens {tokens:?}.",
                        token = item.token(),
                        pos = item.position(),
                        template = self.template,
                        tokens = self.tokens)
                        .into()
            }
        }

        if nodes.is_empty() {
            return err!(ParserErrorCode::Logic)
                .explain(format!("Parser could not extract node from token stream."))
                .into()
        }

        return Ok(nodes);

        // ALTERNATIVELY SOME STRANGE NESTING (!)
        // if nodes.len() > 1 {
        //     let position = nodes[0].position();
        //     let virtual_node = node::Virtual::boxed(position);
        //
        //     return Ok(virtual_node);
        // }
        //
        // return match nodes.pop() {
        //     Some(node) => {
        //         Ok(node)
        //     },
        //     None => err!(ParserErrorCode::Logic)
        //         .explain(format!("Parser could not extract node from token stream."))
        //         .into(),
        // }
    }

    pub fn cursor(&mut self) -> &mut Cursor<'stream> {
        &mut self.cursor
    }
}

// #TODO:500 switch to Debug-Builder once stable
impl<'p, 'tpl> fmt::Debug for Job<'p, 'tpl> {
    fn fmt(&self, _f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        unimplemented!()
        // write!(f, "[\n\
        //     Tokenstream: {stream}\n\
        //     ]",
        //     stream = self.stream
        // )
    }
}
