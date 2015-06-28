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
// exports //
/////////////

pub mod cursor;
pub use self::cursor::Cursor as Cursor;


#[allow(dead_code)]
#[allow(unused_variables)]
#[derive(Default)]
#[derive(Debug)]
pub struct Raw {
    filename: String,
    pub code: String,
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl Raw {
    pub fn new(code: &str, filename: &str) -> Raw {
        let mut x = Raw {
            filename: filename.to_string(),
            code: code.to_string(),
        };
        x.fix_linebreaks();

        return x;
    }

    fn fix_linebreaks(&mut self) {
        self.code = self.code.replace("\r\n","\n").replace("\r","\n");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn new() {
        let t = Raw::new("A", "B");

        assert_eq!(t.code, "A");
        assert_eq!(t.filename, "B");
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
