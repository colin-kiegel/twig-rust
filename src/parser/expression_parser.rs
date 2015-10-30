// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

///
///
/// @author Colin Kiegel <kiegel@gmx.de>

/////////////
// imports //
/////////////

use parser::node;
use parser::api::Node;
use parser::job::Job;
use parser::{ParserError, ParserErrorCode};
use compiler::extension::api::BinaryOperator;
use compiler::extension::api::operator::Precedence;
use compiler::ExtensionRegistry;
use lexer::token::{Token, Punctuation, BracketType};
use std::rc::Rc;

/////////////
// exports //
/////////////


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

        let _token = try!(job.cursor().peek_token().ok_or_else(|| {
            return err!(ParserErrorCode::Logic,
                "Unexpected end of token stream")
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
        match *try!(job.cursor().peek_token().ok_or_else(|| {
            return err!(ParserErrorCode::Logic,
                "Unexpected end of token stream")
        })) {
            Token::Operator(ref _op) => unimplemented!(),
            Token::Punctuation(Punctuation::OpeningBracket(BracketType::Round)) => {
                unimplemented!()
            },
            _ => {}
        }

        return self.parse_primary_expression(job)
    }

    pub fn is_binary(&self, _token: &Token) -> bool { // #TODO:280 move this to token??
        unimplemented!()
    }

    pub fn binary_operator(&self, token: &Token) -> Result<&BinaryOperator, ParserError> {
        let name = try!(token.value().ok_or_else(|| {
            err!(ParserErrorCode::Logic,
                "Could not parse binary operator, because the token type {type:?} has no value",
                type = token.get_type())
        }));

        let operator = try!(self.ext.operators_binary().get(&name).ok_or_else(|| {
            err!(ParserErrorCode::Logic,
                "The binary operator {name:?} is unknown to the compiler",
                name = name)
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
        let item = try!(job.cursor().peek().ok_or_else(|| {
            err!(ParserErrorCode::Logic,
                "Unexpected end of token stream")
        }));

        let node: Box<Node> = match *item.token() {
            Token::Name(ref value) => {
                job.cursor().next_token();

                match value.as_ref() {
                    "true" | "TRUE" => { unimplemented!() },
                    "false" | "FALSE" => { unimplemented!() },
                    "none" | "NONE" | "null" | "NULL" => { unimplemented!() },
                    _ => if job.cursor().peek_token() ==
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
            job.cursor().peek_token().ok_or_else(|| {
                err!(ParserErrorCode::Logic,
                    "Unexpected end of token stream")
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
