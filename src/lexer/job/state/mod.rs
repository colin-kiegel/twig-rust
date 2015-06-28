/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * FSM model of the lexer.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use lexer::SyntaxError;
use lexer::job::Job;
use std::fmt::Debug;

/////////////
// exports //
/////////////

pub mod initial;
pub mod data;
pub mod _final;
pub use self::initial::Initial;


pub trait Tokenize : Debug {
    fn new() -> Box<Self>
        where Self : Sized;

    fn is_finished(&self) -> bool{
        false
    }

    fn step<'a>(&self, &mut Job<'a>) -> Result<Box<Tokenize>,SyntaxError>;

    fn get_type(&self) -> Code;

    fn is_type(&self, code: Code) -> bool {
        self.get_type() == code
    }
}

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Code {
    Data            = 0,
    Block           = 1,
    Var             = 2,
    String          = 3,
    Interpolation   = 4,
    Initial         = -1, // orig: implicit sub-state of Data
    Final           = -2, // orig: implicit sub-state
}

#[allow(dead_code)]
impl Default for Code {
    fn default() -> Code {
        Code::Initial
    }
}
