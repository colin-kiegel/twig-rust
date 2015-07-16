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

use std::fmt;
//use lexer::error::LexerError;
//use lexer::SyntaxErrorCode;

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

impl<'a> Cursor<'a> {
    #[allow(dead_code)] // only used in test and elsewhere
    pub fn new(template: &'a super::Raw) -> Cursor<'a> {
        Cursor {
            end: template.code.chars().count(),
            template: template,
            pos: 0,
            lineno: 1,
        }
    }

    /// move the cursor `position` by `increment` and keep track of the `lineno`
    ///
    /// `increment` in bytes (not chars)
    ///
    /// # panics
    /// when the `increment` would move the cursor `position` out of range
    pub fn move_by(&mut self, increment: usize) {
        if increment == 0 {
            println!("cursor.move_by 0"); // TODO DEBUG + REMOVE
            return;
        };

        let pos = self.pos + increment;
        if pos > self.end {
            panic!("cursor is out of range");
        }
        self.lineno += self.template.code[self.pos..pos].chars().filter(|c| *c == '\n').count();
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
            self.lineno += self.template.code[self.pos..pos].chars().filter(|c| *c == '\n').count();
        } else if pos < self.pos {
            self.lineno -= self.template.code[pos..self.pos].chars().filter(|c| *c == '\n').count();
        }
        self.pos = pos;
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
        self.lineno += slice.chars().filter(|c| *c == '\n').count();
        self.pos = pos;

        slice
    }

    pub fn _template(&self) -> &super::Raw {
        &self.template
    }

    pub fn tail(&self) -> &'a str {
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
        // Alternative: self.template.code[0..self.pos].chars().filter(|c| c == '\n').count() + 1;
        // - might be better if called seldomly!
    }

    pub fn column(&self) -> usize { // start counting with `1`
        self.template.code[0..self.pos].chars().rev().take_while(|c| *c != '\n').count() + 1
    }

    pub fn set_line(&mut self, line: usize) {
        self.lineno = line;
    }
}

impl<'a> fmt::Display for Cursor<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "`{filename}` line {line} column {column}",
            filename = self.template.filename(),
            line = self.line(),
            column = self.column())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::Raw;

    #[test]
    pub fn new() {
        let tpl = Raw::new("123", "");
        let c = Cursor::new(&tpl);

        assert_eq!(c.position(), 0);
        assert_eq!(c.line(), 1);
        assert_eq!(c.template.code, "123");
    }

    #[test]
    pub fn move_by() {
        let tpl = Raw::new("\nline2\n\nline4\n\n\nline7", "");
        let mut c = Cursor::new(&tpl);

        let x = "\nline2\n\nline4".len();
        c.move_by(x);
        assert_eq!(c.position(), x);
        assert_eq!(c.line(), 4);
    }

    #[test]
    pub fn move_to() {
        let tpl = Raw::new("\nline2\n\nline4\n\n\nline7", "");
        let mut c = Cursor::new(&tpl);

        let x = "\nline2".len();
        c.move_to(x);
        assert_eq!(c.position(), x);
        assert_eq!(c.line(), 2);
    }

    #[test]
    pub fn slice_by() {
        let tpl = Raw::new("\nline2\n\nline4\n\n\nline7", "");
        let mut c = Cursor::new(&tpl);

        let x = "\nline2\n\nline4".len();
        c.slice_by(x);
        assert_eq!(c.position(), x);
        assert_eq!(c.line(), 4);
    }

    #[test]
    pub fn slice_to() {
        let tpl = Raw::new("\nline2\n\nline4\n\n\nline7", "");
        let mut c = Cursor::new(&tpl);

        let x = "\nline2".len();
        c.slice_to(x);
        assert_eq!(c.position(), x);
        assert_eq!(c.line(), 2);
    }

    #[test]
    pub fn slice_to_end() {
        let tpl = Raw::new("\nline2\n\nline4\n\n\nline7", "");
        let mut c = Cursor::new(&tpl);

        let x = "\nline2\n\nline4\n\n\nline7".len();
        c.slice_to_end();
        assert_eq!(c.position(), x);
        assert_eq!(c.line(), 7);
    }

    #[test]
    pub fn column() { // start counting with `1`
        let tpl = Raw::new("\nline2\n\nline4\n\n\nline7", "");
        let mut c = Cursor::new(&tpl);

        assert_eq!(c.column(), 1);
        c.move_to("\nline2".len());
        assert_eq!(c.column(), 6);
        c.move_to("\nline2\n\nline4\n\n\nl".len());
        assert_eq!(c.column(), 2);
    }
}
