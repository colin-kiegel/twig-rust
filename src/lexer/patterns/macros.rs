/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Rust macros
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// exports //
/////////////

pub use super::*;

macro_rules! quote {
    ($string:expr) => ({
        use regex;
        regex::quote(&$string)
    });
}

macro_rules! regex_concat {
    ($modifier:expr, $pattern:expr) => ({
        //format!("/{}/{}", $pattern, $modifier)
        &("/".to_string() + &$pattern + "/" + &$modifier)
    });
}

macro_rules! try_new_regex {
    ($regex:expr) => ({
        use regex::Regex;

        try!(Regex::new(&$regex))
    });
}
