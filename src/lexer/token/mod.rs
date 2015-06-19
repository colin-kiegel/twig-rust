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

// exports //
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
    Number(String),
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
    Number             = 6,
    String             = 7,
    Operator           = 8,
    Punctuation        = 9,
    InterpolationStart = 10,
    InterpolationEnd   = 11,
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl Token {
    pub fn get_value<'a>(&'a self) -> Option<&'a str> {
        match *self {
            Token::Eof => None,
            Token::Text(ref x) => Some(x),
            Token::BlockStart => None,
            Token::VarStart => None,
            Token::BlockEnd => None,
            Token::VarEnd => None,
            Token::Name(ref x) => Some(x),
            Token::Number(ref x) => Some(x),
            Token::String(ref x) => Some(x),
            Token::Operator(ref x) => Some(x),
            Token::Punctuation(ref x) => Some(x),
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
            Token::Number(_) => Type::Number,
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
            Type::Eof => "EOF_TYPE",
            Type::Text => "TEXT_TYPE",
            Type::BlockStart => "BLOCK_START_TYPE",
            Type::VarStart => "VAR_START_TYPE",
            Type::BlockEnd => "BLOCK_END_TYPE",
            Type::VarEnd => "VAR_END_TYPE",
            Type::Name => "NAME_TYPE",
            Type::Number => "NUMBER_TYPE",
            Type::String => "STRING_TYPE",
            Type::Operator => "OPERATOR_TYPE",
            Type::Punctuation => "PUNCTUATION_TYPE",
            Type::InterpolationStart => "INTERPOLATION_START_TYPE",
            Type::InterpolationEnd => "INTERPOLATION_END_TYPE",
        };
        
        if short {
            name.to_string()
        } else {
            format!("Token::Type::{}", name)
        }
    }

    // Returns the description of the token type in plain english.

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
