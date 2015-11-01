/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Position within a token stream.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use lexer::token::stream::{self, Stream, Item};
use lexer::Token;
use parser::{ParserError, ParserErrorCode};

/////////////
// exports //
/////////////

pub type Position = usize;

#[derive(Debug)]
pub struct Cursor<'stream> {
    pos: Position,   // 0,..
    end: Position,   // 0,..
    stream: &'stream Stream<'stream>, // inner lifetime: 'template
}

impl<'stream> Cursor<'stream> {
    #[allow(dead_code)] // only used in test and elsewhere
    pub fn new(stream: &'stream Stream) -> Cursor<'stream> {
        Cursor {
            end: stream.len(),
            stream: stream, // read-only, so `end` will always be valid
            pos: 0,
        }
    }

    pub fn _stream(&self) -> &Stream {
        self.stream
    }

    /// move the cursor `position` by `increment`
    ///
    /// # panics
    /// when the `increment` would move the cursor `position` out of range
    pub fn move_by(&mut self, increment: usize) {
        self.pos = self.pos + increment;
        assert!(self.pos <= self.end, "cursor is out of range");
    }

    pub fn next(&mut self) -> Option<&'stream stream::Item> {
        let next = self.peek();
        if next.is_some() {
            self.pos += 1;
        }

        next
    }

    pub fn next_token(&mut self) -> Option<&'stream Token> {
        self.next().map(|item| item.token())
    }

    pub fn next_pos(&mut self) -> Option<&'stream stream::Position> {
        self.next().map(|item| item.position())
    }

    pub fn next_expect(&mut self, token: Token) -> Result<(), ParserError> {
        match self.next() {
            Some(item) => Ok(try!(item.expect(token))),
            None => err!(ParserErrorCode::Eof,
                "Expected token {t:?} but found end of stream",
                t = token)
                .into()
        }
    }

    pub fn peek(&self) -> Option<&'stream Item> {
        self.stream.as_vec().get(self.pos)
    }

    pub fn peek_token(&self) -> Option<&'stream Token> {
        self.peek().map(|item| item.token())
    }

    pub fn peek_pos(&self) -> Option<&'stream stream::Position> {
        self.peek().map(|item| item.position())
    }

    pub fn peek_expect(&self, token: Token) -> Result<(), ParserError> {
        match self.peek() {
            Some(item) => Ok(try!(item.expect(token))),
            None => err!(ParserErrorCode::Eof,
                "Expected token {t:?} but found end of stream",
                t = token)
                .into()
        }
    }
}
