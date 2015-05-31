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

pub mod stream;

pub use self::stream::Stream;
pub use template::raw::cursor::Position;
use lexer;

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

#[derive(Debug, PartialEq)]
pub struct Value(pub String); // TODO evaluate switch to slice

#[derive(Debug)]
pub struct Token {
    typ: Type, // orig called 'type' 
    val: Value,
    pos: Position, // orig called 'lineno'
}

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

impl Token {
    /// Constructor.
    /// 
    /// # Arguments
    ///
    /// * `typ` - The type of the token
    /// * `val` - The token value
    /// * `pos` - The line position in the source
    
    pub fn new(typ: Type, val: Value, pos: Position) -> Token {
        Token {
            typ: typ,
            val: val,
            pos: pos,
        }
    }
    
    pub fn is_type(&self, typ: Type) -> bool {
        self.typ == typ
    }
    
    pub fn is_value(&self, val: Value) -> bool {
        self.val == val
    }
}

impl ToString for Token {
    /// Returns a string representation of the token type.
    fn to_string(&self) -> String {
        let Value(ref val_string) = self.val;
        format!("{}({})", self.typ.to_string(), val_string)
        //return sprintf('%s(%s)', self::typeToString($this->type, true), $this->value);
    }
}

#[cfg(test)]
mod test {
    use super::{Token, Type, Value, Position};

    #[test]
    fn new_token() {
        let token = Token::new(
            Type::Text,
            Value("Hello World!".to_string()),
            0,
        );
        assert_eq!(token.val, Value("Hello World!".to_string()));
        assert!(token.is_type(Type::Text));
    }
}
