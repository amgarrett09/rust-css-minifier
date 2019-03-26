# Rust CSS Minifier

A simple command line tool which minifies CSS.

This is a work in process and **is still very early**. Currently it does the
following:

* Removes line breaks
* Removes spaces that don't affect syntax
* Removes semicolons before ending braces
* Removes comments

In the future I'd like to add more minifications.

The app minifies CSS in O(n) time and is quite fast. Although at this time 
I don't make guarantees about not breaking styles, I have tested the app on large
files (like Materialize CSS) and it has yet to break anything. There could be
some edge cases I haven't encountered though. If you decide
to use this minifier, please let me know if you encounter problems.

## Usage
```
css-minifier [FLAGS] [OPTIONS] <file paths>...
```

### For single files

To minify a single .css file, the app takes two arguments, like so:

```css-minifier <input path> <output path>```

### For multiple files

To minify multiple files at once, you need to set the -m flag, and also
specify an ouput folder with the -o option:

```css-minifier -m -o=<output folder> <input file paths>...```

The app will create minified .css files **with the same name as the input files,
inside the specified output folder**.

##Examples

### Minifying a single file

```
css-minifier ./example.css ./example-min.css
```

This will take example.css, and create a minified version in the same folder.

There's no requirement that it's in the same folder though, you can put the
output anywhere, e.g.:

```
css-minifier ./example.css ../public/css/example.css
```

This will take the imput file and create a minified version in the
`./public/css` folder.

### Minifying multiple files

```
css-minifier -m -o=./minified ./test.css ./example.css ./foo.css
```

This will take the three input files (test.css, example.css, and foo.css),
and create three minified files (also named test.css, example.css, and foo.css)
inside the `minified` folder.

You don't need the equals sign after the -o option. You could have written:

```
css-minifier -m -o ./minified ./test.css ./example.css ./foo.css
```

and the app would do the same thing. This is a matter of personal preference.