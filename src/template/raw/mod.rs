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

pub mod cursor;

pub use self::cursor::Cursor as Cursor;

pub struct Raw {
    filename: String,
    code: String,
}

impl Raw {
    pub fn new(code: String, filename: String) -> Raw {
        let x = Raw {
            filename: filename,
            code: code,
        };
        x.fix_linebreaks();

        return x;
    }
    
    fn fix_linebreaks(&self) {
        self.code.replace("\r\n","\n").replace("\r","\n");
    }
}
