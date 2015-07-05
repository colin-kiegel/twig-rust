/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Lexes a template string.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use lexer::Lexer;
use template::raw::Raw;
use std::rc::Rc;
use lexer::token::Token;


static TWIG_TEMPLATE_NAME : &'static str = "twig.template.html";
static TWIG_TEMPLATE_CODE : &'static str = include_str!("twig.template.html");

// TODO read tokens from file too
/// #panics
/// when the tokenstream does not equal `tokens`
pub fn assert_tokenize(code: &str, filename: &str, tokens: Vec<Token>) {
    let tpl = Rc::new(Raw::new(code, filename));
    let lxr = Lexer::default();

    println!("{:?}", tpl);
    let tokenstream = lxr.tokenize(&tpl).unwrap();
    // println!("{}", tokenstream.to_string());

    assert_eq!(
        tokenstream.into_iter().map(|i| i.into()).collect::<Vec<Token>>(),
        tokens
    );
}

#[test]
pub fn tokenize() {
    assert_tokenize(
        TWIG_TEMPLATE_CODE,
        TWIG_TEMPLATE_NAME,
        vec![
            Token::Text("".to_string())
        ]
    )
}
