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

    ( $code:expr, $message:expr ) => ({
            ::error::Exception::new(err_details!(Some($message.to_string())), $code)
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
