// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Rust macros

macro_rules! quote {
    ($string:expr) => ({
        use regex;
        regex::quote(&$string)
    });
}

macro_rules! try_new_regex {
    ($regex:expr) => ({
        use regex::Regex;

        try!(Regex::new(&$regex))
    });
}
