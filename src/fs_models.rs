use std::fs;
use std::path::Path;
use std::os::unix::fs::MetadataExt;

pub enum FileType {
    File,
    ExecutableFile,
    CompressedFile,
    Directory,
    SymLink,
}

pub struct DirEntry {
    pub file_type: FileType,
    pub name: String,
    pub link_count: u64,
    pub owner: String,
    pub group: String,
    pub size: u64,
    pub hidden: bool
}

impl DirEntry {
    pub fn new(path: &Path) -> DirEntry {
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

pub struct Directory {
    pub entries: Vec<DirEntry>
}

impl Directory {
    pub fn new(path: &str) -> Directory {
        let directory = Directory {
            entries: {
                let mut entries: Vec<DirEntry> = Vec::new();

                for dir in fs::read_dir(path).expect("invalid path") {
                    entries.push(DirEntry::new(dir.unwrap().path().as_path()));
                }
                entries
            }
        };

        directory
    }
}