// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Twig macro for error handling

pub use super::*;


#[macro_export]
macro_rules! err {
    ( $code:expr ) => ({
        Err(::error::Error::new($code, loc!()))
    });
}

#[macro_export]
macro_rules! loc {
    () => ({
        ::error::Location {
            module_path : module_path!(),
            filename : file!(),
            line : line!(),
            column : column!(),
        }
    });
}

// `try_chain!`-macro will create an error-chain with location for each chaining-operation
//
// use as follows
//
// fn foo() -> Result<(), Exception<CODE_A>> {
//    let result_B: Result<(), Exception<CODE_B>> = ...;
//
//    try_chain!(result_B);
// }
macro_rules! try_chain {
    ( $result:expr ) => (
        match $result {
            Ok(value) => value,
            Err(cause) => {
                let code = ::error::api::GeneralizeTo::generalize(cause.code());

                return Err(::error::Error::new(code, loc!())
                    .caused_by(cause)
                    .into())
            }
        }
    )
}
