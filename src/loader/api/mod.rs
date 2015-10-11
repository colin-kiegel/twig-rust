/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Twig library for rust
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use std::fmt::Debug;
use std::borrow::Cow;
use compiler::TwigError;

/////////////
// exports //
/////////////

pub trait Loader : Debug {
    /// Gets the source code of a template, given its name
    ///
    /// # Failures
    ///
    /// * When `name` is not found
    fn source<'a>(&'a mut self, name: &str) -> Result<Cow<'a, str>, TwigError>;  // #TODO:240 more meaningful errors

    /// Gets the cache key to use for the cache for a given template
    ///
    /// # Failures
    ///
    /// * When `name` is not found
    fn cache_key<'a>(&'a mut self, name: &str) -> Result<Cow<'a, str>, TwigError>;  // #TODO:250 more meaningful errors

    /// returns true if the template is still fresh
    fn is_fresh(&mut self, name: &str, time: i64) -> bool;
}
