use std::io::{self, Write};
use std::{env, fs, process};

fn main() {
    let args = env::args().skip(1);

    // partioning args to flags and paths
    let (flags, paths): (Vec<String>, Vec<String>) = args.into_iter().partition(|a| a.starts_with('-'));

    let mut show_hidden = false;
    // flag identifying
    for flag in &flags {
        match flag.as_str() {
            "-a" => {
                show_hidden = true;
            }
            _ => {
                eprintln!("Invalid flag {}", flag);
                process::exit(1)
            }
        }
    }

    // defaulting to ./ if no path provided
    if paths.len() == 0 {
        display_contents( "./", show_hidden);
    } else {
        for path in &paths {
            print!("{}:\n", path);
            display_contents(path, show_hidden);
        }
    }
}

fn display_contents(path: &str, show_hidden: bool) {
    let paths = fs::read_dir(path).unwrap();
    if show_hidden {
        print!(". .. ")
    }

    for file in paths {
        let display_file = &file.unwrap().path().display().to_string()[path.len()..];

        if !show_hidden && display_file.starts_with('.') {
            continue;
        } 
        print!("{} ", &display_file);
    }
    print!("\n");
    io::stdout().flush().unwrap();
}