// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Typisation of loader errors.
///
/// @author Colin Kiegel <kiegel@gmx.de>


/////////////
// imports //
/////////////

use std::fmt::{self, Display};
use error::Error;
use error::api::ErrorCode;
use std::path::PathBuf;

/////////////
// exports //
/////////////

pub type LoaderError = Error<LoaderErrorCode>;

#[derive(Debug)]
pub enum LoaderErrorCode {
    Unreachable {
        reason: String
    },
    ArrayTemplateNotFound {
        name: String
    },
    FileSystemTemplateNotFound {
        raw_path: PathBuf,
        namespace: String,
        dirs: Vec<PathBuf>
    },
    FileSystemNamespaceNotInitialized {
        namespace: String
    },
    FileSystemMalformedNamespacedPath {
        template_name: String
    },
    FileSystemInvalidPath {
        path: PathBuf
    },
    FileSystemTemplateNotReadable {
        name: String,
        path: PathBuf
    }
}

impl ErrorCode for LoaderErrorCode {
    fn description(&self) -> &str {
        match *self {
            LoaderErrorCode::Unreachable{..} => "Unexptected template loader error (please report as bug with details).",
            LoaderErrorCode::ArrayTemplateNotFound{..}
            | LoaderErrorCode::FileSystemTemplateNotFound{..} => "Template not found.",
            LoaderErrorCode::FileSystemNamespaceNotInitialized{..} => "Loader is not initialized.",
            LoaderErrorCode::FileSystemMalformedNamespacedPath{..}
            | LoaderErrorCode::FileSystemInvalidPath{..} => "Invalid template path.",
            LoaderErrorCode::FileSystemTemplateNotReadable{..} => "Could not read template file.",
        }
    }
}

impl Display for LoaderErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.description()));

        match *self {
            LoaderErrorCode::Unreachable {
                ref reason
            } => {
                write!(f, " {}", reason)
            },
            LoaderErrorCode::ArrayTemplateNotFound {
                ref name
            } => {
                write!(f, " Template {:?} is not present in array template loader.", name)
            },
            LoaderErrorCode::FileSystemTemplateNotFound {
                ref raw_path, ref namespace, ref dirs
            } => {
                write!(f, " Unable to find template path {path:?} in namespace {id:?} (looked into: {dirs:?})",
                    path = raw_path,
                    id = namespace,
                    dirs = dirs)
            },
            LoaderErrorCode::FileSystemNamespaceNotInitialized {
                ref namespace
            } => {
                write!(f, " There are no registered directories for template namespace {}", namespace)
            },
            LoaderErrorCode::FileSystemMalformedNamespacedPath {
                ref template_name
            } => {
                write!(f, " {template} is malformed (expecting a namespaced template path like '@namespace/path_to_file').",
                    template = template_name)
            },
            LoaderErrorCode::FileSystemInvalidPath{
                ref path
            } => {
                write!(f, " Looks like you try to load a template outside configured directories ({path:?}).",
                    path = path)
            },
            LoaderErrorCode::FileSystemTemplateNotReadable{
                ref name, ref path
            } => {
                write!(f, " Missing read-permission for {path:?} while loading template {name:?}.",
                    path = path, name = name)
            },
        }
    }
}
