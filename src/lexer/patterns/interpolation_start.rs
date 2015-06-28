/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * The `interpolation start` pattern used by the lexer to tokenize the templates.
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
    Ok(try_new_regex!(format!(r"\A{i0}\s*",
        i0 = opt.interpolation_start.quoted())))
}   // orig: '/'.$interpolation[0].'\s*/A'

#[cfg(test)]
mod test {
    use super::super::*;
    use regex;

    #[test]
    pub fn regex() {
        let rx_o = super::regex(&Options::default()).unwrap();
        let rx_x = regex::Regex::new(r"\A\#\{\s*").unwrap();

        assert_eq!(rx_o, rx_x);
    }
}
