// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Twig library for rust
///
/// @author Colin Kiegel <kiegel@gmx.de>


/////////////
// imports //
/////////////

extern crate twig;
use twig::{Engine, Setup};
use twig::loader;
use twig::runtime::Runtime;
use twig::template::api::Template;

/////////////
// exports //
/////////////

#[test]
fn hello_world_static() {
    let mut loader = loader::array::Array::default();
    let mut engine = Engine::new(Setup::default()).unwrap();
    let runtime = Runtime::default();

    loader.set_template("test","Hello world!");
    engine.set_loader(Box::new(loader));

    let compiled = engine.load_template("test", None).unwrap();
    println!("{:?}", compiled);

    assert_eq!(&compiled.render(&runtime).unwrap(), "Hello world!")
}

#[test]
fn hello_world_variable() {
    let mut loader = loader::array::Array::default();
    let mut engine = Engine::new(Setup::default()).unwrap();
    let mut runtime = Runtime::default();

    loader.set_template("test","Hello {{name}}!");
    engine.set_loader(Box::new(loader));
    runtime.set("name", "world");

    let compiled = engine.load_template("test", None).unwrap();
    println!("{:?}", compiled);

    assert_eq!(&compiled.render(&runtime).unwrap(), "Hello world!")
}

// #[test]
/// http://twig.sensiolabs.org/doc/tags/if.html
fn _if_elseif_else() {
    let mut loader = loader::array::Array::default();
    let mut engine = Engine::new(Setup::default()).unwrap();
    let mut runtime = Runtime::default();

    loader.set_template("test","\
        {% if A %}A is true\
        {% elseif B %}A is false and B is true\
        {% else %}A and B are false\
        {% endif %}");

    engine.set_loader(Box::new(loader));

    { // if A-branch
        runtime.set("A", "true");
        let compiled = engine.load_template("test", None).unwrap();
        println!("{:?}", compiled);
        assert_eq!(&compiled.render(&runtime).unwrap(), "A and B are false")
    }
    { // if B-branch
        runtime.clear_data();
        runtime.set("A", "false");
        runtime.set("B", "true");
        let compiled = engine.load_template("test", None).unwrap();
        println!("{:?}", compiled);
        assert_eq!(&compiled.render(&runtime).unwrap(), "A and B are false")
    }
    { // else-branch
        runtime.clear_data();
        let compiled = engine.load_template("test", None).unwrap();
        println!("{:?}", compiled);
        assert_eq!(&compiled.render(&runtime).unwrap(), "A and B are false")
    }

}
