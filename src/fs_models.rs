use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

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
    pub hidden: bool,
}

impl DirEntry {
    pub fn new(path: &Path) -> DirEntry {
        let metadata = path.metadata().expect("invalid_path_error");
        let entry_name = path
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("default_error")
            .to_string();

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
            hidden: if entry_name.starts_with(".") {
                true
            } else {
                false
            },
        };
        dir_entry
    }
}

pub struct Directory {
    pub entries: Vec<DirEntry>,
}

impl Directory {
    pub fn new(path: &str) -> Directory {
        let directory = Directory {
            entries: fs::read_dir(path)
                .expect("invalid_directory_path")
                .into_iter()
                .map(|f| DirEntry::new(f.unwrap().path().as_path()))
                .collect(),
        };

        directory
    }
}
