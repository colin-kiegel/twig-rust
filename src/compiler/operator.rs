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

/////////////
// exports //
/////////////


#[derive(Debug, PartialEq)]
pub struct Extension(String); // might switch to ID for faster lookups

#[derive(Debug, PartialEq)]
pub struct Precedence(usize);

#[derive(Debug, PartialEq)]
pub enum Operation {
    Class(Class),
    Callable(Function)
}

/// Associativity
#[derive(Debug, PartialEq)]
pub enum Assoc {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub struct Function {
    name: String
}

#[derive(Debug, PartialEq)]
pub struct Class {
    name: String
}

/// Unary operator
#[derive(Debug, PartialEq)]
pub struct Unary {
    pub repr: String, // token representation like "-"
    pub ext: Extension,
    pub prec: Precedence,
    pub op: Operation,
}

/// Binary operator
#[derive(Debug, PartialEq)]
pub struct Binary {
    pub repr: String, // token representation like "!="
    pub ext: Extension,
    pub prec: Precedence,
    pub op: Operation,
    pub assoc: Assoc,
}
