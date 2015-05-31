/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * The patterns used by the lexer to tokenize the templates.
 * 
 * Written as regular expressions (perl-style).
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

use super::options::Options;

use std::rc::Rc;

const REGEX_NAME            : &'static str = "/[a-zA-Z_\\x7f-\\xff][a-zA-Z0-9_\\x7f-\\xff]*/A";
const REGEX_NUMBER          : &'static str = "/[0-9]+(?:\\.[0-9]+)?/A";
const REGEX_STRING          : &'static str = "/\"([^#\"\\\\]*(?:\\\\.[^#\"\\\\]*)*)\"|\'([^\'\\\\]*(?:\\\\.[^\'\\\\]*)*)\'/As"; // TODO check if it should be ' instead of \'
const REGEX_DQ_STRING_DELIM : &'static str = "/\"/A";
const REGEX_DQ_STRING_PART  : &'static str = "/[^#\"\\\\]*(?:(?:\\\\.|#(?!\\{))[^#\"\\\\]*)*/As";

#[derive(Default)]
pub struct RegexPatterns {
    var: String,
    block: String,
    raw_data: String,
    operator: String,
    comment: String,
    block_raw: String,
    block_line: String,
    tokens_start: String,
    interpolation_start: String,
    interpolation_end: String,
}


macro_rules! preg_quote {
    ($( $x:expr ),+ ) => ({
            let z = 3;
            concat($($x),*)
        });
}

macro_rules! regex {
    ( $pattern:expr, $modifier:expr ) => ({
            "/".to_string() + $pattern + "/" + $modifier
        });
}

impl From<Rc<Options>> for RegexPatterns {
    fn from(opt: Rc<Options>) -> RegexPatterns {
        unimplemented!();
        
        let o = opt.clone();
        
        RegexPatterns {
            // PHP: '/\s*'.preg_quote($this->options['whitespace_trim'].$this->options['tag_variable'][1], '/').'\s*|\s*'.preg_quote($this->options['tag_variable'][1], '/').'/A',
            var: //regex!(
                format!("\\s*{ws}{v1}\\s*|\\s*{v1}",
                    ws= o.whitespace_trim,
                    v1= o.tag_variable.1),
                //"A"),
            block: format!(""),
            // '/\s*(?:'.preg_quote!(opt.whitespace_trim.opt.tag_block[1], "/").'\s*|\s*'.preg_quote!(opt.tag_block[1], "/").')\n?/A',
            raw_data: format!(""),
            // '/('.preg_quote!(opt.tag_block[0].opt.whitespace_trim, "/").'|'.preg_quote!(opt.tag_block[0], "/").')\s*(?:end%s)\s*(?:'.preg_quote!(opt.whitespace_trim.opt.tag_block[1], "/").'\s*|\s*'.preg_quote!(opt.tag_block[1], "/").')/s',
            operator: format!(""),
            // opt.getOperatorRegex(),
            comment: format!(""),
            // '/(?:'.preg_quote!(opt.whitespace_trim, "/").preg_quote!(opt.tag_comment[1], "/").'\s*|'.preg_quote!(opt.tag_comment[1], "/").')\n?/s',
            block_raw: format!(""),
            // '/\s*(raw|verbatim)\s*(?:'.preg_quote!(opt.whitespace_trim.opt.tag_block[1], "/").'\s*|\s*'.preg_quote!(opt.tag_block[1], "/").')/As',
            block_line: format!(""),
            // '/\s*line\s+(\d+)\s*'.preg_quote!(opt.tag_block[1], "/").'/As',
            tokens_start: format!(""),
            // '/('.preg_quote!(opt.tag_variable[0], "/").'|'.preg_quote!(opt.tag_block[0], "/").'|'.preg_quote!(opt.tag_comment[0], "/").')('.preg_quote!(opt.whitespace_trim, "/").')?/s',
            interpolation_start: format!(""),
            // '/'.preg_quote!(opt.interpolation[0], "/").'\s*/A',
            interpolation_end: format!(""),
            // '/\s*'.preg_quote!(opt.interpolation[1], "/").'/A',
        }    
    }
}



impl RegexPatterns {
    pub fn new(opt: Rc<Options>) -> RegexPatterns {
        RegexPatterns::from(opt)
    }
}

