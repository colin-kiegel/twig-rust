// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Expression parser.

use parser::node;
use parser::api::Node;
use parser::job::Job;
use parser::{ParserError, ParserErrorCode};
use engine::extension::api::BinaryOperator;
use engine::extension::api::operator::Precedence;
use engine::ExtensionRegistry;
use lexer::token::{Token, Punctuation, BracketType};
use std::rc::Rc;
use error::{Dump, ErrorCode};

#[derive(Debug)]
pub struct Expression; // dummy

#[derive(Debug)]
#[allow(dead_code)]
pub struct ExpressionParser {
    ext: Rc<ExtensionRegistry>,
}

impl ExpressionParser {
    pub fn new(ext: &Rc<ExtensionRegistry>) -> ExpressionParser {
        ExpressionParser {
            ext: ext.clone(),
        }
    }

    // orig. parse_expression
    pub fn parse(&self, job: &mut Job, precedence: Precedence) -> Result<Box<Node>, ParserError> {
        let expr = try!(self.primary(job));

        let _token = try!(job.mut_cursor().peek_token().ok_or_else(|| {
            ParserErrorCode::UnexpectedEof {
                expected: None,
                reason: Some("Found unclosed expression"),
                cursor: job.mut_cursor().dump()
            }.at(loc!())
        }));

        // while self.is_binary(token) {
        //     let operator = try!(self.binary_operator(token));
        //
        //     if operator.prec >= precedence {
        //         unimplemented!();//self.parser.stream().next();
        //
        //         // if operator.is_callable() {
        //         //     expr = operator.call(self.parser, expr);
        //         // } else {
        //         //     // let rec_precedence = match operator.associativity {
        //         //     //     Assoc::Left => operator.precedence + 1,
        //         //     //     Assoc::Right => operator.precedence,
        //         //     // }
        //         //     // let rec_expr = self.parse(self);
        //         //     // let class = operator.class();
        //         //     unimplemented!() // orig: new $class()
        //         // }
        //         //
        //         // token = self.parser.current_token();
        //     }
        // }

        return Ok(match precedence {
        //    Precedence(0) => self.parse_conditional_expression(&expr),
            _ => expr});
    }

    pub fn primary(&self, job: &mut Job) -> Result<Box<Node>, ParserError> {
        match *try!(job.mut_cursor().peek_token().ok_or_else(|| {
            ParserErrorCode::UnexpectedEof {
                expected: None,
                reason: Some("Expected to find an expression"),
                cursor: job.mut_cursor().dump(),
            }.at(loc!())
        })) {
            Token::Operator(ref _op) => unimplemented!(),
            Token::Punctuation(Punctuation::OpeningBracket(BracketType::Round)) => {
                unimplemented!()
            },
            _ => {}
        }

        return self.parse_primary_expression(job)
    }

    pub fn is_binary(&self, _token: &Token) -> bool {
        // #TODO:280 refactor
        // a) move this to token
        // b) or merge with binary_operator() below to `binary_operator -> Result<Option<>,>`, so `try!(self.binary_operator(token))` will yield an Option, which we can use in loop `while let Some(binary) = try!(self.binary_operator(token))`
        unimplemented!()
    }

    pub fn binary_operator(&self, token: &Token) -> Result<&BinaryOperator, ParserError> {
        // TODO refactor
        // merge with `is_binary` above - we may then get rid of these errors (!)
        let name = try!(token.value().ok_or_else(|| {
            unimplemented!()
            // ParserErrorCode::Unreachable {
            //     reason: format!("Could not parse binary operator, because the token type {type:?} has no value",
            //     type = token.get_type()),
            //     job: ()
            // }.at(loc!())
        }));

        let operator = try!(self.ext.operators_binary().get(&name).ok_or_else(|| {
            ParserErrorCode::UnexpectedBinaryOperator {
                name: name.to_string(),
                job: unimplemented!()
            }.at(loc!())
        }));

        Ok(operator)
    }

    pub fn parse_conditional_expression(&self, _expr: &Box<Node>) -> Box<Node> {
        unimplemented!()
    }

    pub fn parse_primary_expression(&self, job: &mut Job) -> Result<Box<Node>, ParserError> {
        // TODO: Check if we can call next_token() immediately
        //      instead of peek() + next(), where the next() call
        //      seems to happen in every match-branch (double check!)
        //      this refactoring should be done with sufficient tests.
        let item = try!(job.mut_cursor().peek().ok_or_else(|| {
            ParserErrorCode::UnexpectedEof {
                expected: None,
                reason: Some("Unclosed primary expression"),
                cursor: job.mut_cursor().dump()
            }.at(loc!())
        }));

        let node: Box<Node> = match *item.token() {
            Token::Name(ref value) => {
                job.mut_cursor().next_token();

                match value.as_ref() {
                    "true" | "TRUE" => { unimplemented!() },
                    "false" | "FALSE" => { unimplemented!() },
                    "none" | "NONE" | "null" | "NULL" => { unimplemented!() },
                    _ => if job.mut_cursor().peek_token() ==
                        Some(&Token::Punctuation(Punctuation::OpeningBracket(BracketType::Round))) {
                            unimplemented!()
                        } else {
                            node::expression::Name::boxed(value.clone(), item.position())
                        }
                }
            },
            Token::IntegerNumber(_) => unimplemented!(),
            Token::FloatingNumber(_) => unimplemented!(),
            Token::String(_) => unimplemented!(),
            Token::_InterpolationStart => unimplemented!(),
            Token::Operator(_) => unimplemented!(),
            Token::Punctuation(_) => unimplemented!(),
            Token::_Eof
            | Token::Text(_)
            | Token::_InterpolationEnd
            | Token::BlockStart
            | Token::ExpressionStart
            | Token::BlockEnd
            | Token::ExpressionEnd => {
                unimplemented!()
            }
        };

        self.parse_postfix_expression(job, node)
    }

    fn parse_postfix_expression(&self, job: &mut Job, node: Box<Node>) -> Result<Box<Node>, ParserError> {
        let mut node = node;

        'a: while let Token::Punctuation(ref punc) = *try!(
            job.mut_cursor().peek_token().ok_or_else(|| {
                ParserErrorCode::UnexpectedEof {
                    expected: None,
                    reason: Some("Unclosed postfix expression"),
                    cursor: job.mut_cursor().dump(),
                }.at(loc!())
        })) {
            match *punc {
                Punctuation::Dot
                | Punctuation::OpeningBracket(BracketType::Square) => {
                    node = try!(self.parse_subscript_expression(job, node));
                },
                Punctuation::VerticalBar => {
                    node = try!(self.parse_filter_expression(job, node));
                },
                _ => break 'a,
            }
        }

        return Ok(node);
    }

    fn parse_subscript_expression(&self, _job: &mut Job, _node: Box<Node>) -> Result<Box<Node>, ParserError> {
        unimplemented!()
    }

    fn parse_filter_expression(&self, _job: &mut Job, _node: Box<Node>) -> Result<Box<Node>, ParserError> {
        unimplemented!()
    }
}
