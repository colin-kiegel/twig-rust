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

pub use super::*;

#[macro_export]
macro_rules! err {
    ( $message:expr, $code:expr ) => ({
            //use super::*;
            
            Err(error::Error::new(err_details!($message), $code))
        });
}

#[macro_export]
macro_rules! err_details {
    ( $message:expr ) => ({
            error::Details {
                message : $message.clone(),
                module_path : module_path!(),
                filename : file!(),
                line : line!(),
                column : column!(),
            }
        });
}
