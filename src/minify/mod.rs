use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use regex::Regex;

lazy_static! {
    static ref FILE_TEMPLATE: Regex = Regex::new(r"(.*\.css)").unwrap();
}

pub fn validate_filename(input: &str) -> bool {
    if let Some(caps) = FILE_TEMPLATE.captures(input) {
        if caps[1] == *input {
            return true;
        }
    };

    false
}

/* Takes a string slice and returns a minified String. Admittedly, not that
straightforward, but implemented this way so that parsing is done in O(n)
time.*/
pub fn minify_css(input: &str) -> String {
    // Special chars where a space is unnecessary after them:
    let special_chars: [char; 8] = ['{', '}', ':', ';', ' ', '\n', '!', '>'];
    let mut last_char: Vec<char> = " ".chars().collect();
    let mut output: Vec<char> = Vec::new();

    let mut comment = false;

    for ch in input.chars() {
        // We're in a comment if we find '/*'
        if !comment && ch == '*' && last_char[0] == '/' {
            comment = true;
            output.pop();
        }

        /* We should NOT add a char to the output if:
        1) It's a line break, OR
        2) The char is a space AND the last char scanned was one of our
        special cases OR
        3) We're inside a comment
        should_add_char is the negation of that */
        let should_add_char = !(ch == '\n'
            || (ch == ' ' && special_chars.contains(&last_char[0]))
            || comment);

        // We're no longer in a comment if we find '*/'
        if comment && ch == '/' && last_char[0] == '*' {
            comment = false;
        }

        if should_add_char {
            /* Remove last char (and don't put it back) if it's a space before
            a special character, or if it's a semicolon before an ending brace */
            if let Some(last) = output.pop() {
                if (!special_chars.contains(&ch) || last != ' ')
                    && (ch != '}' || last != ';')
                {
                    output.push(last);
                }
            }

            output.push(ch);
        }

        last_char[0] = ch;
    }

    output.iter().collect()
}

// Reads an input file, minifies the contents, and writes to an output file
pub fn create_minified_file(i_path: &Path, o_path: &Path) {
    let i_display = i_path.display();

    // Open file
    let mut file = match File::open(&i_path) {
        Err(reason) => {
            eprintln!("Couldn't open file {}: {}", i_display, reason.description());
            return;
        }
        Ok(file) => file,
    };

    // Read contents to string
    let mut content = String::new();
    if let Err(reason) = file.read_to_string(&mut content) {
        eprintln!(
            "Couldn't write to file {}: {}",
            i_display,
            reason.description()
        );
        return;
    }

    // Minify
    let minified = minify_css(&content);

    let o_display = o_path.display();

    // Create and write to output file
    let mut o_file = match File::create(&o_path) {
        Err(reason) => {
            eprintln!(
                "Couldn't create file {}: {}",
                o_display,
                reason.description()
            );
            return;
        }
        Ok(file) => file,
    };

    match o_file.write_all(minified.as_bytes()) {
        Err(reason) => {
            eprintln!(
                "Couldn't write to file {}: {}",
                o_display,
                reason.description()
            );
            return;
        }
        Ok(_) => println!("Successfully created file {}", o_display),
    };
}

#[cfg(test)]
mod tests {
    use super::minify_css;
    use super::validate_filename;

    #[test]
    fn validate_basics() {
        let input = "/home/test/test.css";
        assert!(validate_filename(input));

        let input = "/test/test.txt";
        assert!(!validate_filename(input));
    }

    #[test]
    fn minify_basics() {
        let input = " \n\n  p {\n    background-color: red;\n    \
                     color: blue;\n    flex: 1 0;\n}";
        let output = minify_css(input);

        assert_eq!(output, "p{background-color:red;color:blue;flex:1 0}");
    }

    #[test]
    fn minify_nested() {
        let input = "@media (min-height: 300px) {\n    test {\n        \
                     color: red;\n    }\n    }";
        let output = minify_css(input);

        assert_eq!(output, "@media (min-height:300px){test{color:red}}");
    }

    #[test]
    fn minify_comments() {
        let input = ".test {\n    background-color: red;\n    /* some comment */\n}";
        let output = minify_css(input);

        assert_eq!(output, ".test{background-color:red}");
    }

    #[test]
    fn minify_bang() {
        let input = ".hello {\n    background-color: red !important;\n}";
        let output = minify_css(input);

        assert_eq!(output, ".hello{background-color:red!important}")
    }

    #[test]
    fn minify_direct_child() {
        let input = ".hello > h1 {\n    color: green;\n}";
        let output = minify_css(input);

        assert_eq!(output, ".hello>h1{color:green}");
    }
}
