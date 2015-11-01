/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

 /// Loads a template from an array.
 ///
 /// When using this loader with a cache mechanism, you should know that a new cache
 /// key is generated each time a template content "changes" (the cache key being the
 /// source code of the template). If you don't want to see your cache grows out of
 /// control, you need to take care of clearing the old cache file by yourself.
 ///
 /// This loader should only be used for unit testing.

/////////////
// imports //
/////////////

use std::collections::HashMap;
use std::borrow::Cow;
use super::{api, LoaderError, LoaderErrorCode};

/////////////
// exports //
/////////////


#[derive(Default, Debug)]
pub struct Array {
    templates: HashMap<String,String>,
}

impl api::Loader for Array {
    fn source<'a>(&'a mut self, name: &str) -> Result<Cow<'a, str>, LoaderError> {

        return match self.templates.get(name) {
            None => {
                err!(LoaderErrorCode::TemplateNotFound)
                    .explain(format!("Entry not present {:?}.", name))
                    .into()
            }
            Some(x) => {
                Ok(Cow::Borrowed(x))
            }
        }
    }

    fn cache_key<'a>(&'a mut self, name: &str) -> Result<Cow<'a, str>, LoaderError> {
        self.source(name)
    }

    fn is_fresh(&mut self, name: &str, _time: i64) -> bool {
        self.exists(name)
    }
}

impl Array {
    pub fn new(templates: HashMap<String,String>) -> Array {
        Array {
            templates: templates,
        }
    }

    pub fn set_template<N, T>(&mut self, name: N, template: T) where
        N: ToString,
        T: ToString
    {
        self.templates.insert(name.to_string(), template.to_string());
    }

    pub fn exists(&self, name: &str) -> bool {
        self.templates.contains_key(name)
    }
}