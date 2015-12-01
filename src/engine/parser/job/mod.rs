// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! A parser job.

use std::fmt;
use error::{Dump, ErrorCode};
use engine::parser::token::{self, Token, Type};
use engine::parser::{Parser, ParserError, ParserErrorCode};
use engine::node;
use extension::api::operator::Precedence;
use extension::api::token_parser::{Test, TestResult};
use std::iter;
use template;
use engine::Node;

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
            cursor: Cursor::new(tokens),
            parser: parser,
            state: State::default(),
            stack: Vec::new(),
            template: tokens.template(),
        }
    }

    pub fn parse(self: Job<'p, 'stream>) -> Result<template::Compiled, ParserError> {
        self.do_parse(None)
    }

    pub fn parse_until(self: Job<'p, 'stream>, test: &Test) -> Result<template::Compiled, ParserError> {
        self.do_parse(Some(test))
    }

    #[allow(unused_mut)]
    #[allow(dead_code)] // #TODO:710 testcase
    fn do_parse(mut self: Job<'p, 'stream>, test: Option<&Test>) -> Result<template::Compiled, ParserError> {

        // NOTE: try to move this to other point
        //  - to avoid very first redundant push?
        //  - Better have unit tests forst
        self.stack.push(self.state);
        self.state = State::default();

        let nodes = match self.do_sub_parse(test) {
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
        // let traverser = NodeTraverser::new(engine, self.visitors);
        // module = traverser.traverse(module);

        let compiled = template::Compiled::new(module);

        return Ok(compiled);
    }

    pub fn sub_parse(&mut self) -> Result<Vec<Box<Node>>, ParserError> {
        self.do_sub_parse(None)
    }

    pub fn sub_parse_until(&mut self, test: &Test) -> Result<Vec<Box<Node>>, ParserError> {
        self.do_sub_parse(Some(test))
    }

    fn do_sub_parse(&mut self, test: Option<&Test>) -> Result<Vec<Box<Node>>, ParserError> {
        // let line = self.current_token().line();
        let mut nodes : Vec<Box<Node>> = Vec::new();

        while let Some(item) = self.cursor.next() {
            match *item.token() {
                Token::Text(ref value) => {
                    nodes.push(node::Text::boxed(value.to_string(), item.position()));
                },
                Token::ExpressionStart => {
                    let node = try!(self.parse_expression(Precedence(0)));
                    try!(self.cursor.next_expect(Token::ExpressionEnd, None));

                    nodes.push(node::Print::boxed(node, item.position()));
                },
                Token::BlockStart => {
                    let item = {
                        let item = try!(self.cursor.peek_expect(Type::Name,
                            Some("A block must start with a tag name")));

                        if let Some(ref test) = test { // TODO: rename `test` to something more meaningful
                            match test(item) {
                                TestResult::KeepToken => {
                                    return Ok(nodes)
                                },
                                TestResult::DropToken => {
                                    self.cursor.next();
                                    return Ok(nodes)
                                },
                                // TestResult::Error(e) => {
                                //     return Err(e)
                                // },
                                TestResult::Continue => {}
                            }
                        }

                        self.cursor.next(); // we only peeked before
                        item
                    };

                    let subparser = {
                        // we can always destructure due to previous peek_expect(Type::Name)
                        let tag = if let Token::Name(ref tag) = *item.token() { tag }
                            else { unreachable!() };

                        try!(self.parser.tag_handler(tag)
                            .ok_or_else(||{ ParserErrorCode::NoTagHandler {
                                tag: tag.to_string(),
                                position: item.position().clone(),
                                job: self.dump()
                            }.at(loc!())
                        }))
                    };

                    let node = try!(subparser.parse(self, item));
                    nodes.push(node);
                },
                _ => return err!(ParserErrorCode::InvalidState {
                    item: item.dump(),
                    job: self.dump()
                })
            }
        }

        if nodes.is_empty() {
            return err!(ParserErrorCode::Unreachable {
                reason: "Parser could not extract node from token stream.".to_string(),
                job: self.dump(),
            })
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

    pub fn parser(&self) -> &Parser {
        self.parser
    }

    pub fn parse_expression (
        &mut self,
        precedence: Precedence
    ) -> Result<Box<Node>, ParserError>
    {
        self.parser.expression_parser.parse(self, precedence)
    }

    pub fn mut_cursor(&mut self) -> &mut Cursor<'stream> {
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

impl<'p, 'tpl> fmt::Display for Job<'p, 'tpl> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{template} with {cursor}.",
            template = self.template,
            cursor = self.cursor)
    }
}

pub type JobDump = String;

impl<'p, 'stream> Dump for Job<'p, 'stream> {
    type Data = JobDump;

    fn dump(&self) -> Self::Data {
        self.to_string()
    }
}
