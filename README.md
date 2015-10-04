[![Build Status](https://travis-ci.org/colin-kiegel/twig-rust.svg)](https://travis-ci.org/colin-kiegel/twig-rust)

# Twig-Rust

a **template engine** for everyone writing web applications with Rust. By design Twig is
* flexible
* fast
* and secure

This library is derived from [Twig (for PHP)](http://twig.sensiolabs.org/documentation) and intended to become a _fully compatible_ port - as far as it makes sense.

## Current Status

> **EARLY ALPHA** - This library is still in development and not yet ready for use.

## License

Twig-Rust is released under the new BSD license (code and documentation) - as is the original Twig for PHP.

## Syntax and Semantics

Twig uses a syntax similar to the Django and Jinja template languages which inspired the Twig runtime environment.

```html
<!DOCTYPE html>
<html>
    <head>
        <title>Display a thread of posts</title>
    </head>
    <body>
    <h1>{{ thread.title }}</h1>
    <ul>
      {% for post in thread.posts %}
        <li>{{ post }}</li>
      {% endfor %}
    </ul>
    {# note: this comment will be ignored #}
    </body>
</html>
```

Take a look at this introduction: [Twig for template designers](http://twig.sensiolabs.org/doc/templates.html).

## General Architecture

Twig is designed to be highly extensible:
  * the Twig compiler only defines *general semantics* and a very flexible *extension mechanism*.
  * extensions define specific behavior and data transformations (like if-statement, for-loop, escape-filter, multiplication-operator, call-expression, etc.)
  * extensions are chosen at runtime.
  * if you don't like the default behavior (like if-statement, or call-expression) or if you are missing some functionality (like helpers for a new target like excel-files), all you need to do is replace, add or change extensions.

The **Lexer**
  * takes a Twig template and converts it into a token stream via pattern matching
  * the recognized semantics of Twig-Rust should be 100% compatible with Twig-PHP (static text, start+end of variable expression, start+end of generic block, identifier, operator, number and punctuation)
  * patterns are implemented as regular expressions (cached between multiple runs)
  * *operator* patterns can be defined via extensions - all other semantic patterns are generic enough to be independent of extensions
  * the lexer is implemented as a state machine with static dispatch for high performance

The **Parser**
  * converts a token stream into an abstract syntax tree
  * all nodes in the syntax tree are defined via extensions - even the most basic ones like static text. This allows for a very high degree of extensibility

**Extensions**
* define new behavior during the compilation process
  * **token parser**: transforms a sub-sequence from the token stream (=lexed template) to nodes in the abstract syntax tree. E.g. the `TokenParserIf` parses complex if-statements (if, elseif, else, endif) and creates the if-node with according child nodes for each test and conditional branch.
  * **node visitor**: modifies the abstract syntax tree immediately after parsing. E.g. the `optimizer` extension defines the `optimizeRawFilter` node visitor which strips all "raw" filters from the syntax tree.
* extensions define specific new *node types* in the abstract syntax tree - falling into the following generic classes. Note that all *examples* are defined in the `core` extension, if not stated otherwise.
  * **test**: can be used in conditional statements. E.g. the `defined` test checks if a variable is defined in the current context.
  * **unary operator**: can be used in variable expressions to process results. E.g. the `-` (neg) operator inverts the sign of a numeric result.
  * **binary operator**: can be used in variable expressions to combine two results. E.g. the `**` (power) operator takes one number to the power of another number.
  * **function**: can be used to perform complex computations. E.g. the `round` function rounds a floating number with a given precision.
  * **filter** can modify the result of variable expressions. E.g. the `default` filter returns the result of the variable expression if it is defined, otherwise it returns the default value. The `escaper` filter escapes the result according to the output channel (html, html attribute, css, js, url, ..)
  * **global**: can be used to define global constants. Templates can test for these global constants to trigger conditional behavior, or use them as argument for functions, etc.

**Unit-Tests**
* As this started as an educational project, every new feature comes with its own unit tests to prove that it is working.

## More Information

Read the [official documentation](http://twig.sensiolabs.org/documentation) for more information. Note that it refers to the PHP implementation, of course.
