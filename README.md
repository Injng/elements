# elements
A geometry markup language and diagram renderer.

## Development
There are two main parts to this project: the markup language and the diagram renderer. The functions that make up the markup
language are mostly defined in the `lang` folder, while the lexing and interpreting functions are defined in `lexer.rs` and
`interpreter.rs` in the main `src` folder respectively. These functions first convert a source string into a vector of `Token`s,
and then interpret these tokens into a set of `Value`s. These `Value`s then implement the `Element` trait, which will turn them
into `Svg` objects that hold the `Render` trait. The rendering system, located in the file `renderer.rs`, then takes these objects
and outputs the correct svg code.

- [ ] implement better labelling system

Note: main repository is developed using Mercurial, at [https://hg.sr.ht/~lnjng/elements](https://hg.sr.ht/~lnjng/elements).

## Usage
To install the program, simply use cargo:
```bash
cargo install elements-lang
```

To run the program, use the following command:
```bash
elements-lang <input file>
```

The program will then output the svg code to stdout as well as to a file called `out.svg`. To enable the labelling system, the
`--label` flag can be used.

Here is an example to render a triangle:
```lisp
(setq A (point 0 0))
(setq B (point 0 3))
(setq C (point 4 0))
(triangle A B C)
```

More examples can be found under the `examples` directory.

## Reference
The language is written in simple lisp syntax. Notably, however, you can simply write out the variable name with no parantheses,
and the interpreter will automatically substitute the value of the variable and render the appropriate object. For example, the
following code will render a triangle with vertices at (0, 0), (0, 3), and (4, 0):
```lisp
; this is a triangle
(setq A (point 0 0))
(setq B (point 0 3))
(setq C (point 4 0))
(setq T (triangle A B C))
T
```
Notice how the `setq` function is used to set variables, and how comments are started with a semicolon.

Functions are also often overloaded to provide more functionality with the same easy syntax. The following are the available
geometric functions:

### `point`
```lisp
(point [Int/Float] [Int/Float]) -> Point
```

The `point` function creates a point with the given x and y coordinates in the first and second parameters respectively.

### `circumcenter`
```lisp
(circumcenter [Triangle]) -> Point
```

The `circumcenter` function takes in a triangle and returns the circumcenter of that triangle.

### `orthocenter`
```lisp
(orthocenter [Triangle]) -> Point
```

The `orthocenter` function takes in a triangle and returns the orthocenter of that triangle.

### `centroid`
```lisp
(centroid [Triangle]) -> Point
```

The `centroid` function takes in a triangle and returns the centroid of that triangle.

### `inradius`
```lisp
(inradius [Triangle]) -> Int/Float
```

The `inradius` function takes in a triangle and returns the inradius of that triangle.

### `incenter`
```lisp
(incenter [Triangle]) -> Point
```

The `incenter` function takes in a triangle and returns the incenter of that triangle.

### `lineseg`
```lisp
(lineseg [Point] [Point]) -> Lineseg
```

The `lineseg` function creates a line segment with the given two points as the endpoints.

### `midpoint`
```lisp
(midpoint [Point] [Point]) -> Point
```

The `midpoint` function returns a point that is the midpoint of the two given points.

### `triangle`
```lisp
(triangle [Point] [Point] [Point]) -> Triangle
```

The first case for creating a triangle involves three parameters. The three parameters are the three vertices of the triangle.

```lisp
(triangle [Angle]) -> Triangle
```

The second case for creating a triangle is given an angle (i.e. two connected line segments), the function will create a triangle
with the angle as the vertex.

```lisp
(triangle [Circle]) -> Triangle
```

The third and ambiguous case, when given a circle, the function will return a randomly generated inscribed triangle. The triangle
will have points that are greater than half the radius apart.

### `circle`
```lisp
(circle [Point] [Int/Float]) -> Circle
```

The first case for creating a circle involves two parameters. The first is a parameter denoting the center of the circle. The second
parameter is a number denoting the radius of the circle.

```lisp
(circle) -> Circle
```

An ambiguous case for this function, when no parameters are given this function will create a standard circle at (0, 0) with radius 5.

### `angle`
```lisp
(angle [Point] [Point] [Point]) -> Angle
```

The `angle` function creates an angle from three points denoted in the three parameters.

### `iangle`
```lisp
(iangle [Circle] [Int/Float]) -> Angle
```

The `iangle` function creates an inscribed angle in a circle. The first parameter is the circle, and the second parameter is the
angle in degrees.

### `intersect`
```lisp
(intersect [Lineseg] [Circle] [Int]) -> Point
```

The first case for the `intersect` function involves three parameters. The first parameter is a line segment, the second is a circle,
and the third is an int representing either 0 or 1, the index of the point of intersection. As a line can maximally meet a circle at
two points, the index is used to determine which point to return.

```lisp
(intersect [Lineseg] [Lineseg]) -> Point
```

The second case for the `intersect` function involves two line segments. The function will return the point of intersection between
the two line segments.
