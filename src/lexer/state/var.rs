/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Var state of the lexer.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */
use super::*;

#[allow(dead_code)]
struct Var;

impl State for Var {
    fn lex(&self) -> Result<Option<Box<State>>,SyntaxError> {
        /*
        if (empty($this->brackets) && preg_match($this->regexes['lex_var'], $this->code, $match, null, $this->cursor)) {
            $this->pushToken(Twig_Token::VAR_END_TYPE);
            $this->moveCursor($match[0]);
            $this->popState();
        } else {
            $this->lexExpression();
        }
        */
        unimplemented!();
    }
}
