// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Represents a Token
///
/// @author Colin Kiegel <kiegel@gmx.de>


/////////////
// imports //
/////////////

use std::fmt;
use error::api::Dump;

/////////////
// exports //
/////////////

pub mod stream;
pub use self::stream::Stream;

#[derive(PartialEq, Clone)]
pub enum Token {
    _Eof,
    Text(String),
    BlockStart,
    ExpressionStart, // orig. Var
    BlockEnd,
    ExpressionEnd,
    Name(String),
    IntegerNumber(u64), // orig. Number
    FloatingNumber(f64), // orig. Number
    String(String),
    Operator(String),
    Punctuation(Punctuation),
    _InterpolationStart,
    _InterpolationEnd,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Punctuation {
    Dot,
    Comma,
    Colon,
    VerticalBar,
    QuestionMark,
    OpeningBracket(BracketType),
    ClosingBracket(BracketType),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BracketType {
    Round,
    Square,
    Curly,
    DoubleQuote, // Pseudo-Bracket - never being pushed to a real token Stream
                 // but used as a temporary state of the lexer
}

#[derive(PartialEq)]
pub enum Type { // #TODO:10 - remove ?
    Eof                = -1,
    Text               = 0,
    BlockStart         = 1,
    ExpressionStart    = 2,
    BlockEnd           = 3,
    ExpressionEnd      = 4,
    Name               = 5,
    Number             = 6, // Floating or Integer
    String             = 7,
    Operator           = 8,
    Punctuation        = 9,
    InterpolationStart = 10,
    InterpolationEnd   = 11,
}

#[allow(unused_variables)]
impl Token {
    // TODO store String representation for numbers and Punctuation?
    // NOTE: Because of Number Types we need to return `String` copys instead of `&'a str`
    pub fn value<'a>(&'a self) -> Option<String> {
        match *self {
            Token::_Eof => None,
            Token::Text(ref x) => Some(x.to_string()),
            Token::BlockStart => None,
            Token::ExpressionStart => None,
            Token::BlockEnd => None,
            Token::ExpressionEnd => None,
            Token::Name(ref x) => Some(x.to_string()),
            Token::IntegerNumber(ref x) => Some(x.to_string()),
            Token::FloatingNumber(ref x) => Some(x.to_string()),
            Token::String(ref x) => Some(x.to_string()),
            Token::Operator(ref x) => Some(x.to_string()),
            Token::Punctuation(ref x) => Some(format!("{:?}",x)),
            Token::_InterpolationStart => None,
            Token::_InterpolationEnd => None,
        }
    }

    // NOTE: Does *not* yield number types - use value() instead.
    pub fn value_as_str<'a>(&'a self) -> Option<&str> {
        match *self {
            Token::_Eof => None,
            Token::Text(ref x) => Some(x),
            Token::BlockStart => None,
            Token::ExpressionStart => None,
            Token::BlockEnd => None,
            Token::ExpressionEnd => None,
            Token::Name(ref x) => Some(x),
            Token::IntegerNumber(ref x) => None, // see above
            Token::FloatingNumber(ref x) => None, // see above
            Token::String(ref x) => Some(x),
            Token::Operator(ref x) => Some(x),
            Token::Punctuation(ref x) => None, // see above
            Token::_InterpolationStart => None,
            Token::_InterpolationEnd => None,
        }
    }

    pub fn get_type(&self) -> Type {
        match *self {
            Token::_Eof => Type::Eof,
            Token::Text(_) => Type::Text,
            Token::BlockStart => Type::BlockStart,
            Token::ExpressionStart => Type::ExpressionStart,
            Token::BlockEnd => Type::BlockEnd,
            Token::ExpressionEnd => Type::ExpressionEnd,
            Token::Name(_) => Type::Name,
            Token::IntegerNumber(_) => Type::Number,
            Token::FloatingNumber(_) => Type::Number,
            Token::String(_) => Type::String,
            Token::Operator(_) => Type::Operator,
            Token::Punctuation(_) => Type::Punctuation,
            Token::_InterpolationStart => Type::InterpolationStart,
            Token::_InterpolationEnd => Type::InterpolationEnd,
        }
    }

    #[allow(dead_code)] // #TODO:690 testcase
    pub fn is_type(&self, typ: Type) -> bool {
        self.get_type() == typ
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.get_type().name())
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let typ = self.get_type().name();
        match self.value() {
            Some(ref val) => write!(f, "{typ}({val:?})", typ = typ, val = val),
            None          => write!(f, "{typ}", typ = typ),
        }
    }
}

pub type TokenDump = Token; // may change as soon as we use RefTokens

impl Dump for Token {
    type Data = TokenDump;

    fn dump(&self) -> Self::Data {
        self.clone()
    }
}

#[allow(unused_variables)]
impl Type {
    /// Returns the name of the token type (internal representation).
    pub fn name(&self) -> &'static str {
         match *self {
            Type::Eof => "EOF",
            Type::Text => "TEXT",
            Type::BlockStart => "BLOCK_START",
            Type::ExpressionStart => "EXPRESSION_START", // orig VAR_START
            Type::BlockEnd => "BLOCK_END",
            Type::ExpressionEnd => "EXPRESSION_END", // orig VAR_END
            Type::Name => "NAME",
            Type::Number => "NUMBER",
            Type::String => "STRING",
            Type::Operator => "OPERATOR",
            Type::Punctuation => "PUNCTUATION",
            Type::InterpolationStart => "INTERPOLATION_START",
            Type::InterpolationEnd => "INTERPOLATION_END",
        }
    }

    /// Returns the description of the token type in plain english.
    pub fn _description(&self) -> &'static str {
         match *self {
            Type::Eof => "end of template",
            Type::Text => "text",
            Type::BlockStart => "begin of statement block",
            Type::ExpressionStart => "begin of print expression",
            Type::BlockEnd => "end of statement block",
            Type::ExpressionEnd => "end of print expression",
            Type::Name => "name",
            Type::Number => "number",
            Type::String => "string",
            Type::Operator => "operator",
            Type::Punctuation => "punctuation",
            Type::InterpolationStart => "begin of string interpolation",
            Type::InterpolationEnd => "end of string interpolation",
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.name())
    }
}

pub trait Pattern: fmt::Debug + fmt::Display {
    fn matches(&self, &Token) -> bool;
}

pub type PatternDump = String;

impl Dump for Pattern {
    type Data = PatternDump;

    fn dump(&self) -> Self::Data {
        format!("{:?}", self)
    }
}

impl Pattern for Token {
    fn matches(&self, token: &Token) -> bool {
        *self == *token
    }
}

impl Pattern for Type {
    fn matches(&self, token: &Token) -> bool {
        *self == token.get_type()
    }
}

#[cfg(test)]
mod test {
    use super::{Token, Type};

    #[test]
    fn new_token() {
        let token = Token::Text("Hello World!".to_string());
        assert_eq!(token.value().unwrap(), "Hello World!".to_string());
        assert!(token.is_type(Type::Text));
    }
}
