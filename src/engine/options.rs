// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Twig configuration options.

use std::path::{Path, PathBuf};
use engine::extension::escaper;
use engine::extension::optimizer;

pub type Autoescape = escaper::Mode;
pub type Optimizations = optimizer::Mode;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Charset {
    UTF8
}

impl Default for Charset {
    fn default() -> Charset {
        Charset::UTF8
    }
}

#[derive(Debug, PartialEq)]
pub struct Options {
    pub debug: bool,
    pub charset: Charset,
    pub strict_variables: bool,
    pub autoescape: Autoescape,
    pub cache: Option<PathBuf>,
    pub auto_reload: Option<bool>, // defaults to `self.debug` if set to `None`
    pub optimizations: Optimizations,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            debug: false,
            charset: Charset::default(),
            strict_variables: false,
            autoescape: escaper::Mode::default(),
            cache: None,
            auto_reload: None,
            optimizations: optimizer::Mode::default(),
        }
    }
}

impl Options {
    pub fn debug(&self) -> bool {
        self.debug
    }

    pub fn charset(&self) -> Charset {
        self.charset
    }

    pub fn strict_variables(&self) -> bool {
        self.strict_variables
    }

    pub fn autoescape(&self) -> Autoescape {
        self.autoescape
    }

    pub fn cache(&self) -> Option<&Path> {
        // #TODO:770 why doesn't this work? -> self.cache.map(|ref buf| buf.as_ref())
        match self.cache {
            Some(ref buf) => Some(buf.as_ref()),
            None => None
        }
    }

    /// if unset it defaults to `self.debug()`
    pub fn auto_reload(&self) -> bool {
        self.auto_reload.unwrap_or(self.debug)
    }

    pub fn optimizations(&self) -> Optimizations {
        self.optimizations
    }
}
