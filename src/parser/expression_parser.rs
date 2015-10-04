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

use parser::job::Job;
use parser::{ParserError, ParserErrorCode};
use compiler::extension::api::BinaryOperator;
use compiler::extension::api::operator::Precedence;
use compiler::ExtensionRegistry;
use lexer::Token;
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
    pub fn parse(&self, job: &mut Job, precedence: Precedence) -> Result<Expression, ParserError> {
        let expr = self.primary();
        let token = job.current_token();

        while self.is_binary(token) {
            let operator = try!(self.binary_operator(token));

            if operator.prec >= precedence {
                unimplemented!();//self.parser.stream().next();

                // if operator.is_callable() {
                //     expr = operator.call(self.parser, expr);
                // } else {
                //     // let rec_precedence = match operator.associativity {
                //     //     Assoc::Left => operator.precedence + 1,
                //     //     Assoc::Right => operator.precedence,
                //     // }
                //     // let rec_expr = self.parse(self);
                //     // let class = operator.class();
                //     unimplemented!() // orig: new $class()
                // }
                //
                // token = self.parser.current_token();
            }
        }

        return Ok(match precedence {
            Precedence(0) => self.parse_conditional_expression(&expr),
            _ => expr});
    }

    pub fn primary(&self) -> Expression {
        unimplemented!()
    }

    pub fn is_binary(&self, _token: &Token) -> bool { // TODO move this to token??
        unimplemented!()
    }

    pub fn binary_operator(&self, token: &Token) -> Result<&BinaryOperator, ParserError> {
        let name = match token.value() {
            None => return err!(ParserErrorCode::Logic)
                .explain(format!("Could not parse binary operator, \
                    because the token type {type:?} has no value",
                    type = token.get_type()))
                .into(),
            Some(value) => value
        };

        let operator = match self.ext.operators_binary().get(&name) {
            None => return err!(ParserErrorCode::Logic)
                .explain(format!("The binary operator {name:?} is unknown to the compiler",
                    name = name))
                .into(),
            Some(value) => value
        };

        Ok(operator)
    }

    pub fn parse_conditional_expression(&self, _expr: &Expression) -> Expression {
        unimplemented!()
    }

    pub fn parse_primary_expression(&self, _expr: &Expression) -> Expression {
        unimplemented!()
    }
}
