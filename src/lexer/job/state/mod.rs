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

// ## exports ##
pub mod initial;
pub mod _final;
//pub mod block;
//pub mod data;
//pub mod interpolation;
//pub mod string;
//pub mod var;
pub use self::initial::Initial;

// ## imports ##
use lexer::SyntaxError;
use lexer::job::Job;
use std::fmt::Debug;
//use self::_final::Final;
//use block;
//use data;
//use interpolation;
//use string;
//use var;

pub trait Tokenize : Debug {
    fn new() -> Box<Self>
        where Self : Sized;
        
    fn is_finished(&self) -> bool;
    
    fn step(&self, &mut Job) -> Result<Box<Tokenize>,SyntaxError>;
}

#[allow(dead_code)]
pub enum Code {
    Data            = 0,
    Block           = 1,
    Var             = 2,
    String          = 3,
    Interpolation   = 4,
    Initial         = -1, // orig: implicit sub-state of Data
    End             = -2, // orig: implicit sub-state
}

#[allow(dead_code)]
impl Default for Code {
    fn default() -> Code {
        Code::Initial
    }
}
