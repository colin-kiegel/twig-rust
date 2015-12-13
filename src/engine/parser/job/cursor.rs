// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Position within a token stream.

use engine::parser::token::stream::{self, Stream, Item};
use engine::parser::token::{self, Token};
use engine::parser::ParserError;
use std::fmt;
use api::error::{Traced, Dump};

pub type Position = usize;

#[derive(Debug)]
pub struct Cursor<'stream> {
    next: Position,   // 0,..
    end: Position,   // 0,..
    stream: &'stream Stream<'stream>, // inner lifetime: 'template
}

impl<'stream> Cursor<'stream> {
    #[allow(dead_code)] // only used in test and elsewhere
    pub fn new(stream: &'stream Stream) -> Cursor<'stream> {
        Cursor {
            end: stream.len(),
            stream: stream, // read-only, so `end` will always be valid
            next: 0,
        }
    }

    pub fn stream(&self) -> &Stream {
        self.stream
    }

    /// move the cursor `position` by `increment`
    ///
    /// # panics
    /// when the `increment` would move the cursor `position` out of range
    pub fn move_by(&mut self, increment: usize) {
        self.next = self.next + increment;
        assert!(self.next <= self.end, "cursor is out of range");
    }

    pub fn next(&mut self) -> Option<&'stream stream::Item> {
        let next = self.peek();
        if next.is_some() {
            self.next += 1;
        }

        next
    }

    pub fn next_token(&mut self) -> Option<&'stream Token> {
        self.next().map(|item| item.token())
    }

    pub fn next_pos(&mut self) -> Option<&'stream stream::Position> {
        self.next().map(|item| item.position())
    }

    pub fn next_expect<T>(&mut self, pattern: T, reason: Option<&'static str>) -> Result<&'stream Item, Traced<ParserError>>
        where T: token::Pattern + 'static
    {
        let next = self.peek();
        if next.is_some() {
            self.next += 1;
        }
        self.expect(pattern, next, reason)
    }

    pub fn peek(&self) -> Option<&'stream Item> {
        self.stream.as_vec().get(self.next)
    }

    pub fn peek_token(&self) -> Option<&'stream Token> {
        self.peek().map(|item| item.token())
    }

    pub fn peek_pos(&self) -> Option<&'stream stream::Position> {
        self.peek().map(|item| item.position())
    }

    pub fn peek_expect<T>(&self, pattern: T, reason: Option<&'static str>) -> Result<&'stream Item, Traced<ParserError>>         where T: token::Pattern + 'static
    {
        self.expect(pattern, self.peek(), reason)
    }

    fn expect<T>(&self, pattern: T, value: Option<&'stream Item>, reason: Option<&'static str>) -> Result<&'stream Item, Traced<ParserError>>
        where T: token::Pattern + 'static
    {
        match value {
            Some(item) => Ok(try_traced!(item.expect(pattern, reason))),
            None => traced_err!(ParserError::UnexpectedEof {
                        expected: Some(<token::Pattern as Dump>::dump(&pattern)),
                        cursor: self.dump(),
                        reason: reason
                    })
        }
    }
}

impl<'stream> fmt::Display for Cursor<'stream> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "cursor (next: {next}/{end}) {tokens:?}",
            next = self.next,
            end = self.end,
            tokens = self.stream)
    }
}

#[derive(Debug)]
pub struct CursorDump {
    next: Position,
    end: Position,
    stream_dump: stream::StreamDump,
}

impl<'stream> Dump for Cursor<'stream> {
    type Data = CursorDump;

    fn dump(&self) -> Self::Data {
        CursorDump {
            next: self.next,
            end: self.end,
            stream_dump: self.stream.dump(),
        }
    }
}

impl fmt::Display for CursorDump {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cursor (next: {next}/{end}) for {stream_dump}",
            next = self.next,
            end = self.end,
            stream_dump = self.stream_dump)
    }
}
