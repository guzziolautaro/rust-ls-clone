use std::io::{self, Write};
use std::{env, process};

use color_print::{cprint, cformat};

use crate::fs_models::FileType;

mod fs_models;

fn main() {
    let args = env::args().skip(1);

    // partioning args to flags and paths
    let (flags, paths): (Vec<String>, Vec<String>) =
        args.into_iter().partition(|a| a.starts_with('-'));

    // default to ./ if no path provided
    let paths = if paths.is_empty() {
        vec![String::from("./")]
    } else { paths };

    let mut show_hidden = false;
    let mut show_list = false;

    // flag identifying
    flags.iter().for_each(|flag| match flag.as_str() {
        "-a" => {
            show_hidden = true;
        }
        "-l" => {
            show_list = true;
        }
        "-lh" => {
            show_list = true;
        }
        _ => {
            eprintln!("Invalid flag {}", flag);
            process::exit(1)
        }
    });

    if paths.len() == 1 {
        let directory = fs_models::Directory::new(&paths[0]);
        if !show_list {
            display_dir_entries(directory.entries, show_hidden);
        } else {
            display_list_dir_entries(directory.entries, show_hidden);
        }
    } else {
        for path in &paths {
            let directory = fs_models::Directory::new(path);
            println!("{}:", path);
            if !show_list {
                display_dir_entries(directory.entries, show_hidden);
            } else {
                display_list_dir_entries(directory.entries, show_hidden);
            }
        }
    }
}

fn display_dir_entries(entries: Vec<fs_models::DirEntry>, show_hidden: bool) {
    entries
        .iter()
        .filter(|f| !(!show_hidden && f.hidden))
        .for_each(|f| match f.file_type {
            fs_models::FileType::File => cprint!("{} ", &f.name),
            fs_models::FileType::ExecutableFile => cprint!("<green><bold>{} ", &f.name),
            fs_models::FileType::CompressedFile => cprint!("<red><bold>{} ", &f.name),
            fs_models::FileType::Directory => cprint!("<blue><bold>{} ", &f.name),
            fs_models::FileType::SymLink => cprint!("<cyan><bold>{} ", &f.name),
        });
    print!("\n");
    io::stdout().flush().unwrap();
}

fn display_list_dir_entries(entries: Vec<fs_models::DirEntry>, show_hidden: bool) {
    entries
        .iter()
        .filter(|f| !(!show_hidden && f.hidden))
        .for_each(|f| {
            let display_file = format!("{dir}{permissions} {n_links} {user} {group} {size} {mod_time} {name} \n",
                dir = if f.file_type == FileType::Directory { "d" } else { "-" },
                permissions = f.permissions(),
                n_links = f.link_count,
                user = f.owner,
                group = f.group,
                size = f.size,
                mod_time = f.mod_time(),
                name = match f.file_type {
                    fs_models::FileType::File => cformat!("{} ", &f.name),
                    fs_models::FileType::ExecutableFile => cformat!("<green><bold>{} ", &f.name),
                    fs_models::FileType::CompressedFile => cformat!("<red><bold>{} ", &f.name),
                    fs_models::FileType::Directory => cformat!("<blue><bold>{} ", &f.name),
                    fs_models::FileType::SymLink => cformat!("<cyan><bold>{} ", &f.name),
                }
            );
            print!("{}", display_file);
        });
    print!("\n");
    io::stdout().flush().unwrap();
}