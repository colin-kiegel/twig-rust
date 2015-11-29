// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// RegExes of the lexer.

use lexer::{Lexer, token, LexerError};
use std::fmt;

#[derive(Default)]
#[derive(Debug)]
pub struct Raw {
    name: String, // twig template name, e.g. "@namespace/path/to/template"
    pub code: String,
}

#[allow(unused_variables)]
impl Raw {
    #[allow(dead_code)] // #TODO:730 testcase
    pub fn new<C,N>(code: C, name: N) -> Raw where
        C: ToString,
        N: ToString,
        // NOTE: Into<String> would be more efficient
        //      but Cow<'_, str> does not implement Into<String>
        //      -> suggest this as new std lib feature?
    {
        let mut x = Raw {
            name: name.to_string(),
            code: code.to_string(),
        };
        x.fix_linebreaks();

        return x;
    }

    fn fix_linebreaks(&mut self) {
        self.code = self.code.replace("\r\n","\n").replace("\r","\n");
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    #[allow(dead_code)]
    pub fn tokenize<'a, 't> (&'t self, lexer: &'a Lexer) -> Result<token::Stream<'t>, LexerError>
        where 't: 'a // the template must outlive the Lexer
    {
        lexer.tokenize(self)
    }
}

impl fmt::Display for Raw {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "template ({name:?}): {code:?}",
            name = self.name(),
            code = self.code)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn new() {
        let t = Raw::new("A", "B");

        assert_eq!(t.code, "A");
        assert_eq!(t.name, "B");
    }

    #[test]
    pub fn fix_linebreaks() {
        let mut t = Raw {
            code: "1\r\n2\n3\r".to_string(),
            ..Default::default()
        };
        t.fix_linebreaks();
        assert_eq!(t.code, "1\n2\n3\n");
    }
}
