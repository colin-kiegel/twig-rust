// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Twig filecache for compiled templates.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[allow(dead_code)]
pub struct Cache {
    loaded_templates: HashMap<String, String>,
    filecache: Option<PathBuf>,
    _auto_reload: bool,
}

#[allow(dead_code)]
impl Cache {
    /// Clears the internal template cache.
    pub fn clear(&mut self) {
        self.loaded_templates.clear()
    }

    /// Loads a template by name.
    pub fn load(_name: &str) -> &str {
        unimplemented!()
    }

    /// Sets the cache directory or None if filecache is disabled.
    pub fn set_filecache(&mut self, filecache: Option<&Path>) {
        match filecache {
            None => self.filecache = None,
            Some(path) => {
                if path.file_name().is_some() {
                    panic!("path must be a directory")
                }

                self.filecache = Some(path.to_path_buf());
            }
        }
    }

    /// Gets the cache directory or None if filecache is disabled.
    pub fn filecache(&self) -> Option<&Path> {
        match self.filecache {
            None => None,
            Some(ref pathbuf) => Some(pathbuf.as_path()),
        }
    }

    /// Clears the template cache files on the filesystem.
    pub fn clear_filecache(&self) {
        if let Some(ref _path) = self.filecache {
            unimplemented!()
        }
    }

    /// Gets the cache filename for a given template.
    pub fn get_cache_filename(&self, _name: &str) -> String {
        unimplemented!()
    }
}
