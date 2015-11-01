// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Twig Template API
///
/// @author Colin Kiegel <kiegel@gmx.de>


/////////////
// imports //
/////////////

use compiler::TwigError;
use runtime::Runtime;

/////////////
// exports //
/////////////

pub const _ANY_CALL    : &'static str = "any";
pub const _ARRAY_CALL  : &'static str = "array";
pub const _METHOD_CALL : &'static str = "method";

pub trait Template {
    /// Renders the template with the given context and returns it as string.
    fn render(&self, runtime: &Runtime) -> Result<String, TwigError>;

    /// Displays the template with the given context.
    ///
    /// context is an array of parameters to pass to the template
    /// blocks is an array of blocks to pass to the template
    fn display(&self, runtime: &Runtime, blocks: Option<Vec<()>>); // #TODO:210 error handling

    // /**
    //  * Get the bound compiler for this template.
    //  */
    // fn compiler(&self) -> &Compiler;
}
