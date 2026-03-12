use std::io::{self, Write};
use std::{env, process};

use color_print::cprint;

mod fs_models;

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
        let directory = fs_models::Directory::new("./");
        display_dir_entries(directory.entries, show_hidden);
    } else {
        for path in &paths {
            let directory = fs_models::Directory::new(path);
            print!("{}:\n", path);
            display_dir_entries(directory.entries, show_hidden);
        }
    }
}

fn display_dir_entries(entries: Vec<fs_models::DirEntry>, show_hidden: bool) {
    for entry in entries {
        if !show_hidden && entry.hidden { continue; }
        
        match entry.file_type {
            fs_models::FileType::File => cprint!("{} ", &entry.name),
            fs_models::FileType::ExecutableFile => cprint!("<green><bold>{} ", &entry.name),
            fs_models::FileType::CompressedFile => cprint!("<red><bold>{} ", &entry.name),
            fs_models::FileType::Directory => cprint!("<blue><bold>{} ", &entry.name),
            fs_models::FileType::SymLink => cprint!("<cyan><bold>{} ", &entry.name)
        }
    }
    print!("\n");
    io::stdout().flush().unwrap();
}