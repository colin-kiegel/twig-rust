// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Extension `operator` definition

pub type UnaryOperator = Unary;
pub type BinaryOperator = Binary;

#[derive(Debug, PartialEq)]
pub struct Extension(String); // might switch to ID for faster lookups

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Precedence(pub usize);

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
pub struct Unary { // #TODO:430 switch from struct to trait?
    pub repr: String, // token representation like "-"
    pub ext: Extension,
    pub prec: Precedence,
    pub op: Operation,
}

/// Binary operator
#[derive(Debug, PartialEq)]
pub struct Binary { // #TODO:440 switch from struct to trait?
    pub repr: String, // token representation like "!="
    pub ext: Extension,
    pub prec: Precedence,
    pub op: Operation,
    pub assoc: Assoc,
}
