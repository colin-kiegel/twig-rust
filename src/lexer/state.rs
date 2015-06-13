/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * State of the lexer.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

#[allow(dead_code)]
pub enum State {
    Data            = 0,
    Block           = 1,
    Var             = 2,
    String          = 3,
    Interpolation   = 4,
}

#[allow(dead_code)]
impl Default for State {
    fn default() -> State {
        State::Data
    }
}
