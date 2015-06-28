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


static TWIG_TEMPLATE_NAME : &'static str = "twig.template.html";
static TWIG_TEMPLATE_CODE : &'static str = include_str!("twig.template.html");

#[test]
pub fn new() {
    let tpl = Rc::new(Raw::new(TWIG_TEMPLATE_CODE, TWIG_TEMPLATE_NAME));
    let lxr = Lexer::default();

    let tokenstream = lxr.tokenize(&tpl).unwrap();

    assert!(!tokenstream.is_eof());
}
