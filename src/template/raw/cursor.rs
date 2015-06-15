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

use error;
use lexer::SyntaxError;
use lexer::SyntaxErrorCode;
use std::rc::Rc;

pub type Position = usize;
pub type Line = usize;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Cursor {
    pos: Position,   // 0,..
    end: Position,   // 0,..
    lineno: Line,    // 1,..
    template: Option<Rc<super::Raw>>, // TODO switch to pointer
}

impl Default for Cursor {
    fn default() -> Cursor {
        Cursor {
            pos: 0,
            end: 0,
            lineno: 1,
            template: None,
            /* TODO store a slice of the code directly? */
        }
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl Cursor {
    pub fn new(template: Rc<super::Raw>) -> Cursor {
        Cursor {
            end: template.code.chars().count(),
            template: Some(template),
            .. Default::default()
        }
    }
    
    pub fn move_by(&mut self, text: &str) -> Result<(), SyntaxError> { // TODO return a new slice??
        self.pos += text.chars().count();
        self.lineno += text.lines().count();
        
        if self.pos > self.end {
            self.pos = self.end;
            return err!("out of range", SyntaxErrorCode::Unknown);
        }
        
        Ok(())
    }
    
    pub fn move_to(&mut self, pos: usize) -> Result<(),SyntaxError> {
        unimplemented!(); // TODO - might want to switch to this for better integrity
    }

    pub fn is_eof(&self) -> bool {
        self.pos >= self.end
    }
    
    pub fn get_position(&self) -> Position {
        self.pos
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::Raw;
    use std::rc::Rc;

    #[test]
    pub fn new() {
        let tpl = Rc::new(Raw::new("123", ""));
        let t_o = Cursor::new(tpl);
        let t_x = Cursor {
            pos: 0,
            end: 3,
            lineno: 1,
            template: None, // not tested
        };
        
        assert_eq!(t_o.pos, t_x.pos);
        assert_eq!(t_o.end, t_x.end);
        assert_eq!(t_o.lineno, t_x.lineno);
    }
}
