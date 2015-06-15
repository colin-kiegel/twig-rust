/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Interpolation state of the lexer.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */
use super::*;

#[allow(dead_code)]
struct Interpolation(State);

impl Tokenize for Interpolation {
    fn lex<T>(&mut self) -> &mut T 
    where T: Tokenize {
    /*
        $bracket = end($this->brackets);
        if ($this->options['interpolation'][0] === $bracket[0] && preg_match($this->regexes['interpolation_end'], $this->code, $match, null, $this->cursor)) {
            array_pop($this->brackets);
            $this->pushToken(Twig_Token::INTERPOLATION_END_TYPE);
            $this->moveCursor($match[0]);
            $this->popState();
        } else {
            $this->lexExpression();
        }
    */
        unimplemented!();
    }
}
