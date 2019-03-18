# Rust CSS Minifier

A simple command line tool which minifies CSS.

This is a work in process and **is still very early**. Currently it only does
very basic minification (e.g. removing all spaces that don't break the code
and removing all line breaks).

However the app minifies CSS in O(n) time and is quite fast. Currently, I don't 
make guarantees about not breaking styles, but I have tested the app on large
files (like Materialize CSS) and it has worked so far.

## Usage
```css-minifier <input path> <outputpath>```

## Todo

* 
