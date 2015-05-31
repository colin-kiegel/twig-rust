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
use error::aliases::SyntaxErrorCode;

pub type Position = usize;
pub type Line = usize;

pub struct Cursor {
    pos: Position,   // 0,..
    end: Position,   // 0,..
    lineno: Line,    // 1,..
    template: Option<super::Raw>, // TODO switch to pointer
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

impl Cursor {
    pub fn new(template: super::Raw) -> Cursor {
        Cursor {
            end: template.code.chars().count(),
            template: Some(template),
            .. Default::default()
        }
    }

    pub fn move_by(&mut self, text: &str) -> Result<(), error::aliases::SyntaxError/*, error::aliases::SyntaxError*/> { // return a new slice??
        self.pos += text.chars().count();
        self.lineno += text.lines().count();
        
        if self.pos > self.end {
            self.pos = self.end;
            //error::syntax::Code::Unknown
            let x: error::syntax::Code = error::syntax::Code::Unknown;
            return err!("out of range", x)
        }
        
        Ok(())
    }
    
    pub fn move_to(&mut self, pos: usize) -> Result<(),()/*, Err*/> {
        unimplemented!(); // TODO - might want to switch to this for better integrity
    }
    
    pub fn is_eof(&self) -> bool {
        self.pos >= self.end
    }
    
    pub fn get_position(&self) -> Position {
        self.pos
    }
}
