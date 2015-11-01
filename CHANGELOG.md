# Planned

* `core` extension
  * test fileloader
  * template cache
  * more (unit) tests
  * {Mutstache}-Extension :-)
  * Pear Templates?
  * if conditional
  * for-each-loop + arrays
* runtime
  * split off jobs to separate data and writer
* TODOs + unimplemented!

* move template/raw/cursor to lexer/job/cursor

# Unreleased

## Added

* lexer: 100%
* extension API: 90%
* loader: 80%
* parser: 25%
* runtime: 20%

* load Templates from
 * array
 * filesystem
* parse/render
 * static text
 * variable expression

# 0.0.0 first commit (2015-06-16)

Rust aims to be a modern systems programming language with superb performance - ideally suited for low-level libraries.

Decision to start a Rust port of Twig-PHP 1.18.1

As a self-educational project I decided to learn Rust while doing something potentially useful (i.e. porting a well-designed library from PHP to Rust).
