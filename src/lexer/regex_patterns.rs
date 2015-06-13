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

use regex::Regex;
use regex::Error as regexError;

use environment::Environment;
use lexer::options::Options;

use std::rc::Rc;

//const REGEX_NAME            : &'static str = r"/[a-zA-Z_\x7f-\xff][a-zA-Z0-9_\x7f-\xff]*/A";
//const REGEX_NUMBER          : &'static str = r"/[0-9]+(?:\.[0-9]+)?/A";
//const REGEX_STRING          : &'static str = r"/\"([^#\"\\]*(?:\\.[^#\"\\]*)*)\"|\'([^\'\\]*(?:\\.[^\'\\]*)*)\'/As"; // TODO check if it should be ' instead of \'
//const REGEX_DQ_STRING_DELIM : &'static str = r"/\"/A";
//const REGEX_DQ_STRING_PART  : &'static str = r"/[^#\"\\]*(?:(?:\\.|#(?!\\{))[^#\"\\]*)*/As";

#[derive(Debug)]
#[derive(PartialEq)]
pub struct RegexPatterns {
    var: Regex,
    block: Regex,
    raw_data: Regex,
    operator: Regex,
    comment: Regex,
    block_raw: Regex,
    block_line: Regex,
    tokens_start: Regex,
    interpolation_start: Regex,
    interpolation_end: Regex,
}

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
        try!(Regex::new(&$regex))
    });
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl RegexPatterns {
    pub fn new(env: Rc<Environment>, opt: Rc<Options>) -> Result<RegexPatterns, regexError> {
        let ws = quote!(opt.whitespace_trim);
        let b0 = quote!(opt.tag_block.0);
        let b1 = quote!(opt.tag_block.1);
        let v0 = quote!(opt.tag_variable.0);
        let v1 = quote!(opt.tag_variable.1);
        let c0 = quote!(opt.tag_comment.0);
        let c1 = quote!(opt.tag_comment.1);
        let i0 = quote!(opt.interpolation.0);
        let i1 = quote!(opt.interpolation.1);
                
        Ok(RegexPatterns {
            var: try_new_regex!(format!(r"/\s*{ws}{v1}\s*|\s*{v1}/A",
                        ws = ws,
                        v1 = v1)),
                // PHP: '/\s*'.$whitespace_trim.$tag_variable[1].'\s*|\s*'.$tag_variable[1].'/A'
          
            block: try_new_regex!(format!(r"/\s*(?:{ws}{b1}\s*|\s*{b1})\n?/A",
                        ws = ws,
                        b1 = b1)),
                // PHP: '/\s*(?:'.$whitespace_trim.$tag_block[1].'\s*|\s*'.$tag_block[1].')\n?/A'
            
            raw_data: try_new_regex!(format!(r"/({b0}{ws}|{b0})\s*(?:end%s)\s*(?:{ws}{b1}\s*|\s*{b1})/s",
                        ws = ws,
                        b0 = b0,
                        b1 = b1)),
                // PHP: '/('.$tag_block[0].$whitespace_trim.'|'.$tag_block[0].')\s*(?:end%s)\s*(?:'.$whitespace_trim.$tag_block[1].'\s*|\s*'.$tag_block[1].')/s'
            
            operator: try_new_regex!(&RegexPatterns::get_operator_regex(env)),
                // PHP: $this.getOperatorRegex(),
            
            comment: try_new_regex!(format!(r"/(?:{ws}{c1}\s*|{c1})\n?/s",
                        ws = ws,
                        c1 = c1)),
                // PHP: '/(?:'.$whitespace_trim.$tag_comment[1].'\s*|'.$tag_comment[1].')\n?/s'
            
            block_raw: try_new_regex!(format!(r"/\s*(raw|verbatim)\s*(?:{ws}{b1}\s*|\s*{b1})/As",
                        ws = ws,
                        b1 = b1)),
                // PHP: '/\s*(raw|verbatim)\s*(?:'.$whitespace_trim.$tag_block[1].'\s*|\s*'.$tag_block[1].')/As'
            
            block_line: try_new_regex!(format!(r"/\s*line\s+(\d+)\s*{b1}/As",
                        b1 = b0)),
                // PHP: '/\s*line\s+(\d+)\s*'.$tag_block[1].'/As'
            
            tokens_start: try_new_regex!(format!(r"/({v0}|{b0}|{c0})({ws})?/s",
                        ws = ws,
                        b0 = b0,
                        v0 = v0,
                        c0 = c0)),
                // PHP: '/('.$tag_variable[0].'|'.$tag_block[0].'|'.$tag_comment[0].')('.$whitespace_trim.')?/s'
            
            interpolation_start: try_new_regex!(format!(r"/{i0}\s*/A",
                        i0 = i0)),
                // PHP: '/'.$interpolation[0].'\s*/A'
            
            interpolation_end: try_new_regex!(format!(r"/\s*{i1}/A",
                        i1 = i1)),
                // PHP: '/\s*'.$interpolation[1].'/A'
        })
    }
    
    fn get_operator_regex(env: Rc<Environment>) -> String {
    
        //$operators = array_merge(
            //array('='),
            // TODO array_keys($this->env->getUnaryOperators()),
            // TODO array_keys($this->env->getBinaryOperators())
        //);

        //$operators = array_combine($operators, array_map('strlen', $operators));
        //arsort($operators);

        //$regex = array();
        //foreach ($operators as $operator => $length) {
            // an operator that ends with a character must be followed by
            // a whitespace or a parenthesis
            //if (ctype_alpha($operator[$length - 1])) {
            //    $r = preg_quote($operator, '/').'(?=[\s()])';
            //} else {
            //    $r = preg_quote($operator, '/');
            //}

            // an operator with a space can be any amount of whitespaces
            //$r = preg_replace('/\s+/', '\s+', $r);

            //$regex[] = $r;
        //}

        //return '/'.implode('|', $regex).'/A';
        "".to_string()
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use environment::Environment;
    use lexer::options::Options;
    use std::rc::Rc;
    use regex::Regex;

    #[test]
    pub fn from_default_options() {
        let env = Rc::new(Environment{..Default::default()});
        let opt = Rc::new(Options{..Default::default()});
        let rp_o = RegexPatterns::new(env, opt).unwrap();
        let rp_x = RegexPatterns {
            var: Regex::new(r"/\s*-\}\}\s*|\s*\}\}/A").unwrap(),
            block: Regex::new(r"/\s*(?:-%\}\s*|\s*%\})\n?/A").unwrap(),
            raw_data: Regex::new(r"/(\{%-|\{%)\s*(?:end%s)\s*(?:-%\}\s*|\s*%\})/s").unwrap(),
            operator: Regex::new(r"").unwrap(),
            comment: Regex::new(r"/(?:-\#\}\s*|\#\})\n?/s").unwrap(),
            block_raw: Regex::new(r"/\s*(raw|verbatim)\s*(?:-%\}\s*|\s*%\})/As").unwrap(),
            block_line: Regex::new(r"/\s*line\s+(\d+)\s*\{%/As").unwrap(),
            tokens_start: Regex::new(r"/(\{\{|\{%|\{\#)(-)?/s").unwrap(),
            interpolation_start: Regex::new(r"/\#\{\s*/A").unwrap(),
            interpolation_end: Regex::new(r"/\s*\}/A").unwrap(),
        };
        
        println!(".var");
        assert_eq!(rp_o.var, rp_x.var);
        println!(".block");
        assert_eq!(rp_o.block, rp_x.block);
        println!(".raw_data");
        assert_eq!(rp_o.raw_data, rp_x.raw_data);
        println!(".operator");
        assert_eq!(rp_o.operator, rp_x.operator);
        println!(".comment");
        assert_eq!(rp_o.comment, rp_x.comment);
        println!(".block_raw");
        assert_eq!(rp_o.block_raw, rp_x.block_raw);
        println!(".block_line");
        assert_eq!(rp_o.block_line, rp_x.block_line);
        println!(".tokens_start");
        assert_eq!(rp_o.tokens_start, rp_x.tokens_start);
        println!(".interpolation_start");
        assert_eq!(rp_o.interpolation_start, rp_x.interpolation_start);
        println!(".interpolation_end");
        assert_eq!(rp_o.interpolation_end, rp_x.interpolation_end);
    }
}
