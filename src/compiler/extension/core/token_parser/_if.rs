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

use compiler::extension::api::TokenParser;
use parser::{Node, Job, ParserError, ParserErrorCode};
use lexer::token::stream::Item;
use compiler::extension::api::operator::Precedence;
use compiler::extension::api::token_parser::TestResult;
use lexer::Token;

/////////////
// exports //
/////////////

#[derive(Debug, Default)]
pub struct If;

impl TokenParser for If {
    fn tag(&self) -> &'static str {
        "if"
    }

    fn parse(&self, job: &mut Job, item: &Item) -> Result<Box<Node>, ParserError> {
        let if_test = try!(job.parse_expression(Precedence(0)));
        try!(job.cursor().next_expect(Token::BlockEnd, Some("if-Block must be closed")));
        let if_body = try!(job.sub_parse_until(&is_if_fork));
        let mut conditionals = vec![(if_test, if_body)];
        let mut _default : Option<Vec<Box<Node>>> = None;

        'a: loop {
            let x = try!(job.cursor().next().ok_or_else({||
                return err!(ParserErrorCode::Eof,
                    "Unexpected end of template. Twig was looking for \
                    the following tags \"else\", \"elseif\", or \"endif\" \
                    to close the \"if\" block started at {p} in {j}",
                    p = item.position(),
                    j = job)
            }));

            match x.token().value_as_str() {
                Some("else") => {
                    try!(job.cursor().next_expect(Token::BlockEnd, Some("else-Block must be closed")));

                    let node = try!(job.sub_parse_until(&is_if_end));
                    _default = Some(node);
                },
                Some("elseif") => {
                    let elseif_test = try!(job.parse_expression(Precedence(0)));
                    try!(job.cursor().next_expect(Token::BlockEnd, Some("elseif-Block must be closed")));
                    let elseif_body = try!(job.sub_parse_until(&is_if_fork));

                    conditionals.push((elseif_test, elseif_body));
                },
                Some("endif") => {
                    try!(job.cursor().next_expect(Token::BlockEnd, Some("endif-Block must be closed")));

                    break 'a;
                },
                _ => {
                    return err!(ParserErrorCode::Eof,
                            "Unexpected end of template. Twig was looking for \
                            the following tags \"else\", \"elseif\", or \"endif\" \
                            to close the \"if\" block started with token {token:?} \
                            at {p} in {j}",
                            token = x.token(),
                            p = x.position(),
                            j = job)
                        .into()
                }
            }
        }

        unimplemented!()
        //return Ok(node::If::boxed(/* conditionals ,*/ default, item.position(), self.tag()));
    }
}

pub fn is_if_fork(item: &Item) -> TestResult {
    match item.token().value_as_str() {
        Some(ref x) => match *x {
            "else"
            | "elseif"
            | "endif" => TestResult::KeepToken,
            _ => TestResult::Continue
        },
        _ => TestResult::Continue
    }
}

pub fn is_if_end(item: &Item) -> TestResult {
    match item.token().value_as_str() {
        Some("endif") => TestResult::KeepToken,
        _ => TestResult::Continue
    }
}
