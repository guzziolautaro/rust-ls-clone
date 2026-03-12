use std::io::{self, Write};
use std::{env, fs, process, path::Path};
use std::os::unix::fs::MetadataExt;

use color_print::cprint;

enum FileType {
    File,
    ExecutableFile,
    CompressedFile,
    Directory,
    SymLink,
}

struct DirEntry {
    file_type: FileType,
    name: String,
    link_count: u64,
    owner: String,
    group: String,
    size: u64,
    hidden: bool
}

impl DirEntry {
    fn new(path: &Path) -> DirEntry {
        let metadata = path.metadata().unwrap();
        let entry_name = match path.file_name() {
            Some(s) => {
                s.to_string_lossy().to_string()
            }
            None => String::new()
        };

        let dir_entry = DirEntry {
            file_type: {
                if metadata.is_dir() {
                    FileType::Directory
                } else {
                    FileType::File
                }
            },
            name: String::from(&entry_name),
            link_count: metadata.nlink(),
            owner: String::from("root"), // todo
            group: String::from("root"), // todo
            size: metadata.size(),
            hidden: if entry_name.starts_with(".") { true } else { false }
        };
        dir_entry
    }
}

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

fn get_entries_from_dir(path: &str) -> Vec<DirEntry> {
    let mut entries: Vec<DirEntry> = Vec::new();

    for dir in fs::read_dir(path).expect("invalid path") {
        entries.push(DirEntry::new(dir.unwrap().path().as_path()));
    } 

    entries
}

fn display_dir_entries(entries: Vec<DirEntry>, show_hidden: bool) {
    for entry in entries {
        
        let file_name = entry.name;

        if !show_hidden && entry.hidden { continue; }
        
        match entry.file_type {
            FileType::File => cprint!("{} ", &file_name),
            FileType::ExecutableFile => cprint!("<green><bold>{} ", &file_name),
            FileType::CompressedFile => cprint!("<red><bold>{} ", &file_name),
            FileType::Directory => cprint!("<blue><bold>{} ", &file_name),
            FileType::SymLink => cprint!("<cyan><bold>{} ", &file_name)
        }
    }
    print!("\n");
    io::stdout().flush().unwrap();
}