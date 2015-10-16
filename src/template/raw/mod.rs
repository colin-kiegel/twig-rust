/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * RegExes of the lexer.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use super::api::Template;
use compiler::{Compiler, TwigError};
use lexer::{Lexer, token, LexerError};

/////////////
// exports //
/////////////

pub mod cursor;
pub use self::cursor::Cursor as Cursor;


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

    pub fn tokenize<'a, 't> (&'t self, lexer: &'a Lexer) -> Result<token::Stream<'t>, LexerError>
        where 't: 'a // the template must outlive the Lexer
    {
        lexer.tokenize(self)
    }
}

// NOTE: probably no need to implement this for template::Raw?
impl Template for Raw {
    fn render(&self, compiler: &mut Compiler, _context: Vec<()>) -> Result<String, TwigError> {
        let _tokenstream = try!(self.tokenize(try!(compiler.lexer())));

        unimplemented!()
    }

    fn display(&self, _compiler: &mut Compiler, _context: Vec<()>, _blocks: Option<Vec<()>>) {
        unimplemented!()
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
