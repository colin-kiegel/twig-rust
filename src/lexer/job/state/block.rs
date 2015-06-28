/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Block state of the lexer.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use super::*;


#[allow(dead_code)]
struct Block(State);

impl Tokenize for Block {
    fn lex<T>(&mut self) -> &mut T
    where T: Tokenize {
        /*
        if (empty($this->brackets) && preg_match($this->regexes['lex_block'], $this->code, $match, null, $this->cursor)) {
            $this->pushToken(Twig_Token::BLOCK_END_TYPE);
            $this->moveCursor($match[0]);
            $this->popState();
        } else {
            $this->lexExpression();
        }
        */
        unimplemented!();
    }
}
