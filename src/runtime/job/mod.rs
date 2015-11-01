// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Runtime job
///
/// @author Colin Kiegel <kiegel@gmx.de>

/////////////
// imports //
/////////////

use runtime::Runtime;
use runtime::api::Execute;

/////////////
// exports //
/////////////

#[allow(dead_code)]
#[derive(Debug)]
pub struct Job {
    writer: String, // #TODO:560 switch to a 'more generic' string writer
}

#[allow(dead_code)]
impl Job {
    pub fn new() -> Job {
        Job {
            writer: String::new(),
        }
    }

    pub fn reserve_writer(&mut self, additional: usize) -> &mut Job {
        self.writer.reserve(additional);

        self
    }

    pub fn run(mut self, runtime: &Runtime, node: &Execute) -> String {
        node.execute(runtime, &mut self);

        self.writer
    }

    pub fn write<T>(&mut self, text: T) where
        T: AsRef<str>
    {
        self.writer.push_str(text.as_ref())
    }

    pub fn result(&self) -> &str {
        &self.writer
    }
}
