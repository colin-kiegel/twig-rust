// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Runtime job.

use runtime::Runtime;
use runtime::api::Execute;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Job {
    writer: String, // TODO: switch to a 'more generic' string writer
}

#[allow(dead_code)]
impl Job {
    pub fn new() -> Job {
        Job { writer: String::new() }
    }

    pub fn reserve_writer(&mut self, additional: usize) -> &mut Job {
        self.writer.reserve(additional);

        self
    }

    pub fn run(mut self, runtime: &Runtime, template: &Execute) -> String {
        template.execute(runtime, &mut self);

        self.writer
    }

    pub fn write<T>(&mut self, text: T)
        where T: AsRef<str>
    {
        self.writer.push_str(text.as_ref())
    }

    pub fn result(&self) -> &str {
        &self.writer
    }
}
