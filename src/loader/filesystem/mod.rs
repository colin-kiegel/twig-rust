// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Loads a template from the filesystem.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::os::unix::fs::MetadataExt;
use std::io::Read;
use std::borrow::Cow;
use super::{api, LoaderError, LoaderErrorCode};
use self::namespace::Namespace;
use error::ErrorCode;

pub mod namespace;
pub mod path;

#[derive(Default, Debug)]
pub struct Filesystem {
    namespaces: HashMap<String, Namespace>,
    path_cache: HashMap<String, PathBuf>,
}

impl api::Loader for Filesystem {
    fn source<'a>(&'a mut self, name: &str) -> Result<Cow<str>, LoaderError> {
        let path = try!(self.find_template(name));

        return match Self::read(&path) {
            Err(e) => {
                // TODO: self.unset_cached_path(name);

                // #NOTE:10 could add one final retry here, to support
                // seamless fallback to other template directories
                // - but should avoid infinite loops (one retry only)

                return Err(LoaderErrorCode::FileSystemTemplateNotReadable {
                        name: name.to_string(),
                        path: path.to_path_buf()
                    }.at(loc!())
                    .caused_by(e))
            },
            Ok(source) => Ok(Cow::Owned(source))
        }
    }

    fn cache_key<'a>(&'a mut self, name: &str) -> Result<Cow<'a, str>, LoaderError> {
        self.find_template(name).map(|x| Cow::Borrowed(x.to_str().unwrap())) // TODO: remove unwrap!
    }

    fn is_fresh(&mut self, name: &str, time: i64) -> bool {
        if let Ok(path) = self.find_template(name) {
            if let Ok(metadata) = fs::metadata(path) {
                return metadata.mtime() <= time
            }
        }
        false
    }
}

impl Filesystem {
    pub fn new() -> Filesystem {
        Default::default()
    }

    /// Returns all registered namespaces ids.
    pub fn namespace_ids(&self) -> Vec<&str> {
        self.namespaces.keys().map(|x| &x[..]).collect()
    }

    // /// Returns a namespace, if it exists.
    // fn namespace(&self, id: &str) -> Option<&Namespace> {
    //     self.namespaces.get(id)
    // }

    /// Returns a namespace, if it exists.
    fn mut_namespace(&mut self, id: &str) -> Option<&mut Namespace> {
        self.namespaces.get_mut(id)
    }

    /// Returns a namespace as mutable reference.
    ///
    /// *Creates* the namespace, if it does not exist.
    fn mut_namespace_or_new(&mut self, id: &str) -> &mut Namespace {
        if let Some(namespace) = self.namespaces.get_mut(id) {
            // #NOTE:30 working around rust limitation with conditionally escaping borrows
            //          * https://github.com/rust-lang/rust/issues/16481
            //          * https://github.com/rust-lang/rfcs/issues/811
            //       Another workaround would be the Entry API HashMap::entry()
            //       but that requires heap allocation of the key with every lookup.
            let sneak_out: *mut Namespace = namespace;
            return unsafe{ &mut *sneak_out } // equivalent to `return namespace`
            // <-- borrow ends here and is not extended
        }

        return self.namespaces.entry(id.to_string())
                    .or_insert(Namespace::new(id))
    }

    /// Sets the template directories for a given namespace.
    // #NOTE:20 taking a Vec<String> might be more efficient,
    //       but less flexible than IntoIterator<Item=String>
    pub fn set_dirs<D>(&mut self, dirs: D, namespace_id: Option<&str>)
        where D: IntoIterator<Item=PathBuf>
    {
        self.path_cache.clear();
        let namespace_id = namespace_id.unwrap_or(namespace::DEFAULT);
        let namespace = self.mut_namespace_or_new(namespace_id);

        namespace.set_dirs(dirs);
    }

    /// Appends a template directory for a given namespace.
    pub fn append_dir(&mut self, dir: PathBuf, namespace_id: Option<&str>) {
        // no need to clear the path cache
        let namespace_id = namespace_id.unwrap_or(namespace::DEFAULT);
        let namespace = self.mut_namespace_or_new(namespace_id);

        namespace.add_dir(dir);
    }

    /// Prepends a template directory for a given namespace.
    pub fn prepend_dir(&mut self, dir: PathBuf, namespace_id: Option<&str>) {
        self.path_cache.clear();
        let namespace_id = namespace_id.unwrap_or(namespace::DEFAULT);
        let namespace = self.mut_namespace_or_new(namespace_id);

        namespace.insert_dir(0, dir);
    }

    /// Find template.
    fn find_template(&mut self, template_path: &str) -> Result<&Path, LoaderError> {
        if let Some(cached) = self.path_cache.get(template_path) {
            // TODO: clear cache if file vanished - else return

            // #NOTE:50 working around rust limitation with conditionally escaping borrows
            //          * https://github.com/rust-lang/rust/issues/16481
            //          * https://github.com/rust-lang/rfcs/issues/811
            //       Another workaround would be the Entry API HashMap::entry()
            //       but that requires heap allocation of the key with every lookup.
            let sneak_out: *const PathBuf = cached;
            return Ok(unsafe{ &*sneak_out }) // equivalent to `return Ok(cached)`
            // <-- borrow ends here and is not extended
        }

        let path = try!(path::TemplatePath::parse(template_path));
        let namespace_id = path.namespace_id();
        let raw_path = path.raw_path();

        match self.namespaces.get_mut(namespace_id) {
            None => return err!(LoaderErrorCode::FileSystemNamespaceNotInitialized {
                    namespace: namespace_id.to_string()
            }),
            Some(namespace) => {
                try!(path.validate()); // #Doing:0 move these checks somewhere else :-)
                                       // e.g. postpone to reading the directoy
                let full_path = try!(namespace.find_template(raw_path));
                self.path_cache.insert(template_path.to_string(), full_path.to_path_buf());

                return Ok(full_path);
            }
        }
    }

    pub fn read(path: &Path) -> ::std::io::Result<String> {
        let mut file : File = try!(File::open(path));
        let mut buffer = String::new();

        try!(file.read_to_string(&mut buffer));

        return Ok(buffer)
    }

    /// Checks if the template path can be found or was previously cached.
    pub fn exists(&mut self, template_path: &str) -> bool {
        self.find_template(template_path).is_ok()
    }

    /// Clears the path cache for a given namespace.
    pub fn clear_path_cache(&mut self, namespace_id: Option<&str>) {
        let namespace_id = namespace_id.unwrap_or(namespace::DEFAULT);
        if let Some(namespace) = self.mut_namespace(namespace_id) {
            namespace.clear_path_cache();
        }

        self.path_cache.clear();
    }

    /// Removes the path from the path cache.
    pub fn unset_cached_path(&mut self, template_path: &str) {
        self.path_cache.remove(template_path);

        if let Ok(path) = path::TemplatePath::parse(template_path) {
            if let Some(namespace) = self.mut_namespace(path.namespace_id()) {
                namespace.unset_cached_path(path.raw_path());
            }
        }
    }
}
