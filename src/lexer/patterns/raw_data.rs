/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * The `raw_data` pattern used by the lexer to tokenize the templates.
 *
 * Written as regular expressions (perl-style).
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use super::*;
use regex;
use regex::Error as regexError;

/////////////
// exports //
/////////////

pub type Regex = regex::Regex;

#[allow(dead_code)]
pub struct Match;

pub fn regex(opt: &Options) -> Result<Regex, regexError> {
    Ok(try_new_regex!(format!(r"({b0}{ws}|{b0})\s*(?:end%s)\s*(?:{ws}{b1}\s*|\s*{b1})",
        ws = opt.whitespace_trim.quoted(),
        b0 = opt.tag_block_start.quoted(),
        b1 = opt.tag_block_end.quoted())))
}   // orig: '/('.$tag_block[0].$whitespace_trim.'|'.$tag_block[0].')\s*(?:end%s)\s*(?:'.$whitespace_trim.$tag_block[1].'\s*|\s*'.$tag_block[1].')/s'

#[cfg(test)]
mod test {
    use super::super::*;
    use regex;

    #[test]
    pub fn regex() {
        let rx_o = super::regex(&Options::default()).unwrap();
        let rx_x = regex::Regex::new(r"(\{%-|\{%)\s*(?:end%s)\s*(?:-%\}\s*|\s*%\})").unwrap();

        assert_eq!(rx_o, rx_x);
    }
}
