
// http://rustbyexample.com/mod/split.html

// mod my;
// 
// This declaration will look for a file named `my.rs` or `my/mod.rs` and will
// insert its contents inside a module named `my` under this scope

#[macro_use]
pub mod error;

pub mod lexer;
pub mod environment;
pub mod template;

pub use error::syntax;
pub use error::syntax::Code;
pub use error::aliases;
pub use error::aliases::SyntaxError;

#[test]
fn it_works() {
    // TODO
}
