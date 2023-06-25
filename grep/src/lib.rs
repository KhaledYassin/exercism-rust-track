use std::{fs, path::Path};

use anyhow::Error;

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// In the real world, it's common to use crates such as [`clap`] or
/// [`structopt`] to handle argument parsing, and of course doing so is
/// permitted in this exercise as well, though it may be somewhat overkill.
///
/// [`clap`]: https://crates.io/crates/clap
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
/// [`structopt`]: https://crates.io/crates/structopt

fn get_text_from_file(file_name: &str) -> Result<String, Error> {
    let file_path = Path::new(file_name);

    if !file_path.exists() {
        return Err(Error::msg(format!("File {} not found", file_name)));
    }

    if let Ok(text) = fs::read_to_string(file_path) {
        Ok(text)
    } else {
        Err(Error::msg(format!("Failed to read {}", file_name)))
    }
}

pub struct Flags<'a> {
    inner: &'a [&'a str],
}

impl<'a> Flags<'a> {
    pub fn new(flags: &'a [&'a str]) -> Self {
        Flags { inner: flags }
    }
}

pub fn grep(pat: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut output = vec![];

    let mark_file_names = files.len() > 1;
    let inner_flags = flags.inner;

    let print_line_numbers = inner_flags.contains(&"-n");
    let print_file_name = inner_flags.contains(&"-l");
    let match_case_insensitive = inner_flags.contains(&"-i");
    let match_inverted = inner_flags.contains(&"-v");
    let match_entire_line = inner_flags.contains(&"-x");

    for file in files {
        let text = get_text_from_file(file)?;

        output.extend(
            text.lines()
                .enumerate()
                .filter(|&(_, line)| {
                    let mut inner_line = line.to_string();

                    let mut inner_pattern = String::from(pat);

                    if match_case_insensitive {
                        inner_line = inner_line.to_lowercase();
                        inner_pattern = inner_pattern.to_lowercase();
                    }

                    if match_inverted {
                        !inner_line.contains(&inner_pattern)
                    } else if match_entire_line {
                        inner_line == inner_pattern
                    } else {
                        inner_line.contains(&inner_pattern)
                    }
                })
                .map(|(line_number, line)| {
                    let mut result = line.to_owned();

                    if print_line_numbers {
                        result.insert_str(0, &format!("{}:", line_number + 1));
                    }

                    if mark_file_names {
                        result.insert_str(0, &format!("{}:", file))
                    }

                    if print_file_name {
                        result = file.to_owned().to_owned();
                    }

                    result
                }),
        );
    }

    output.dedup_by(|a, b| (*a).eq(b));

    Ok(output)
}
