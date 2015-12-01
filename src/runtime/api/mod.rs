// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Node of compiled templates (forming an Abstract-Syntax-Tree)

use runtime::{Runtime, Job};
use std::fmt::Debug;

pub trait Execute : Debug {
    fn execute(&self, runtime: &Runtime, job: &mut Job);

    // NOTE: This function is *not* object safe, because
    //      runtime.run() requires a cast of self to object type `Execute`
    //      This limitation of rust seems awkward, because except for the
    //      type erasure this function would be object safe.
    //
    //      It feels like type erasure should be 'idempotent', i.e.
    //      type erasure on a trait object should just return the trait
    //      object. Thus type erasure should *not* require Self:Sized
    //
    // TODO: report to upstream rust
    fn run(&self, runtime: &Runtime) -> String where
        Self: Sized
    {
        runtime.run(self)
    }
}
