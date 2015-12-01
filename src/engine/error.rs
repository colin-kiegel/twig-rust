// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Typisation of syntax errors.

use std::fmt::{self, Display};
use error::Error;
use error::api::{GeneralizeTo, ErrorCode};

use loader::LoaderErrorCode;
use lexer::LexerErrorCode;
use parser::ParserErrorCode;
use runtime::RuntimeErrorCode;
use engine::extension;

pub type TwigError = Error<TwigErrorCode>;
pub type ExtensionRegistryError = Error<ExtensionRegistryErrorCode>;

#[derive(Debug)]
pub enum TwigErrorCode {
    Unreachable {
        reason: String
    },
    Loader,
    LoaderNotInitialized,
    Lexer,
    LexerNotInitialized,
    Parser,
    Runtime,
    ExtensionRegistry,
}

impl GeneralizeTo<TwigErrorCode> for LoaderErrorCode {
    fn generalize(&self) -> TwigErrorCode { TwigErrorCode::Loader }
}

impl GeneralizeTo<TwigErrorCode> for LexerErrorCode {
    fn generalize(&self) -> TwigErrorCode { TwigErrorCode::Lexer }
}

impl GeneralizeTo<TwigErrorCode> for ParserErrorCode {
    fn generalize(&self) -> TwigErrorCode { TwigErrorCode::Parser }
}

impl GeneralizeTo<TwigErrorCode> for RuntimeErrorCode {
    fn generalize(&self) -> TwigErrorCode { TwigErrorCode::Runtime }
}

impl GeneralizeTo<TwigErrorCode> for ExtensionRegistryErrorCode {
    fn generalize(&self) -> TwigErrorCode { TwigErrorCode::ExtensionRegistry }
}

impl ErrorCode for TwigErrorCode {
    fn description(&self) -> &str {
        match *self {
            TwigErrorCode::Unreachable{..} => "Unexptected twig error (please report as bug with details).",
            TwigErrorCode::Loader => "Twig loader error.",
            TwigErrorCode::LoaderNotInitialized => "The template loader must be initializied prior usage.",
            TwigErrorCode::Lexer => "Twig lexer error.",
            TwigErrorCode::LexerNotInitialized => "The template lexer must be initializied prior usage.",
            TwigErrorCode::Parser => "Twig parser error.",
            TwigErrorCode::Runtime => "Twig runtime error.",
            TwigErrorCode::ExtensionRegistry => "Twig extension registry error."
        }
    }
}

impl Display for TwigErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.description()));

        match *self {
            TwigErrorCode::Unreachable {
                ref reason
            } => {
                write!(f, " {}.", reason)
            },
            TwigErrorCode::Loader
            | TwigErrorCode::LoaderNotInitialized
            | TwigErrorCode::Lexer
            | TwigErrorCode::LexerNotInitialized
            | TwigErrorCode::Parser
            | TwigErrorCode::Runtime
            | TwigErrorCode::ExtensionRegistry
            => Ok(())
        }
    }
}

#[derive(Debug)]
pub enum ExtensionRegistryErrorCode {
    AlreadyInitialized,
    NotInitialized,
    DuplicateExtension {
        prev: Box<extension::api::Extension>,
    },
    DuplicateFilter {
        prev: Box<extension::api::Filter>,
        ext_name: &'static str,
    },
    DuplicateFunction {
        prev: Box<extension::api::Function>,
        ext_name: &'static str,
    },
    DuplicateOperatorUnary {
        prev: extension::api::UnaryOperator,
        ext_name: &'static str,
    },
    DuplicateOperatorBinary {
        prev: extension::api::BinaryOperator,
        ext_name: &'static str,
    },
    DuplicateTest {
        prev: Box<extension::api::Test>,
        ext_name: &'static str,
    },
    DuplicateTagHandler {
        prev: Box<extension::api::TokenParser>,
        ext_name: &'static str,
    },
    DuplicateTokenParser {
        prev: Box<extension::api::TokenParser>,
        ext_name: &'static str,
    },
}

impl ErrorCode for ExtensionRegistryErrorCode {
    fn description(&self) -> &str {
        match *self {
            ExtensionRegistryErrorCode::AlreadyInitialized => "Engine extensions are already initialized.",
            ExtensionRegistryErrorCode::NotInitialized => "Engine extensions are not yet initialized.",
            ExtensionRegistryErrorCode::DuplicateExtension{..} => "Duplicate extension.",
            ExtensionRegistryErrorCode::DuplicateFilter{..} => "Duplicate filter.",
            ExtensionRegistryErrorCode::DuplicateFunction{..} => "Duplicate function.",
            ExtensionRegistryErrorCode::DuplicateOperatorBinary{..} => "Duplicate binary operator.",
            ExtensionRegistryErrorCode::DuplicateOperatorUnary{..} => "Duplicate unary operator.",
            ExtensionRegistryErrorCode::DuplicateTest{..} => "Duplicate test.",
            ExtensionRegistryErrorCode::DuplicateTagHandler{..} => "Duplicate tag handler.",
            ExtensionRegistryErrorCode::DuplicateTokenParser{..} => "Duplicate token parser.",
        }
    }
}

impl Display for ExtensionRegistryErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.description()));

        match *self {
            ExtensionRegistryErrorCode::AlreadyInitialized => Ok(()),
            ExtensionRegistryErrorCode::NotInitialized => Ok(()),
            ExtensionRegistryErrorCode::DuplicateExtension {
                prev: ref p
            } => {
                write!(f, " {prev:?} has already been registered.",
                    prev = p)
            },
            ExtensionRegistryErrorCode::DuplicateFilter {
                prev: ref p, ext_name: ref x
            } => {
                write!(f, " {prev:?} has already been registered, while loading extension {ext:?}.",
                    prev = p, ext = x)
            },
            ExtensionRegistryErrorCode::DuplicateFunction {
                prev: ref p, ext_name: ref x
            } => {
                write!(f, " {prev:?} has already been registered, while loading extension {ext:?}.",
                    prev = p, ext = x)
            },
            ExtensionRegistryErrorCode::DuplicateOperatorBinary {
                prev: ref p, ext_name: ref x
            } => {
                write!(f, " {prev:?} has already been registered, while loading extension {ext:?}.",
                    prev = p, ext = x)
            },
            ExtensionRegistryErrorCode::DuplicateOperatorUnary {
                prev: ref p, ext_name: ref x
            } => {
                write!(f, " {prev:?} has already been registered, while loading extension {ext:?}.",
                    prev = p, ext = x)
            },
            ExtensionRegistryErrorCode::DuplicateTest {
                prev: ref p, ext_name: ref x
            } => {
                write!(f, " {prev:?} has already been registered, while loading extension {ext:?}.",
                    prev = p, ext = x)
            },
            ExtensionRegistryErrorCode::DuplicateTagHandler {
                prev: ref p, ext_name: ref x
            } => {
                write!(f, " {prev:?} has already been registered, while loading extension {ext:?}.",
                    prev = p, ext = x)
            },
            ExtensionRegistryErrorCode::DuplicateTokenParser {
                prev: ref p, ext_name: ref x
            } => {
                write!(f, " {prev:?} has already been registered, while loading extension {ext:?}.",
                    prev = p, ext = x)
            }
        }
    }
}
