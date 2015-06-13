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
 
use lexer::Lexer;
use lexer::Options;
use environment::Environment;
use template::raw::Raw;
use std::rc::Rc;

const TWIG_TEMPLATE_CODE : &'static str = include_str!("twig.template.html");
const TWIG_TEMPLATE_NAME : &'static str = "twig.template.html";

#[test]
pub fn new() {
    let env = Environment{..Default::default()};
    let opt = Options{..Default::default()};
    let tpl = Rc::new(Raw::new(TWIG_TEMPLATE_CODE, TWIG_TEMPLATE_NAME));
    let mut lxr = Lexer::new(env, opt);
    
    let tokenstream = lxr.tokenize(tpl).unwrap();
    
    assert!(!tokenstream.is_eof());
}
