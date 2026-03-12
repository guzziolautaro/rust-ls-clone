use std::io::{self, Write};
use std::{env, fs, process};

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
        let entries = get_entries_from_dir("./");
        display_dir_entries(entries, show_hidden);
    } else {
        for path in &paths {
            let entries = get_entries_from_dir(path);
            print!("{}:\n", path);
            display_dir_entries(entries, show_hidden);
        }
    }
}

fn get_entries_from_dir(path: &str) -> Vec<fs_models::DirEntry> {
    let mut entries: Vec<fs_models::DirEntry> = Vec::new();

    for dir in fs::read_dir(path).expect("invalid path") {
        entries.push(fs_models::DirEntry::new(dir.unwrap().path().as_path()));
    } 

    entries
}

fn display_dir_entries(entries: Vec<fs_models::DirEntry>, show_hidden: bool) {
    for entry in entries {
        
        let file_name = entry.name;

        if !show_hidden && entry.hidden { continue; }
        
        match entry.file_type {
            fs_models::FileType::File => cprint!("{} ", &file_name),
            fs_models::FileType::ExecutableFile => cprint!("<green><bold>{} ", &file_name),
            fs_models::FileType::CompressedFile => cprint!("<red><bold>{} ", &file_name),
            fs_models::FileType::Directory => cprint!("<blue><bold>{} ", &file_name),
            fs_models::FileType::SymLink => cprint!("<cyan><bold>{} ", &file_name)
        }
    }
    print!("\n");
    io::stdout().flush().unwrap();
}