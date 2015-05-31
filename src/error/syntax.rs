/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Typisation of syntax errors.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

#[derive(Debug)]
pub enum Code {
    Unknown,
    UnexpectedCharacter,
    UnexpectedBracket,
    UnexpectedEof,
    UnclosedBracket,
    UnclosedComment,
}
