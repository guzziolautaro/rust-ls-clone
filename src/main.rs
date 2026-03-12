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
}

impl DirEntry {
    fn new(path: &str) -> DirEntry {
        let metadata = fs::metadata(path).expect("invalid path");

        let dir_entry = DirEntry {
            file_type: {
                if metadata.is_dir() {
                    FileType::Directory
                } else {
                    FileType::File
                }
            },
            name: match Path::new(path).file_name() {
                Some(s) => {
                    s.to_string_lossy().to_string()
                }
                None => String::new()
            },
            link_count: metadata.nlink(),
            owner: String::from("root"), // todo
            group: String::from("root"), // todo
            size: metadata.size()
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
        display_direntry( "./", show_hidden);
    } else {
        for path in &paths {
            print!("{}:\n", path);
            display_direntry(path, show_hidden);
        }
    }
}

fn get_entries() {
    // todo
}

fn display_direntry(path: &str, show_hidden: bool) {
    let entries = fs::read_dir(path).unwrap();
    if show_hidden {
        cprint!("<blue><bold>. .. ")
    }

    for entry in entries {
        // unrwap entry
        let entry = entry.unwrap();
        
        let file_name = entry.file_name();
        let display_file = file_name.to_string_lossy();
        let file_type = entry.file_type().unwrap();

        if !show_hidden && display_file.starts_with('.') {
            continue;
        }
        
        if file_type.is_dir() {
            cprint!("<blue><bold>{} ", &display_file)
        } else if file_type.is_symlink() {
            cprint!("<cyan><bold>{} ", &display_file)
        } else if file_type.is_file() {
            print!("{} ", &display_file);
        }
    }
    print!("\n");
    io::stdout().flush().unwrap();
}