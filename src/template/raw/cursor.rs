/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Position within a raw template.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use lexer::error::LexerError;
use lexer::SyntaxErrorCode;

/////////////
// exports //
/////////////

pub type Position = usize;
pub type Line = usize;

#[derive(Debug)]
pub struct Cursor<'a> {
    pos: Position,   // 0,..
    end: Position,   // 0,..
    lineno: Line,    // 1,..
    template: &'a super::Raw, // TODO switch to pointer or slice
}

#[allow(dead_code)]
impl<'a> Cursor<'a> {
    pub fn new(template: &'a super::Raw) -> Cursor<'a> {
        Cursor {
            end: template.code.chars().count(),
            template: template,
            pos: 0,
            lineno: 1,
        }
    }

    // pub fn move_by_text(&mut self, text: &str) -> Result<(), SyntaxError> {
    //     self.pos += text.chars().count();
    //     self.lineno += text.lines().count();
    //
    //     if self.pos > self.end {
    //         return err!(SyntaxErrorCode::Unknown, "out of range");
    //     }
    //
    //     Ok(())
    // }

    /// move the cursor `position` by `increment` and keep track of the `lineno`
    /// # panics
    /// when the `increment` would move the cursor `position` out of range
    pub fn move_by(&mut self, increment: usize) {
        let pos = self.pos + increment;
        if pos > self.end {
            panic!("cursor is out of range");
        }
        self.lineno += self.template.code[self.pos..pos].lines().count();
        self.pos = pos;
    }

    /// moves the cursor to a new position
    ///
    /// # Panics
    /// Panics if `pos` is greater than the template's length.
    pub fn move_to(&mut self, pos: usize) {
        if pos > self.pos {
            // if pos > self.end {
            //     panic!("Cursor::move_to() is out of range")
            // }
            self.lineno += self.template.code[self.pos..pos].lines().count();
        } else {
            self.lineno -= self.template.code[pos..self.pos].lines().count();
        }
        self.pos = pos;
    }

    pub fn move_to_end(&mut self) {
        self.lineno += self.template.code[self.pos..self.end].lines().count();
        self.pos = self.end;
    }

    #[inline]
    pub fn slice_by(&mut self, len: usize) -> &'a str  {
        let pos = self.pos + len;
        self.slice_to(pos)
    }

    #[inline]
    pub fn slice_to_end(&mut self) -> &'a str {
        let pos = self.end;
        self.slice_to(pos)
    }

    pub fn slice_to(&mut self, pos: usize) -> &'a str  {
        let ref slice = &self.template.code[self.pos..pos];
        self.lineno += slice.lines().count();
        self.pos = self.end;

        slice
    }

    pub fn tail(&mut self) -> &'a str {
        &self.template.code[self.pos..]
    }

    pub fn is_eof(&self) -> bool {
        self.pos >= self.end
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn line(&self) -> usize {
        self.lineno
    }

    pub fn set_line(&mut self, line: usize) {
        self.lineno = line;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::Raw;

    #[test]
    pub fn new() {
        let tpl = Raw::new("123", "");
        let c_o = Cursor::new(&tpl);
        let c_x = Cursor {
            pos: 0,
            end: 3,
            lineno: 1,
            template: &tpl,
        };

        assert_eq!(c_o.pos, c_x.pos);
        assert_eq!(c_o.end, c_x.end);
        assert_eq!(c_o.lineno, c_x.lineno);
    }
}
