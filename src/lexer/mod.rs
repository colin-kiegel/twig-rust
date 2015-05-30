/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Lexes a template string.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

pub mod options;
mod state;
mod regExes;

pub use lexer::options::Options;
pub use environment::Environment;
use token::Token;
use lexer::state::State;
use lexer::regExes::RegExes;

const PUNCTUATION           : &'static str = "()[]{}?:.,|";

#[derive(Default)]
struct Lexer {
    tokens: Vec<Token>,
    code: String,
    cursor: usize,
    lineno: usize,
    end: usize,
    state: State,
    states: Vec<State>,
    brackets: Vec<String>,
    env: Environment,
    filename: String,
    options: Options,
    regexes: RegExes,
    position: usize,
    positions: Vec<usize>,
    currentVarBlockLine: usize,
}

impl Lexer {
    pub fn new(env: Environment, options: Options) -> Lexer {
         
        Lexer {
            env: env,
            regexes: RegExes::new(&options),
            options: options,
            .. Default::default()
        }
    }
}
