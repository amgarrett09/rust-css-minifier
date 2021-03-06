# Rust CSS Minifier

A simple command line tool which minifies CSS.

Currently it does the following:

* Removes line breaks
* Removes spaces that don't affect syntax
* Removes semicolons before ending braces
* Removes comments

In the future I'd like to add more minifications.

The app minifies CSS in O(n) time and is quite fast, so it can be
used to process large files, and also large numbers of files at once.

If decide to use this tool and you encounter any bugs, feel free to open an issue or pull request.

## Usage
```
css-minifier [FLAGS] [OPTIONS] [file paths]...
```

### For single files

To minify a single .css file, the app takes two arguments, like so:

```css-minifier <input path> <output path>```

### For multiple files

To minify multiple files at once, you need to set the -m flag, and also
specify an output folder with the -o option:

```css-minifier -m -o=<output folder> <input file paths>...```

The app will create minified .css files **with the same name as the input files,
inside the specified output folder**.

## Examples

### Minifying a single file

```
css-minifier example.css example-min.css
```

This will take example.css, and create a minified version in the same folder.

There's no requirement that it's in the same folder though, you can put the
output anywhere, e.g.:

```
css-minifier example.css ../public/css/example.css
```

This will take the input file and create a minified version in the
`public/css` folder that's a level above the current directory.

### Minifying multiple files

```
css-minifier -m -o=../public/css test.css example.css foo.css
```

This will take the three input files (test.css, example.css, and foo.css),
and create three minified files (also named test.css, example.css, and foo.css)
inside the `public/css` folder one level up.

You don't need the equals sign after the -o option. You could have written:

```
css-minifier -m -o ../public/css test.css example.css foo.css
```

and the app would do the same thing. This is a matter of personal preference.

### Reading files from standard in

When using the -m flag, you can pipe in a list of .css files to minify.

```
ls | awk '/.+\.css/' | css-minifier -m -o=minified
```

This lists all files in the current directory, then awk prints out
only the ones that are .css files, and finally this list of .css files is piped
to css-minifier. The app will minify the whole list, and put the minified
versions in the `minified` folder.