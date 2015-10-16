/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Typisation of loader errors.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use error;

/////////////
// exports //
/////////////

pub type LoaderError = error::Exception<LoaderErrorCode>;

#[derive(Debug, PartialEq)]
pub enum LoaderErrorCode {
    Unknown,
    TemplateNotFound,
    NotInitialized,
    InvalidPath,
}

impl ToString for LoaderErrorCode {
    fn to_string(&self) -> String {
        match *self {
            LoaderErrorCode::Unknown => "Unknown",
            LoaderErrorCode::TemplateNotFound => "TemplateNotFound",
            LoaderErrorCode::NotInitialized => "NotInitialized",
            LoaderErrorCode::InvalidPath => "InvalidPath",
        }.to_string()
    }
}
