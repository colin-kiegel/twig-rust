// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Options of the lexer.
///
/// @author Colin Kiegel <kiegel@gmx.de>


/////////////
// imports //
/////////////

use regex;

/////////////
// exports //
/////////////

#[derive(Debug, PartialEq)]
pub struct OptionData {
    raw:    String,
    quoted: String,
}

#[derive(Debug, PartialEq)]
pub struct Options {
    pub interpolation_start:    OptionData,
    pub interpolation_end:      OptionData,
    pub tag_block_start:        OptionData,
    pub tag_block_end:          OptionData,
    pub tag_comment_start:      OptionData,
    pub tag_comment_end:        OptionData,
    pub tag_expression_start:     OptionData,
    pub tag_variable_end:       OptionData,
    pub whitespace_trim:        OptionData,
}

impl<'a> Into<OptionData> for &'a str {
    fn into(self) -> OptionData {
        OptionData {
            raw: self.to_string(),
            quoted: regex::quote(&self),
        }
    }
}

impl OptionData {
    pub fn raw(&self) -> &str {
        self.raw.as_ref()
    }

    pub fn quoted(&self) -> &str {
        self.quoted.as_ref()
    }
}

impl Default for Options {
    fn default() -> Options {
        Options {
            interpolation_start : "#{".into(),
            interpolation_end   : "}".into(),
            tag_block_start     : "{%".into(),
            tag_block_end       : "%}".into(),
            tag_comment_start   : "{#".into(),
            tag_comment_end     : "#}".into(),
            tag_expression_start  : "{{".into(),
            tag_variable_end    : "}}".into(),
            whitespace_trim     : "-".into(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn default() {
        let opt_o = Options::default();
        let opt_x = Options {
            interpolation_start : OptionData { raw: "#{".into(), quoted: r"\#\{".into()},
            interpolation_end   : OptionData { raw: "}".into(),  quoted: r"\}".into()},
            tag_block_start     : OptionData { raw: "{%".into(), quoted: r"\{%".into()},
            tag_block_end       : OptionData { raw: "%}".into(), quoted: r"%\}".into()},
            tag_comment_start   : OptionData { raw: "{#".into(), quoted: r"\{\#".into()},
            tag_comment_end     : OptionData { raw: "#}".into(), quoted: r"\#\}".into()},
            tag_expression_start  : OptionData { raw: "{{".into(), quoted: r"\{\{".into()},
            tag_variable_end    : OptionData { raw: "}}".into(), quoted: r"\}\}".into()},
            whitespace_trim     : OptionData { raw: "-".into(),  quoted: r"-".into()},
        };

        assert_eq!(opt_o, opt_x);
    }
}
