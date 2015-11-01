// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

 /// Paths for the filesystem template loader.

/////////////
// imports //
/////////////

use std::path::{Path, PathBuf, Component};
use super::namespace;
use loader::{LoaderError, LoaderErrorCode};

/////////////
// exports //
/////////////


pub struct TemplatePath {
    namespace: String,
    raw_path: PathBuf,
}

impl TemplatePath {
    // pub fn new(namespace: &str, raw_path: &str) -> Result<TemplatePath, TwigError> {
    //
    //
    //     template_path.normalize();
    //     try!(template_path.parse());
    //
    //     return template_path;
    // }
    pub fn parse(template_path: &str) -> Result<TemplatePath, LoaderError> {
        let normalized_path = TemplatePath::normalize(template_path);
        let namespace: &str;
        let raw_path: &str;

        if normalized_path.chars().nth(1) == Some('@') {
            match normalized_path.find('/') {
                None => return err!(LoaderErrorCode::InvalidPath)
                    .explain(format!("Malformed namespaced template path {template} (expecting '@namespace/path_to_file').",
                    template = normalized_path))
                    .into(),
                Some(pos) => {
                    namespace = &normalized_path[1..pos];
                    raw_path = &normalized_path[pos+1..];
                }
            }
        } else {
            namespace = namespace::DEFAULT;
            raw_path = &normalized_path;
        }

        Ok(TemplatePath {
            namespace: namespace.to_string(),
            raw_path: PathBuf::from(raw_path),
        })
    }

    pub fn raw_path(&self) -> &Path {
        &self.raw_path
    }

    pub fn namespace_id(&self) -> &str {
        &self.namespace
    }

    fn normalize(template_path: &str) -> String {
        // #TODO:740 these operations could be done by the path object?
        let mut normalized_path = String::with_capacity(template_path.len());
        let mut accepting_slash = true;

        for c in template_path.chars() {
            match c {
                '/' | '\\' => if accepting_slash {
                    normalized_path.push('/');
                    accepting_slash = false;
                },
                _ => {
                    normalized_path.push(c);
                    accepting_slash = true;
                }
            }
        }

        normalized_path
    }

    pub fn validate(&self) -> Result<(), LoaderError> {
        let mut level = 0;
        for component in self.raw_path.components() {
            match component {
                Component::ParentDir => level += 1,
                Component::Normal(_) => level += 1,
                Component::CurDir |
                Component::Prefix(_) |
                Component::RootDir => {}
            }
        }

        if level < 0 {
            return err!(LoaderErrorCode::InvalidPath)
                .explain(format!("Looks like you try to load a template outside configured directories ({template:?}).",
                template = self.raw_path))
                .into()
        }

        Ok(())
    }
}
