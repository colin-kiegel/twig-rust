/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Twig configuration options.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use compiler::extension::escaper;
use compiler::extension::optimizer;

/////////////
// exports //
/////////////


#[derive(Debug, PartialEq)]
pub enum Charset {
    UTF8
}

impl Default for Charset {
    fn default() -> Charset {
        Charset::UTF8
    }
}

#[derive(Debug, PartialEq)]
pub struct Options {
    /// debug: When set to true, it automatically set "auto_reload" to true as
    ///     well (default to false).
    pub debug: bool,

    /// charset: The charset used by the templates (default to UTF-8).
    pub charset: Charset,

    /// base_template_class: The base template class to use for generated
    ///     templates (default to Twig_Template).
    ///     TODO: We can remove that - since we will not support this kind of code-generation
    pub _base_template_class: String,

    /// strict_variables: Whether to ignore invalid variables in templates
    ///     (default to false).
    pub strict_variables: bool,

    /// autoescape: Whether to enable auto-escaping (default to html):
    ///     * false: disable auto-escaping
    ///     * true: equivalent to html
    ///     * html, js: set the autoescaping to one of the supported strategies
    ///     * filename: set the autoescaping strategy based on the template filename extension
    ///     * PHP callback: a PHP callback that returns an escaping strategy based on the template "filename"
    pub autoescape: escaper::Mode,

    /// cache: An absolute path where to store the compiled templates (optional)
    pub cache: Option<String>,

    /// auto_reload: Whether to reload the template if the original source changed (optional).
    ///     If you don't provide the auto_reload option, it will be
    ///     determined automatically based on the debug value.
    pub auto_reload: Option<bool>,

    /// optimizations: A flag that indicates whether optimizations are applied
    pub optimizations: optimizer::Mode,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            debug: false,
            charset: Charset::default(),
            _base_template_class: "Twig_Template".to_string(),
            strict_variables: false,
            autoescape: escaper::Mode::default(),
            cache: None,
            auto_reload: None,
            optimizations: optimizer::Mode::default(),
        }
    }
}
