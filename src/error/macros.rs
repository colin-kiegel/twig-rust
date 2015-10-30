/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Rust macro
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// exports //
/////////////

pub use super::*;

#[macro_export]
macro_rules! err {
    ( $code:expr ) => ({
            ::error::Exception::new(err_details!(None), $code)
        });
    //
    // ( $code:expr, $message:expr ) => ({
    //         ::error::Exception::new(err_details!(Some($message.to_string())), $code)
    //         // #TODO:750 treat Strings differently
    //     });

    // Use the syntax described in std::fmt.
    ( $code:expr, $ ( $ arg : tt ) * ) => ({
            let message: String = format ! ( $ ( $ arg ) * );
            ::error::Exception::new(err_details!(Some(message)), $code)
            // #TODO:750 treat Strings differently
        });
}

#[macro_export]
macro_rules! err_details {
    ( $opt_message:expr ) => ({
            ::error::Details {
                message : $opt_message,
                module_path : module_path!(),
                filename : file!(),
                line : line!(),
                column : column!(),
            }
        });
}

// NOTE: because convert::From<T> already is reflexive (generic `impl From<T> for T`)
//       we can't generically `impl From<Exception<A>> for Exception<B> where ...`
//       - to bad!
//       TODO: file a bug!
#[macro_export]
macro_rules! impl_convert_exception {
    ( $source_type:ty, $target_type:ty, $target_error_code:expr ) => (
        impl convert::From<::error::Exception<$source_type>> for ::error::Exception<$target_type> {
            fn from(cause: ::error::Exception<$source_type>) -> ::error::Exception<$target_type> {
                let details = ::error::Details {
                    message: None,
                    .. *cause.details()
                };
                ::error::Exception::new(details, $target_error_code)
                    .caused_by(cause)
            }
        }
    );
}
