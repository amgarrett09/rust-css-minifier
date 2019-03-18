use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[macro_use]
extern crate clap;
use clap::{App, Arg};

#[macro_use]
extern crate lazy_static;

use regex::Regex;

lazy_static! {
    static ref FILE_TEMPLATE: Regex = Regex::new(r"(.*\.css)").unwrap();
}

fn main() {
    let matches = App::new("css-minifier")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(Arg::with_name("input path")
            .required(true)
        )
        .arg(Arg::with_name("output path")
            .required(true)
        )
        .get_matches();

    let i_path = matches.value_of("input path").unwrap();
    let o_path = matches.value_of("output path").unwrap();

    // Make sure user passed paths to css files
    if !validate_filename(&i_path) || !validate_filename(&o_path) {
        println!("ERROR: Both input and output need to be css files");
        return;
    }

    // Open input file and read from it
    let read_path = Path::new(&i_path);
    let display = read_path.display();

    let mut file = match File::open(&read_path) {
        Err(reason) => {
            println!(
                "ERROR: Couldn't open file {}: {}",
                display,
                reason.description()
            );
            return;
        }
        Ok(file) => file,
    };

    let mut contents = String::new();
    if let Err(reason) = file.read_to_string(&mut contents) {
        println!(
            "ERROR: Couldn't read contents of {}: {}",
            display,
            reason.description()
        );

        return;
    }

    // Minify
    let minified = minify_css(&contents);

    // Create output file and write to it
    let output_path = Path::new(&o_path);
    let output_display = output_path.display();

    let mut o_file = match File::create(&output_path) {
        Err(reason) => {
            println!(
                "Couldn't create file {}: {}",
                output_display,
                reason.description()
            );
            return;
        }
        Ok(file) => file,
    };

    match o_file.write_all(minified.as_bytes()) {
        Err(reason) => {
            println!(
                "Couldn't write to file {}: {}",
                output_display,
                reason.description()
            );
        }
        Ok(_) => println!("Successfully created {}", output_display),
    };
}


fn validate_filename(input: &str) -> bool {
    if let Some(caps) = FILE_TEMPLATE.captures(input) {
        if caps[1] == *input {
            return true;
        }
    };

    false
}

fn minify_css(input: &str) -> String {
    // Special chars where a space is unnecessary after them:
    let special_chars: Vec<char> = "{}:; \n".chars().collect();
    let mut last_char: Vec<char> = " ".chars().collect();
    let mut output: Vec<char> = Vec::new();

    for ch in input.chars() {
        // We shouldn't add a char to the output if:
        // 1) It's a line break, OR
        // 2) The char is a space AND the last char scanned was one of our special cases
        // should_add_char is the negation of that
        let should_add_char =
            !(ch == '\u{000a}' || (ch == '\u{0020}' && special_chars.contains(&last_char[0])));

        if should_add_char {
            // Remove spaces in front of special chars (done this way to deal with edge cases)
            if let Some(last) = output.pop() {
                if !(special_chars.contains(&ch) && last == '\u{0020}') {
                    output.push(last);
                }
            }

            output.push(ch);
        }

        last_char[0] = ch;
    }

    output.iter().collect()
}


#[cfg(test)]
mod tests {
    use super::minify_css;
    use super::validate_filename;

    #[test]
    fn validate_basics() {
        let input = "/home/test/test.css";
        assert!(validate_filename(input));

        let input="/test/test.txt";
        assert!(!validate_filename(input));
    }

    #[test]
    fn minify_basics() {
        let input = " \n\n  p {\n    background-color: red;\n    \
                     color: blue;\n    flex: 1 0;}";
        let output = minify_css(input);

        assert_eq!(output, "p{background-color:red;color:blue;flex:1 0;}");
    }

    #[test]
    fn minify_nested() {
        let input = "@media (min-height: 300px) {\n    test {\n        \
                     color: red;\n    }\n    }";
        let output = minify_css(input);

        assert_eq!(output, "@media (min-height:300px){test{color:red;}}");
    }
}