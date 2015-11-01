// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Typisation of runtime errors.
///
/// @author Colin Kiegel <kiegel@gmx.de>


/////////////
// imports //
/////////////

use error;

/////////////
// exports //
/////////////

pub type RuntimeError = error::Exception<RuntimeErrorCode>;

#[derive(Debug, PartialEq)]
pub enum RuntimeErrorCode {
    Unknown,
}

impl ToString for RuntimeErrorCode {
    fn to_string(&self) -> String {
        match *self {
            RuntimeErrorCode::Unknown => "Unknown",
        }.to_string()
    }
}
