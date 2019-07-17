use std::path::Path;

mod minify;
mod argparse;

#[macro_use]
extern crate clap;
use clap::{App, Arg};

#[macro_use]
extern crate lazy_static;


fn main() {
    let matches = App::new("css-minifier")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(Arg::with_name("file paths").multiple(true))
        .arg(
            Arg::with_name("mult")
                .short("m")
                .long("multiple")
                .help("Minify multiple files at once")
                .requires("output folder"),
        )
        .arg(
            Arg::with_name("output folder")
                .short("o")
                .long("out")
                .help("Specifies the output folder to put minified files into")
                .takes_value(true)
                .requires("mult"),
        )
        .get_matches();

    // If the -m flag is set
    if matches.is_present("mult") {
        let inputs_paths: Vec<String> = argparse::parse_multiple_file_paths(&matches);

        let o_folder = matches.value_of("output folder").unwrap();

        for item in inputs_paths.iter() {
            if !minify::validate_filename(&item) {
                eprintln!("Inputs need to be .css files");
                return;
            }

            let i_path = Path::new(&item);
            let file_name = i_path.file_name().unwrap();
            let o_path = Path::new(&o_folder).join(file_name);

            // Read input file, minify contents, and write to new file
            minify::create_minified_file(&i_path, &o_path);
        }

        return;
    }

    // Default path from here on

    let file_paths: Vec<&str> = match matches.values_of("file paths") {
        Some(paths) => paths.collect(),
        None => {
            eprintln!(
                "error: File paths expected but not supplied.\n If \
                 trying to get file paths from standard in, this only works with \
                 the -m flag."
            );
            return;
        }
    };

    if file_paths.len() != 2 {
        eprintln!(
            "error: Invalid arguments. You must supply an input path and an \
             output path.\n If you need to minify multiple files, use the -m \
             flag.\n For help, use the -h flag."
        );
        return;
    }

    if !minify::validate_filename(file_paths[0]) || !minify::validate_filename(file_paths[1]) {
        eprintln!("Both input and output files must be .css files");
        return;
    }

    let i_path = Path::new(file_paths[0]);
    let o_path = Path::new(file_paths[1]);

    minify::create_minified_file(&i_path, &o_path);
}
