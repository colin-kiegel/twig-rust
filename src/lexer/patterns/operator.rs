/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * The `operator` pattern used by the lexer to tokenize the templates.
 *
 * Written as regular expressions (perl-style).
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use regex;
use regex::Error as regexError;
use environment::Environment;

/////////////
// exports //
/////////////

pub type Regex = regex::Regex;


#[allow(dead_code)]
pub struct Match;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn regex(env: &Environment) -> Result<Regex, regexError> {
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
    unimplemented!()
}

#[cfg(test)]
mod test {
    use environment::Environment;
    use regex;

    #[test]
    pub fn regex() {
        let rx_o = super::regex(&Environment::default()).unwrap();
        let rx_x = regex::Regex::new(r"").unwrap();

        assert_eq!(rx_o, rx_x);
    }
}
