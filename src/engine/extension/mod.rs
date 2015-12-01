// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Twig Extensions
//!
//! * define new behavior during the compilation process
//!   * **token parser**: transforms a sub-sequence from the token stream (=lexed template) to nodes in the abstract syntax tree. E.g. the `TokenParserIf` parses complex if-statements (if, elseif, else, endif) and creates the if-node with according child nodes for each test and conditional branch.
//!   * **node visitor**: modifies the abstract syntax tree immediately after parsing. E.g. the `optimizer` extension defines the `optimizeRawFilter` node visitor which strips all "raw" filters from the syntax tree.
//! * extensions define specific new *node types* in the abstract syntax tree - falling into the following generic classes. Note that all *examples* are defined in the `core` extension, if not stated otherwise. The core extension is not yet fully implemented (see [CHANGELOG][changelog]).
//!   * **test**: can be used in conditional statements. E.g. the `defined` test checks if a variable is defined in the current context.
//!   * **unary operator**: can be used in variable expressions to process results. E.g. the `-` (neg) operator inverts the sign of a numeric result.
//!   * **binary operator**: can be used in variable expressions to combine two results. E.g. the `**` (power) operator takes one number to the power of another number.
//!   * **function**: can be used to perform complex computations. E.g. the `round` function rounds a floating number with a given precision.
//!   * **filter** can modify the result of variable expressions. E.g. the `default` filter returns the result of the variable expression if it is defined, otherwise it returns the default value. The `escaper` filter escapes the result according to the output channel (html, html attribute, css, js, url, ..)
//!   * **global**: can be used to define global constants. Templates can test for these global constants to trigger conditional behavior, or use them as argument for functions, etc

pub mod api;
pub mod core;
pub mod debug;
pub mod escaper;
pub mod optimizer;
pub mod profiler;
pub mod sandbox;
pub mod staging;
pub mod string_loader;
pub use self::core::Core;
pub use self::debug::Debug;
pub use self::escaper::Escaper;
pub use self::optimizer::Optimizer;
pub use self::profiler::Profiler;
pub use self::sandbox::Sandbox;
pub use self::staging::Staging;
pub use self::string_loader::StringLoader;
