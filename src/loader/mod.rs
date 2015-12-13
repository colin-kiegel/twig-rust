// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Twig template loader.

pub mod array;
pub mod filesystem;
pub mod error;
pub use self::error::LoaderError;
use api::error::Traced;

use std::fmt::Debug;
use std::borrow::Cow;

pub trait Loader : Debug {
    /// Gets the source code of a template, given its name
    ///
    /// Returns a Cow<str> to allow for efficient caching mechanisms.
    ///
    /// # Failures
    /// * When `name` is not found
    fn source<'a>(&'a mut self, name: &str) -> Result<Cow<str>, Traced<LoaderError>>;

    /// Gets the cache key to use for the cache for a given template
    ///
    /// # Failures
    /// * When `name` is not found
    fn cache_key<'a>(&'a mut self, name: &str) -> Result<Cow<'a, str>, Traced<LoaderError>>;

    /// returns true if the template is still fresh
    fn is_fresh(&mut self, name: &str, time: i64) -> bool;
}
