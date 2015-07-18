/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Twig Templates.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use compiler::Compiler;

/////////////
// exports //
/////////////

pub mod compiled;
pub mod raw;
pub use self::compiled::Compiled;
pub use self::raw::{Raw, Cursor};
pub use self::raw::cursor;


pub const _ANY_CALL    : &'static str = "any";
pub const _ARRAY_CALL  : &'static str = "array";
pub const _METHOD_CALL : &'static str = "method";

pub trait Template {

    /**
     * Renders the template with the given context and returns it as string.
     *
     * @param array $context An array of parameters to pass to the template
     *
     * @return string The rendered template
     */
    fn render(&self, context: Vec<()>) -> String; // TODO error handling

    /**
     * Displays the template with the given context.
     *
     * @param array $context An array of parameters to pass to the template
     * @param array $blocks  An array of blocks to pass to the template
     */
    fn display(&self, context: Vec<()>, blocks: Option<Vec<()>>); // TODO error handling

    /**
     * Get the bound compiler for this template.
     */
    fn compiler(&self) -> &Compiler;
}
