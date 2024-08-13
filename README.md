# elements
A geometry markup language and diagram renderer.

## Development
There are two main parts to this project: the markup language and the diagram renderer. The functions that make up the markup
language are mostly defined in the `lang` folder, while the lexing and interpreting functions are defined in `lexer.rs` and
`interpreter.rs` in the main `src` folder respectively. These functions first convert a source string into a vector of `Token`s,
and then interpret these tokens into a set of `Value`s. These `Value`s then implement the `Element` trait, which will turn them
into `Svg` objects that hold the `Render` trait. The rendering system, located in the file `renderer.rs`, then takes these objects
and outputs the correct svg code.

## Usage
Here is an example to render a triangle:
```lisp
(setq A (point 0 0))
(setq B (point 0 3))
(setq C (point 4 0))
(triangle A B C)
```

More examples can be found under the `examples` directory.

Note: main repository is developed using Mercurial, at [https://hg.sr.ht/~lnjng/elements](https://hg.sr.ht/~lnjng/elements).
