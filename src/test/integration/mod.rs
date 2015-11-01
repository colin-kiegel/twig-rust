/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Twig library for rust
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

/////////////
// imports //
/////////////

use compiler::{Compiler, Builder};
use loader;
use runtime::Runtime;
use template::api::Template;

/////////////
// exports //
/////////////

#[test]
fn hello_world_static() {
    let mut loader = loader::array::Array::default();
    let mut compiler : Compiler = Builder::default().compiler().unwrap();
    let runtime = Runtime::default();

    loader.set_template("test","Hello world!");
    compiler.set_loader(Box::new(loader));

    let compiled = compiler.load_template("test", None).unwrap();
    println!("{:?}", compiled);

    assert_eq!(&compiled.render(&runtime).unwrap(), "Hello world!")
}

#[test]
fn hello_world_variable() {
    let mut loader = loader::array::Array::default();
    let mut compiler : Compiler = Builder::default().compiler().unwrap();
    let mut runtime = Runtime::default();

    loader.set_template("test","Hello {{name}}!");
    compiler.set_loader(Box::new(loader));
    runtime.set("name", "world");

    let compiled = compiler.load_template("test", None).unwrap();
    println!("{:?}", compiled);

    assert_eq!(&compiled.render(&runtime).unwrap(), "Hello world!")
}
