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

/////////////
// exports //
/////////////


type PeekableTokenStreamIterator<'a> = iter::Peekable<::std::slice::Iter<'a, token::stream::Item>>;

#[allow(dead_code)]
//#[derive(Debug)]
pub struct Job<'a> {
    parser: &'a Parser,    // orig: env

    // state:
    stream: PeekableTokenStreamIterator<'a>,
    state: State,
    stack: Vec<State>,
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

impl<'a> Job<'a> {
    #[allow(dead_code)] // #TODO:700 testcase
    pub fn new(tokens: &'a token::Stream, parser: &'a Parser) -> Job<'a> {
        let mut _j = Job {
            stream: tokens.iter().peekable(),
            parser: parser,
            state: State::default(),
            stack: Vec::new(),
        };

        unimplemented!();
        // j.handlers.set_parser(j); // TODO
        //return _j;
    }

    #[allow(unused_mut)]
    #[allow(dead_code)] // #TODO:710 testcase
    pub fn parse(mut self: Job<'a>, test: String, drop_needle: bool) -> Result<(), ParserError> {

        self.stack.push(self.state);
        self.state = State::default();

        let _body = match self.sub_parse(test, drop_needle) {
            Err(e) => return Err(e),
            _ => unimplemented!()
            // Some(body) => {
            //     if self.state.parent.is_some() {
            //         self.filter_body_nodes(body).unwrap_or_else(|| Node::new())
            //     } else {
            //         body
            //     }
            // }
        };

        unimplemented!()
        // let node = NodeModule::new(
        //     NodeBody::new(
        //         body,
        //         self.state.parent,
        //         Node::new(self.state.blocks),
        //         Node::new(self.state.macros),
        //         Node::new(self.state.traits),
        //         self.state.embedded_templates,
        //         self.filename()));
        // self.state = self.stack.pop.unwrap();
        //
        // let traverser = NodeTraverser::new(compiler, self.visitors);
        // node = traverser.traverse(node);
        //
        // return node;
    }

    pub fn sub_parse(&mut self, _test: String, _drop_needle: bool) -> Result<(), ParserError> {
        // let line = self.current_token().line();
        let mut rv : Vec<Box<node::Node>> = Vec::new();

        while let Some(item) = self.stream.next() {
            match *item.token() {
                Token::Text(ref value) => {
                    rv.push(node::Text::new(value, item.position()));
                },
                Token::ExpressionStart => {
                    let _expr = self.parser.parse_expression(self, Precedence(0));
                    try!(self.expect_token(Token::ExpressionEnd));

                    // rv.push(node::Print::new(expr, item.position()));
                    unimplemented!() // TODO
                },
                Token::BlockStart => {
                    unimplemented!() // TODO
                },
                _ => return err!(ParserErrorCode::InvalidState)
                    .explain(format!("Parser ended up in unsupported state with token {token:?} at {pos}.",
                        token = item.token(), pos = item.position()))
                    .into()
            }
        }

        unimplemented!()
    }

    pub fn current_token(&self) -> &Token {
        unimplemented!()
    }

    pub fn expect_token(&mut self, token: Token) -> Result<(), ParserError> {
        match self.stream.peek() {
            None => return err!(ParserErrorCode::Eof)
                .explain(format!("Expected token {:?}.", token))
                .into(),
            Some(item) => {
                if *item.token() == token { return Ok(()) };

                return err!(ParserErrorCode::UnexpectedToken)
                    .explain(format!("Expected token {t:?} but found item {x:?} at {p:?}",
                        t = token, x = item.token(), p = item.position()))
                    .into();
            }
        }
    }
}

// #TODO:500 switch to Debug-Builder once stable
impl<'a> fmt::Debug for Job<'a> {
    fn fmt(&self, _f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        unimplemented!()
        // write!(f, "[\n\
        //     Tokenstream: {stream}\n\
        //     ]",
        //     stream = self.stream
        // )
    }
}
