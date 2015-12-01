// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Namespaces for the filesystem template loader.

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs;
use loader::{LoaderError, LoaderErrorCode};

/// Identifier of the default namespace.
pub const DEFAULT : &'static str  = "__main__";

#[derive(Debug)]
pub struct Namespace {
    id: String,
    dirs: Vec<PathBuf>,
    path_cache: HashMap<PathBuf, PathBuf>,
}

impl Namespace {
    pub fn new<K>(key: K) -> Namespace
        where K: ToString
    {
        Namespace {
            id: key.to_string(),
            dirs: Default::default(),
            path_cache: Default::default(),
        }
    }

    /// Returns the template directories.
    pub fn dirs(&self) -> &Vec<PathBuf> {
        &self.dirs
    }

    /// Sets the template directories.
    pub fn set_dirs<D>(&mut self, dirs: D)
        where D: IntoIterator<Item=PathBuf>
    {
        self.path_cache.clear();
        self.dirs = dirs.into_iter().collect();
    }

    /// Adds a template directory.
    pub fn add_dir(&mut self, dir: PathBuf) {
        // path cache will not be poisoned!
        self.dirs.push(dir);
    }

    /// Insert the dir at a given position.
    ///
    /// Shifts all elements after position `index` one position to the right
    ///
    /// # Panics
    ///
    /// Panics if index is out of bounds.
    pub fn insert_dir(&mut self, index: usize, dir: PathBuf) {
        self.path_cache.clear();
        self.dirs.insert(index, dir);
    }

    pub fn find_template(&mut self, raw_path: &Path) -> Result<&Path, LoaderError> {
        if let Some(cached) = self.path_cache.get(raw_path) {
            // TODO: clear cache if file vanished - else return

            // #NOTE:40 working around rust limitation with conditionally escaping borrows
            //          * https://github.com/rust-lang/rust/issues/16481
            //          * https://github.com/rust-lang/rfcs/issues/811
            //       Another workaround would be the Entry API HashMap::entry()
            //       but that requires heap allocation of the key with every lookup.
            let sneak_out: *const PathBuf = cached;
            return Ok(unsafe{ &*sneak_out }) // equivalent to `return Ok(cached)`
            // <-- borrow ends here and is not extended
        }

        if self.dirs.len() == 0 {
            return err!(LoaderErrorCode::FileSystemNamespaceNotInitialized {
                namespace: self.id.to_string()
            })
        }

        for dir in self.dirs.iter() {
            let fullpath: PathBuf = dir.join(Path::new(raw_path));

            if let Ok(metadata) = fs::metadata(&fullpath) {
                if metadata.is_file() {
                    return Ok(self.path_cache.entry(PathBuf::from(raw_path))
                        .or_insert(fullpath))
                }
            }
        }

        return err!(LoaderErrorCode::FileSystemTemplateNotFound {
            raw_path: raw_path.to_path_buf(),
            namespace: self.id.to_string(),
            dirs: self.dirs.clone()
        })
    }

    pub fn clear_path_cache(&mut self) {
        self.path_cache.clear()
    }

    /// Removes the path from the path cache.
    pub fn unset_cached_path(&mut self, raw_path: &Path) {
        self.path_cache.remove(raw_path);
    }
}

impl Default for Namespace {
    fn default() -> Namespace {
        Namespace {
            id: DEFAULT.to_string(),
            dirs: Default::default(),
            path_cache: Default::default(),
        }
    }
}
