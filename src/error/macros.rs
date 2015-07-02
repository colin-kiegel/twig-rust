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
            Err(error!($code))
        });

    ( $code:expr, $message:expr ) => ({
            Err(error!($code, $message))
        });
    ( $code:expr, $message:expr, $cause:expr ) => ({
            Err(error!($code, $message, $cause))
        });
}

#[macro_export]
macro_rules! error {
    ( $code:expr ) => ({
            ::error::Error::new(err_details!(None), $code)
        });

    ( $code:expr, $message:expr ) => ({
            ::error::Error::new(err_details!(Some($message.to_string())), $code) // TODO: treat Strings differently
        });
    ( $code:expr, $message:expr, $cause:expr ) => ({
            ::error::Error::new(err_details!(Some($message.to_string())), $code).chain(Box::new($cause))
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
