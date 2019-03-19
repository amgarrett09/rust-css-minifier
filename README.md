# Rust CSS Minifier

A simple command line tool which minifies CSS.

This is a work in process and **is still very early**. Currently it does the
following:

* Removes line breaks
* Removes spaces that don't affect syntax
* Removes comments

In the future I'd like to add more minifications.

The app minifies CSS in O(n) time and is quite fast. Although at this time 
I don't make guarantees about not breaking styles, I have tested the app on large
files (like Materialize CSS) and it has yet to break anything. If you decide
to use this minifier, please let me know if you encounter problems.

## Usage
```css-minifier <input path> <output path>```
