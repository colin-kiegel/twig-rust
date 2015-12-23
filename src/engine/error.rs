// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Typisation of syntax errors.

use std::fmt::{self, Display};
use std::error::Error;

use loader::LoaderError;
use engine::parser::{ParserError, LexerError};
use extension;


#[derive(Debug)]
pub enum TwigError {
    Loader(LoaderError),
    Lexer(LexerError),
    Parser(ParserError),
    ExtensionRegistry(ExtensionRegistryError),
    Unreachable {
        reason: String,
    },
    LoaderNotInitialized,
    LexerNotInitialized,
    Runtime,
}

impl From<LoaderError> for TwigError {
    fn from(err: LoaderError) -> TwigError {
        TwigError::Loader(err)
    }
}

impl From<LexerError> for TwigError {
    fn from(err: LexerError) -> TwigError {
        TwigError::Lexer(err)
    }
}

impl From<ParserError> for TwigError {
    fn from(err: ParserError) -> TwigError {
        TwigError::Parser(err)
    }
}

impl From<ExtensionRegistryError> for TwigError {
    fn from(err: ExtensionRegistryError) -> TwigError {
        TwigError::ExtensionRegistry(err)
    }
}

impl Error for TwigError {
    fn description(&self) -> &str {
        match *self {
            TwigError::Loader(..) => "Twig loader error.",
            TwigError::Lexer(..) => "Twig lexer error.",
            TwigError::Parser(..) => "Twig parser error.",
            TwigError::ExtensionRegistry(..) => "Twig extension registry error.",
            TwigError::Unreachable{..} => {
                "Unexptected twig error (please report as bug with details)."
            }
            TwigError::LoaderNotInitialized => {
                "The template loader must be initializied prior usage."
            }
            TwigError::LexerNotInitialized => {
                "The template lexer must be initializied prior usage."
            }
            TwigError::Runtime => "Twig runtime error.",
        }
    }
}

impl Display for TwigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.description()));

        match *self {
            TwigError::Loader(ref e) => Display::fmt(e, f),
            TwigError::Lexer(ref e) => Display::fmt(e, f),
            TwigError::Parser(ref e) => Display::fmt(e, f),
            TwigError::ExtensionRegistry(ref e) => Display::fmt(e, f),
            TwigError::Unreachable {
                ref reason
            } => write!(f, " {}.", reason),
            TwigError::LoaderNotInitialized |
            TwigError::LexerNotInitialized |
            TwigError::Runtime => Ok(()),
        }
    }
}

#[derive(Debug)]
pub enum ExtensionRegistryError {
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

impl Error for ExtensionRegistryError {
    fn description(&self) -> &str {
        match *self {
            ExtensionRegistryError::AlreadyInitialized => {
                "Engine extensions are already initialized."
            }
            ExtensionRegistryError::NotInitialized => "Engine extensions are not yet initialized.",
            ExtensionRegistryError::DuplicateExtension{..} => "Duplicate extension.",
            ExtensionRegistryError::DuplicateFilter{..} => "Duplicate filter.",
            ExtensionRegistryError::DuplicateFunction{..} => "Duplicate function.",
            ExtensionRegistryError::DuplicateOperatorBinary{..} => "Duplicate binary operator.",
            ExtensionRegistryError::DuplicateOperatorUnary{..} => "Duplicate unary operator.",
            ExtensionRegistryError::DuplicateTest{..} => "Duplicate test.",
            ExtensionRegistryError::DuplicateTagHandler{..} => "Duplicate tag handler.",
            ExtensionRegistryError::DuplicateTokenParser{..} => "Duplicate token parser.",
        }
    }
}

impl Display for ExtensionRegistryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.description()));

        match *self {
            ExtensionRegistryError::AlreadyInitialized => Ok(()),
            ExtensionRegistryError::NotInitialized => Ok(()),
            ExtensionRegistryError::DuplicateExtension {
                prev: ref p
            } => write!(f, " {prev:?} has already been registered.", prev = p),
            ExtensionRegistryError::DuplicateFilter {
                prev: ref p, ext_name: ref x
            } => {
                write!(f,
                       " {prev:?} has already been registered, while loading extension {ext:?}.",
                       prev = p,
                       ext = x)
            }
            ExtensionRegistryError::DuplicateFunction {
                prev: ref p, ext_name: ref x
            } => {
                write!(f,
                       " {prev:?} has already been registered, while loading extension {ext:?}.",
                       prev = p,
                       ext = x)
            }
            ExtensionRegistryError::DuplicateOperatorBinary {
                prev: ref p, ext_name: ref x
            } => {
                write!(f,
                       " {prev:?} has already been registered, while loading extension {ext:?}.",
                       prev = p,
                       ext = x)
            }
            ExtensionRegistryError::DuplicateOperatorUnary {
                prev: ref p, ext_name: ref x
            } => {
                write!(f,
                       " {prev:?} has already been registered, while loading extension {ext:?}.",
                       prev = p,
                       ext = x)
            }
            ExtensionRegistryError::DuplicateTest {
                prev: ref p, ext_name: ref x
            } => {
                write!(f,
                       " {prev:?} has already been registered, while loading extension {ext:?}.",
                       prev = p,
                       ext = x)
            }
            ExtensionRegistryError::DuplicateTagHandler {
                prev: ref p, ext_name: ref x
            } => {
                write!(f,
                       " {prev:?} has already been registered, while loading extension {ext:?}.",
                       prev = p,
                       ext = x)
            }
            ExtensionRegistryError::DuplicateTokenParser {
                prev: ref p, ext_name: ref x
            } => {
                write!(f,
                       " {prev:?} has already been registered, while loading extension {ext:?}.",
                       prev = p,
                       ext = x)
            }
        }
    }
}
