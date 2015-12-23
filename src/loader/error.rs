// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Typisation of loader errors.

use std::fmt::{self, Display};
use std::error::Error;
use std::path::PathBuf;
use std::io;

#[derive(Debug)]
pub enum LoaderError {
    Unreachable {
        reason: String,
    },
    ArrayTemplateNotFound {
        name: String,
    },
    FileSystemTemplateNotFound {
        raw_path: PathBuf,
        namespace: String,
        dirs: Vec<PathBuf>,
    },
    FileSystemNamespaceNotInitialized {
        namespace: String,
    },
    FileSystemMalformedNamespacedPath {
        template_name: String,
    },
    FileSystemInvalidPath {
        path: PathBuf,
    },
    FileSystemTemplateNotReadable {
        name: String,
        path: PathBuf,
        io_err: io::Error,
    },
}

impl Error for LoaderError {
    fn description(&self) -> &str {
        match *self {
            LoaderError::Unreachable{..} => {
                "Unexptected template loader error (please report as bug with details)."
            }
            LoaderError::ArrayTemplateNotFound{..} |
            LoaderError::FileSystemTemplateNotFound{..} => "Template not found.",
            LoaderError::FileSystemNamespaceNotInitialized{..} => "Loader is not initialized.",
            LoaderError::FileSystemMalformedNamespacedPath{..} |
            LoaderError::FileSystemInvalidPath{..} => "Invalid template path.",
            LoaderError::FileSystemTemplateNotReadable{..} => "Could not read template file.",
        }
    }
}

impl Display for LoaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.description()));

        match *self {
            LoaderError::Unreachable {
                ref reason
            } => write!(f, " {}", reason),
            LoaderError::ArrayTemplateNotFound {
                ref name
            } => {
                write!(f,
                       " Template {:?} is not present in array template loader.",
                       name)
            }
            LoaderError::FileSystemTemplateNotFound {
                ref raw_path, ref namespace, ref dirs
            } => {
                write!(f,
                       " Unable to find template path {path:?} in namespace {id:?} (looked into: \
                        {dirs:?})",
                       path = raw_path,
                       id = namespace,
                       dirs = dirs)
            }
            LoaderError::FileSystemNamespaceNotInitialized {
                ref namespace
            } => {
                write!(f,
                       " There are no registered directories for template namespace {}",
                       namespace)
            }
            LoaderError::FileSystemMalformedNamespacedPath {
                ref template_name
            } => {
                write!(f,
                       " {template} is malformed (expecting a namespaced template path like \
                        '@namespace/path_to_file').",
                       template = template_name)
            }
            LoaderError::FileSystemInvalidPath{
                ref path
            } => {
                write!(f,
                       " Looks like you try to load a template outside configured directories \
                        ({path:?}).",
                       path = path)
            }
            LoaderError::FileSystemTemplateNotReadable{
                ref name, ref path, io_err: _
            } => {
                write!(f,
                       " Missing read-permission for {path:?} while loading template {name:?}.",
                       path = path,
                       name = name)
            }
        }
    }
}
