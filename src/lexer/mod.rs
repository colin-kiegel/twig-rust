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

// exports
pub mod error;
pub mod job;
pub mod options;
pub use self::options::Options;
pub use self::error::*;//{SyntaxErrorCode, SyntaxError};


// imports
#[cfg(test)]
mod test;
mod regex_patterns;
mod token;
use self::regex_patterns::RegexPatterns;
use std::rc::Rc;
use template;
use environment::Environment;
use lexer::job::state::Tokenize;
use lexer::job::Job;

// TODO: where does this belong?
//const PUNCTUATION           : &'static str = "()[]{}?:.,|";

#[allow(dead_code)]
struct Lexer {
    env: Rc<Environment>,
    options: Rc<Options>,
    patterns: Rc<RegexPatterns>,
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl<'r, 't> Lexer {
    pub fn new(env: Environment, opt: Options) -> Lexer {
        let env = Rc::new(env);
        let opt = Rc::new(opt);
        let patterns = Rc::new(RegexPatterns::new(env.clone(), opt.clone())
                        .unwrap()); // TODO Error-Handling
        
        Lexer {
            env: env,
            options: opt,
            patterns: patterns,
        }
    }
    
    pub fn tokenize(&'t mut self, template: &'r Rc<template::Raw>) -> Result<token::Stream, SyntaxError>
        where 'r: 't // the template must outlive the Lexer
    {
        // TODO set/handle encoding (note: Twig-PHP assumes ASCII)
        let job = Job::new(template, &self.patterns);
        
        job.tokenize()
    }
    
    fn lex_expression(&self) {
    /*
        // whitespace
        if (preg_match('/\s+/A', $this->code, $match, null, $this->cursor)) {
            $this->moveCursor($match[0]);

            if ($this->cursor >= $this->end) {
                throw new Twig_Error_Syntax(sprintf('Unclosed "%s"', $this->state === self::STATE_BLOCK ? 'block' : 'variable'), $this->currentVarBlockLine, $this->filename);
            }
        }

        // operators
        if (preg_match($this->regexes['operator'], $this->code, $match, null, $this->cursor)) {
            $this->pushToken(Twig_Token::OPERATOR_TYPE, preg_replace('/\s+/', ' ', $match[0]));
            $this->moveCursor($match[0]);
        }
        // names
        elseif (preg_match(self::REGEX_NAME, $this->code, $match, null, $this->cursor)) {
            $this->pushToken(Twig_Token::NAME_TYPE, $match[0]);
            $this->moveCursor($match[0]);
        }
        // numbers
        elseif (preg_match(self::REGEX_NUMBER, $this->code, $match, null, $this->cursor)) {
            $number = (float) $match[0];  // floats
            if (ctype_digit($match[0]) && $number <= PHP_INT_MAX) {
                $number = (int) $match[0]; // integers lower than the maximum
            }
            $this->pushToken(Twig_Token::NUMBER_TYPE, $number);
            $this->moveCursor($match[0]);
        }
        // punctuation
        elseif (false !== strpos(self::PUNCTUATION, $this->code[$this->cursor])) {
            // opening bracket
            if (false !== strpos('([{', $this->code[$this->cursor])) {
                $this->brackets[] = array($this->code[$this->cursor], $this->lineno);
            }
            // closing bracket
            elseif (false !== strpos(')]}', $this->code[$this->cursor])) {
                if (empty($this->brackets)) {
                    throw new Twig_Error_Syntax(sprintf('Unexpected "%s"', $this->code[$this->cursor]), $this->lineno, $this->filename);
                }

                list($expect, $lineno) = array_pop($this->brackets);
                if ($this->code[$this->cursor] != strtr($expect, '([{', ')]}')) {
                    throw new Twig_Error_Syntax(sprintf('Unclosed "%s"', $expect), $lineno, $this->filename);
                }
            }

            $this->pushToken(Twig_Token::PUNCTUATION_TYPE, $this->code[$this->cursor]);
            ++$this->cursor;
        }
        // strings
        elseif (preg_match(self::REGEX_STRING, $this->code, $match, null, $this->cursor)) {
            $this->pushToken(Twig_Token::STRING_TYPE, stripcslashes(substr($match[0], 1, -1)));
            $this->moveCursor($match[0]);
        }
        // opening double quoted string
        elseif (preg_match(self::REGEX_DQ_STRING_DELIM, $this->code, $match, null, $this->cursor)) {
            $this->brackets[] = array('"', $this->lineno);
            $this->pushState(self::STATE_STRING);
            $this->moveCursor($match[0]);
        }
        // unlexable
        else {
            throw new Twig_Error_Syntax(sprintf('Unexpected character "%s"', $this->code[$this->cursor]), $this->lineno, $this->filename);
        }
    */    
    }
    
    fn lex_raw_data(&self) {
    /*
        if (!preg_match(str_replace('%s', $tag, $this->regexes['lex_raw_data']), $this->code, $match, PREG_OFFSET_CAPTURE, $this->cursor)) {
            throw new Twig_Error_Syntax(sprintf('Unexpected end of file: Unclosed "%s" block', $tag), $this->lineno, $this->filename);
        }

        $text = substr($this->code, $this->cursor, $match[0][1] - $this->cursor);
        $this->moveCursor($text.$match[0][0]);

        if (false !== strpos($match[1][0], $this->options['whitespace_trim'])) {
            $text = rtrim($text);
        }

        $this->pushToken(Twig_Token::TEXT_TYPE, $text);
    */    
    }
    
    fn lex_comment(&self) {
    /*
        if (!preg_match($this->regexes['lex_comment'], $this->code, $match, PREG_OFFSET_CAPTURE, $this->cursor)) {
            throw new Twig_Error_Syntax('Unclosed comment', $this->lineno, $this->filename);
        }

        $this->moveCursor(substr($this->code, $this->cursor, $match[0][1] - $this->cursor).$match[0][0]);
    }

    protected function lexString()
    {
        if (preg_match($this->regexes['interpolation_start'], $this->code, $match, null, $this->cursor)) {
            $this->brackets[] = array($this->options['interpolation'][0], $this->lineno);
            $this->pushToken(Twig_Token::INTERPOLATION_START_TYPE);
            $this->moveCursor($match[0]);
            $this->pushState(self::STATE_INTERPOLATION);
        } elseif (preg_match(self::REGEX_DQ_STRING_PART, $this->code, $match, null, $this->cursor) && strlen($match[0]) > 0) {
            $this->pushToken(Twig_Token::STRING_TYPE, stripcslashes($match[0]));
            $this->moveCursor($match[0]);
        } elseif (preg_match(self::REGEX_DQ_STRING_DELIM, $this->code, $match, null, $this->cursor)) {
            list($expect, $lineno) = array_pop($this->brackets);
            if ($this->code[$this->cursor] != '"') {
                throw new Twig_Error_Syntax(sprintf('Unclosed "%s"', $expect), $lineno, $this->filename);
            }

            $this->popState();
            ++$this->cursor;
        }
    */
    }
}
