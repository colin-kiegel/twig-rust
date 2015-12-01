// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Twig library for rust.
//!
//! # Examples
//!
//! ```
//! use twig::engine::Setup;
//! use twig::loader;
//! use twig::runtime::Runtime;
//! use twig::template::api::Template;
//!
//! let mut loader = loader::array::Array::default();
//! loader.set_template("greetings","Hello {{name}}!");
//!
//! let mut engine = Setup::default().engine().unwrap();
//! engine.set_loader(Box::new(loader));
//!
//! let mut runtime = Runtime::default();
//! runtime.set("name", "world");
//!
//! let compiled = engine.load_template("greetings", None).unwrap();
//! assert_eq!(&compiled.render(&runtime).unwrap(), "Hello world!")
//! ```

extern crate regex;

#[macro_use]
pub mod error;
pub mod engine;
pub mod runtime;
pub mod loader;
pub mod template;
pub mod extension;
