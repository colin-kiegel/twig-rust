/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Options of the lexer.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

pub struct Options {
    tag_comment: (String, String),
    tag_block: (String,String),
    tag_variable: (String,String),
    whitespace_trim: String,
    interpolation: (String,String),
}

impl Default for Options {
    fn default() -> Options {
        Options {
            tag_comment     : ("{#".to_string(), "#}".to_string()),
            tag_block       : ("{%".to_string(), "%}".to_string()),
            tag_variable    : ("{{".to_string(), "}}".to_string()),
            whitespace_trim : "-".to_string(),
            interpolation   : ("#{".to_string(), "}".to_string()),
        }
    }
}
