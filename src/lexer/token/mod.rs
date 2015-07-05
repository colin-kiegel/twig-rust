/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Represents a Token
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// exports //
/////////////

pub mod stream;
pub use self::stream::Stream;


#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Token {
    Eof,
    Text(String),
    BlockStart,
    VarStart,
    BlockEnd,
    VarEnd,
    Name(String),
    IntegerNumber(u64), // orig. Number
    FloatingNumber(f64), // orig. Number
    String(String),
    Operator(String),
    Punctuation(String),
    InterpolationStart,
    InterpolationEnd,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Type {
    Eof                = -1,
    Text               = 0,
    BlockStart         = 1,
    VarStart           = 2,
    BlockEnd           = 3,
    VarEnd             = 4,
    Name               = 5,
    Number             = 6, // Floating or Integer
    String             = 7,
    Operator           = 8,
    Punctuation        = 9,
    InterpolationStart = 10,
    InterpolationEnd   = 11,
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl Token {
    // Because of Number Types we need to return `String` copys instead of `&'a str`
    pub fn get_value<'a>(&'a self) -> Option<String> {
        match *self {
            Token::Eof => None,
            Token::Text(ref x) => Some(x.to_string()),
            Token::BlockStart => None,
            Token::VarStart => None,
            Token::BlockEnd => None,
            Token::VarEnd => None,
            Token::Name(ref x) => Some(x.to_string()),
            Token::IntegerNumber(ref x) => Some(x.to_string()),
            Token::FloatingNumber(ref x) => Some(x.to_string()),
            Token::String(ref x) => Some(x.to_string()),
            Token::Operator(ref x) => Some(x.to_string()),
            Token::Punctuation(ref x) => Some(x.to_string()),
            Token::InterpolationStart => None,
            Token::InterpolationEnd => None,
        }
    }

    pub fn get_type(&self) -> Type {
        match *self {
            Token::Eof => Type::Eof,
            Token::Text(_) => Type::Text,
            Token::BlockStart => Type::BlockStart,
            Token::VarStart => Type::VarStart,
            Token::BlockEnd => Type::BlockEnd,
            Token::VarEnd => Type::VarEnd,
            Token::Name(_) => Type::Name,
            Token::IntegerNumber(_) => Type::Number,
            Token::FloatingNumber(_) => Type::Number,
            Token::String(_) => Type::String,
            Token::Operator(_) => Type::Operator,
            Token::Punctuation(_) => Type::Punctuation,
            Token::InterpolationStart => Type::InterpolationStart,
            Token::InterpolationEnd => Type::InterpolationEnd,
        }
    }

    pub fn is_type(&self, typ: Type) -> bool {
        self.get_type() == typ
    }
}

impl ToString for Token {
    /// Returns a string representation of the token type.
    fn to_string(&self) -> String {
        let typ = self.get_type().get_name(false);
        match self.get_value() {
            Some(ref val) => format!("{typ}({val})", typ = typ, val = val),
            None          => format!("{typ}", typ = typ),
        }
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl Type {
    /// Returns the name of the token type (internal representation).
    ///
    /// # Arguments
    ///
    /// * `short` - short or long representation

    pub fn get_name(&self, short: bool) -> String {
         let name = match *self {
            Type::Eof => "EOF",
            Type::Text => "TEXT",
            Type::BlockStart => "BLOCK_START",
            Type::VarStart => "VAR_START",
            Type::BlockEnd => "BLOCK_END",
            Type::VarEnd => "VAR_END",
            Type::Name => "NAME",
            Type::Number => "NUMBER",
            Type::String => "STRING",
            Type::Operator => "OPERATOR",
            Type::Punctuation => "PUNCTUATION",
            Type::InterpolationStart => "INTERPOLATION_START",
            Type::InterpolationEnd => "INTERPOLATION_END",
        };

        if short {
            name.to_string()
        } else {
            format!("Token::Type::{}", name)
        }
    }

    /// Returns the description of the token type in plain english.

    pub fn get_description(&self) -> String {
         match *self {
            Type::Eof => "end of template",
            Type::Text => "text",
            Type::BlockStart => "begin of statement block",
            Type::VarStart => "begin of print statement",
            Type::BlockEnd => "end of statement block",
            Type::VarEnd => "end of print statement",
            Type::Name => "name",
            Type::Number => "number",
            Type::String => "string",
            Type::Operator => "operator",
            Type::Punctuation => "punctuation",
            Type::InterpolationStart => "begin of string interpolation",
            Type::InterpolationEnd => "end of string interpolation",
        }.to_string()
    }
}

impl ToString for Type {
    fn to_string(&self) -> String {
         self.get_name(true)
    }
}

#[cfg(test)]
mod test {
    use super::{Token, Type};

    #[test]
    fn new_token() {
        let token = Token::Text("Hello World!".to_string());
        assert_eq!(token.get_value().unwrap(), "Hello World!".to_string());
        assert!(token.is_type(Type::Text));
    }
}
