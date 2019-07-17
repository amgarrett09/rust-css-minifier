use clap::ArgMatches;
use std::io::{self, BufRead};

pub fn parse_multiple_file_paths(matches: &ArgMatches) -> Vec<String>{
    let mut stdin_values: Vec<String> = Vec::new();
    let mut paths: Vec<String> = Vec::new();

    // Get file paths from args, or from stdin if no args
    if matches.is_present("file paths") {
        for val in matches.values_of("file paths").unwrap() {
            paths.push(val.to_string())
        }
    } else {
        let stdin = io::stdin();

        for line in stdin.lock().lines() {
            let line = line.expect("Couldn't read line from stdin");
            // Making sure data from line reads lives beyond this loop
            stdin_values.push(line);
        }

        for val in stdin_values.iter() {
            // Parse each line into params
            let params = val.split(' ');

            for param in params {
                match param {
                    "" => continue,
                    any => paths.push(any.to_string())
                }
            }
        }
    }

    paths
}
