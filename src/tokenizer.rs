/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */


/**
 * Trait implemented by lexer classes.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 *
 * @deprecated since 1.12 (to be removed in 3.0)
 */
 
trait Tokenizer {
    /// Tokenizes a source code
    /// 
    /// Returns a TokenStream instance.
    /// Originally called Twig_LexerInterface in Twig PHP
    ///
    /// # Arguments
    ///
    /// * `code` - The source code
    /// * `filename` - A unique identifier for the source code
    ///
    /// # Failures
    ///
    /// * When the code is syntactically wrong

    pub fn tokenize(code: String, filename: Option<String>) -> Vec<Token>;
}
