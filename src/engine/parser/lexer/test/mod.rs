// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Lexer Tests.

use engine::parser::lexer::Lexer;
use template::raw::Raw;
use std::rc::Rc;
use engine::parser::token::Token;
use engine::parser::token::Punctuation;
use engine::parser::lexer::LexerError;
use api::error::Traced;

//static TWIG_TEMPLATE_Token::Name : &'static str = "twig.template.html";
static TWIG_TEMPLATE_CODE : &'static str = include_str!("twig.template.html");

pub fn tokenize_err<'a>(code: &'a str) -> Traced<LexerError> {
    let tpl = Rc::new(Raw::new(code, "test-example"));
    let lxr = Lexer::default();

    println!("Template Code: {:?}", tpl.code);
    lxr.tokenize(&tpl).unwrap_err()
}

// TODO: read tokens from file too
/// #panics
/// when the tokenstream does not equal `tokens`
pub fn assert_tokenize(code: &str, tokens: Vec<Token>) {
    let tpl = Rc::new(Raw::new(code, "test-example"));
    let lxr = Lexer::default();

    println!("Template Code: {:?}", tpl.code);
    let tokenstream = lxr.tokenize(&tpl).unwrap();

    assert_eq!(
        tokenstream.into_iter().map(|i| i.into()).collect::<Vec<Token>>(),
        tokens
    );
}

#[test]
pub fn _tokenize() {
    assert_tokenize(
        TWIG_TEMPLATE_CODE,
        //TWIG_TEMPLATE_Token::Name, // would be nice to reintroduce this later
        vec![
            Token::Text("<!DOCTYPE html>\n<html>\n    <head>\n        <title>My Webpage</title>\n    </head>\n    <body>\n        <ul id=\"navigation\">\n        ".to_string()),
            Token::BlockStart,
            Token::Name("for".to_string()),
            Token::Name("item".to_string()),
            Token::Name("in".to_string()),
            Token::Name("navigation".to_string()),
            Token::BlockEnd,
            Token::Text("            <li><a href=\"".to_string()),
            Token::ExpressionStart,
            Token::Name("item".to_string()),
            Token::Punctuation(Punctuation::Dot),
            Token::Name("href".to_string()),
            Token::ExpressionEnd,
            Token::Text("\">".to_string()),
            Token::ExpressionStart,
            Token::Name("item".to_string()),
            Token::Punctuation(Punctuation::Dot),
            Token::Name("caption".to_string()),
            Token::ExpressionEnd,
            Token::Text("</a></li>\n        ".to_string()),
            Token::BlockStart, Token::Name("endfor".to_string()),
            Token::BlockEnd,
            Token::Text("        </ul>\n\n        <h1>My Webpage</h1>\n        ".to_string()),
            Token::ExpressionStart,
            Token::Name("a_variable".to_string()),
            Token::ExpressionEnd,
            Token::Text("\n    </body>\n</html>\n".to_string()),
        ]
    )
}
