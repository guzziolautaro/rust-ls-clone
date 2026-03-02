use std::io::{self, Write};
use std::{env, fs, process};

fn main() {
    let mut flags: Vec<String> = Vec::new();
    let mut paths: Vec<String> = Vec::new();

    let mut show_hidden = false;

    for arg in env::args().skip(1) {
        if arg.starts_with('-') {
            flags.push(arg);
        } else {
            paths.push(arg);
        }
    }

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

    if paths.len() == 0 {
        paths.push("./".to_string());
    }

    for path in &paths {
        display_contents(path, &show_hidden);
    }
}

fn display_contents(path: &String, show_hidden: &bool) {
    let paths = fs::read_dir(path).unwrap();
    if *show_hidden {
        print!(". .. ")
    }

    for file in paths {
        let display_file = &file.unwrap().path().display().to_string()[path.len()..];

        if !*show_hidden && display_file.starts_with('.') {
            continue;
        } 
        print!("{} ", &display_file);
    }
    print!("\n");
    io::stdout().flush().unwrap();
}