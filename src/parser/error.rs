/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Typisation of parser errors.
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

pub type NodeError = error::Exception<NodeErrorCode>;
pub type ParserError = error::Exception<ParserErrorCode>;


#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum ParserErrorCode {
    Logic,
    NodeError
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum NodeErrorCode {
    Unknown,
    Logic,
}

impl ::std::convert::From<NodeError> for ParserError {
    fn from(cause: NodeError) -> ParserError {
        let details = ::error::Details {
            message: None,
            .. *cause.details()
        };
        ::error::Exception::new(details, ParserErrorCode::NodeError)
            .caused_by(cause)
    }
}

impl ToString for ParserErrorCode {
    fn to_string(&self) -> String {
        match *self {
            ParserErrorCode::Logic => "Logic",
            ParserErrorCode::NodeError => "NodeError",
        }.to_string()
    }
}

impl ToString for NodeErrorCode {
    fn to_string(&self) -> String {
        match *self {
            NodeErrorCode::Unknown => "Unknown",
            NodeErrorCode::Logic => "Logic",
        }.to_string()
    }
}
